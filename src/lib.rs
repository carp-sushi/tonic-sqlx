/// Protobuf definitions.
pub mod proto {
    tonic::include_proto!("gsdx.v1");
    pub const GSDX_V1_FILE_DESCRIPTOR_SET: &[u8] =
        tonic::include_file_descriptor_set!("gsdx_v1_descriptor");
}

/// Configuration from environment variables.
pub mod config;

/// Domain models and logic.
pub mod domain;

/// Project errors.
pub mod error;

/// A light-weight abstraction over the database.
pub mod repo;

/// gRPC service layer.
pub mod service;

/// Export error type
pub use error::Error;

/// Result type for the project.
pub type Result<T, E = Error> = std::result::Result<T, E>;
