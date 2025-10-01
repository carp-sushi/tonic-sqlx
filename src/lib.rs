#![forbid(unsafe_code)]
#![deny(
    clippy::exit,
    clippy::print_stderr,
    clippy::print_stdout,
    clippy::unwrap_used,
    clippy::wildcard_imports
)]

/// Export error type
pub use error::Error;

/// Result type for the project.
pub type Result<T, E = Error> = std::result::Result<T, E>;

/// Protobuf definitions.
pub mod proto {
    tonic::include_proto!("gsdx.v1");
    pub const GSDX_V1_FILE_DESCRIPTOR_SET: &[u8] =
        tonic::include_file_descriptor_set!("gsdx_v1_descriptor");
}

/// Configuration from environment variables.
pub mod config;

/// Domain objects.
pub mod domain;

/// Project errors.
pub mod error;

/// Health check.
pub mod health;

/// Request interceptor.
pub mod interceptor;

/// A light-weight abstraction over the database.
pub mod repo;

/// gRPC server layer.
pub mod server;

/// gRPC service layer.
pub mod service;

/// Utility (validation, paging) functions.
pub mod util;
