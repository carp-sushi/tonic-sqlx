# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

GSDX is a Rust gRPC service for managing stories and tasks with PostgreSQL persistence, built with tonic and sqlx.

## Development Commands

```bash
make all          # Full check: format, build, test, lint
make fmt          # Format code (cargo fmt) and proto files (buf format)
make build        # Build the project
make test         # Run unit tests
make itest        # Run integration tests (requires Docker)
make lint         # Lint code (clippy) and proto files (buf lint)
make run          # Start the server (cargo run -- server)
make migrate      # Run database migrations (cargo run -- migrate)
```

Run a single test: `cargo test <test_name>`
Run a single integration test: `cargo test <test_name> -- --ignored`

## Architecture

The codebase follows a layered architecture with trait-based abstraction between the gRPC transport and business logic:

```
gRPC (src/grpc/)  →  Effect traits (src/effect/)  ←  Service (src/service/)  →  Repo (src/repo/)
```

### Layer Responsibilities

- **`grpc/`**: Implements `GsdxService` (tonic). Handles request validation (`validate.rs`), type conversion between proto and domain types (`adapter.rs`). Depends on effect traits, not concrete services.
- **`effect/`**: Async trait definitions (`StoryEffects`, `TaskEffects`) that abstract side effects. This is the boundary between the gRPC layer and business logic — the gRPC layer holds `Arc<Box<dyn StoryEffects>>` and `Arc<Box<dyn TaskEffects>>`.
- **`service/`**: `StoryService` and `TaskService` implement the effect traits. Contains business logic (e.g., skip update if name unchanged, verify entity exists before delete/update).
- **`repo/`**: `Repo` struct with `sqlx::query_as!` macros against PostgreSQL. Uses private `*Entity` structs for DB mapping, converts to domain types in public methods. Story deletion cascades to tasks via transaction.
- **`domain/`**: Pure data types (`Story`, `Task`, `Status`) with newtype wrappers for IDs (`StoryId`, `TaskId`). No business logic beyond `Status` serialization via strum.
- **`server/`**: Wires everything together — creates `Repo`, services, `Gsdx`, and starts the tonic transport server with health checks, reflection, and gzip compression.

### Key Patterns

- **Newtype IDs**: `StoryId(Uuid)` and `TaskId(Uuid)` prevent mixing up ID types.
- **Compile-time query checking**: `sqlx::query_as!` validates SQL at compile time. Set `SQLX_OFFLINE=true` to skip when no DB is available.
- **Cursor-based pagination**: Stories use `seqno`-based cursors with clamped page bounds (10–100).
- **Clippy strictness**: `lib.rs` forbids unsafe code and denies `unwrap_used`, `print_stdout/stderr`, `exit`, and `wildcard_imports`.

### Error Handling

`Error` enum (thiserror) maps to gRPC status: `NotFound` → `not_found`, `InvalidArgs` → `invalid_argument`, `Internal` → `internal`. The `From<Error> for GrpcStatus` impl lives in `grpc/adapter.rs`. The `From<sqlx::Error> for Error` impl lives in `repo/mod.rs`.

## Testing

- **Unit tests**: Inline `#[cfg(test)]` modules (e.g., `grpc/validate.rs`, `domain/status.rs`)
- **Integration tests**: Marked `#[ignore]`, use `testcontainers` with Postgres 17-alpine. Shared setup helper at `repo/mod.rs::tests::setup_pg_pool`.

## Configuration

Environment variables (loaded via dotenvy from `.env`):
- `DATABASE_URL` (required): e.g. `postgres://postgres:password1@localhost:5432/gsdx`
- `DATABASE_SCHEMA`: default `"public"`
- `DATABASE_MAX_CONNECTIONS`: default `num_cpus`
- `GRPC_SERVER_PORT`: default `"9090"`
- `RUST_LOG`: default `"info"`
- `SQLX_OFFLINE`: set `true` to skip compile-time query validation

## Code Generation

Proto files in `proto/gsdx/v1/` are compiled by `build.rs` using tonic-prost-build. Generated code is included via `tonic::include_proto!("gsdx.v1")` in `lib.rs`. Database migrations are in `migrations/` and auto-embedded into the binary via `sqlx::migrate!()`.
