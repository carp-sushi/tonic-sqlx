# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

GSDX is a Rust-based gRPC service for managing stories and tasks with PostgreSQL persistence. The service provides CRUD operations for stories and their associated tasks through a gRPC API.

## Development Commands

### Core Development Workflow
```bash
# Format code and proto files
make fmt

# Build the project
make build

# Run tests (unit tests)
make test

# Run integration tests (requires Docker)
make itest

# Lint code and proto files
make lint

# Full check (format, build, test, lint)
make all
```

### Database Operations
```bash
# Run database migrations
make migrate

# Start the server
make run

# Build release version
make release
```

### Development Tools
```bash
# Start gRPC UI for testing (requires grpcui)
make run-ui
```

## Architecture

### High-Level Structure
- **gRPC Service**: Core API layer using tonic framework
- **Domain Models**: Business logic types (Story, Task, Status)
- **Repository Layer**: Database abstraction using SQLx
- **Service Layer**: gRPC service implementation with validation
- **Configuration**: Environment-based configuration management

### Key Components

#### Proto Definition (`proto/gsdx/v1/gsdx.proto`)
Defines the gRPC service interface with 8 main operations:
- Story operations: ListStories, CreateStory, DeleteStory, UpdateStory
- Task operations: ListTasks, CreateTask, DeleteTask, UpdateTask

#### Domain Models (`src/domain/`)
- `Story`: Core story entity with UUID and name
- `Task`: Task entity with UUID, story_id, name, and status
- `Status`: Enum with `incomplete` (default) and `complete` states

#### Service Layer (`src/service/`)
- `Service`: Main gRPC service implementation
- `Validate`: Input validation helpers with bounds checking
- Error mapping from domain errors to gRPC status codes

#### Repository Layer (`src/repo/`)
- PostgreSQL-backed data access using SQLx
- Connection pooling with Arc<PgPool>
- Automatic migration support

### Error Handling
Custom error types mapped to gRPC status:
- `Error::NotFound` → `Status::not_found`
- `Error::InvalidArgs` → `Status::invalid_argument` 
- `Error::Internal` → `Status::internal`

### Configuration
Environment variables loaded via dotenvy:
- `DATABASE_URL`: PostgreSQL connection string (required)
- `DATABASE_SCHEMA`: Database schema (default: "public")
- `DATABASE_MAX_CONNECTIONS`: Pool size (default: num_cpus)
- `GRPC_SERVER_PORT`: gRPC listen port (default: "9090")
- `RUST_LOG`: Logging level (default: "info")
- `SQLX_OFFLINE`: Skip compile-time query validation (default: false)

## Testing Strategy

### Unit Tests
- Run with `cargo test` or `make test`
- Located inline with source files using `#[cfg(test)]`
- Example: Status enum validation in `src/domain/status.rs`

### Integration Tests
- Run with `cargo test -- --ignored` or `make itest`
- Use testcontainers for PostgreSQL setup
- Located in files with `#[ignore]` attribute
- Require Docker for database containers

### Development Database
Local PostgreSQL instance expected at:
```
postgres://postgres:password1@localhost:5432/gsdx
```

## Code Generation

### Protocol Buffers
- Proto files in `proto/gsdx/v1/`
- Code generation via `build.rs` using tonic-prost-build
- Generated code includes client and server stubs
- File descriptor sets for gRPC reflection

### Database Migrations
- Located in `migrations/` directory
- SQLx migration support with embedded migrations
- Auto-run on server startup

## Dependencies

### Core Runtime
- `tonic`: gRPC framework with compression support
- `sqlx`: Async PostgreSQL driver with migrations
- `tokio`: Async runtime with multi-thread support
- `chrono`: Date/time handling with serde support
- `uuid`: UUID generation and parsing

### Development Tools
- `testcontainers`: Docker-based integration testing
- `buf`: Protocol buffer tooling (format/lint)
- `grpcui`: gRPC web UI for manual testing