#![forbid(unsafe_code)]
#![deny(
    clippy::exit,
    clippy::print_stderr,
    clippy::print_stdout,
    clippy::unwrap_used,
    clippy::wildcard_imports
)]

/// Protobuf definitions.
pub mod proto {
    tonic::include_proto!("gsdx.v1");
    pub const GSDX_V1_FILE_DESCRIPTOR_SET: &[u8] =
        tonic::include_file_descriptor_set!("gsdx_v1_descriptor");
}

/// Configuration from environment variables.
pub mod config;

/// The service layer.
pub mod service;

/// Domain models.
pub mod domain;

/// Side effects
pub mod effect;

/// Project errors.
pub mod error;

/// A light-weight abstraction over the database.
pub mod repo;

/// The GSDX transport server.
pub mod server;

/// gRPC implementation layer.
pub mod grpc;

/// Export error type
pub use error::Error;

/// Result type for the project.
pub type Result<T, E = Error> = std::result::Result<T, E>;
