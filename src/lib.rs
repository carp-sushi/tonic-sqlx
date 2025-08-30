pub mod proto {
    tonic::include_proto!("gsdx.v1");
    pub const GSDX_V1_FILE_DESCRIPTOR_SET: &[u8] =
        tonic::include_file_descriptor_set!("gsdx_v1_descriptor");
}
pub mod config;
pub mod domain;
pub mod error;
pub mod repo;
pub mod service;

pub use error::Error;
pub type Result<T, E = Error> = std::result::Result<T, E>;
