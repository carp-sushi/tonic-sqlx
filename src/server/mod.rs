use crate::{
    grpc::Gsdx,
    proto::{GSDX_V1_FILE_DESCRIPTOR_SET, gsdx_service_server::GsdxServiceServer},
    repo::Repo,
    service::Service,
};

use sqlx::postgres::PgPool;
use std::{net::SocketAddr, sync::Arc};
use tonic::{codec::CompressionEncoding::Gzip, transport::Server as TransportServer};

mod health;
use health::health_check;

// The GSDX gRPC server
pub struct Server {
    pool: Arc<PgPool>,
}

impl Server {
    /// Create a new server
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }
}

impl Server {
    /// Start the GSDX gRPC server on the given socket address.
    pub async fn listen(
        &self,
        grpc_listen_addr: SocketAddr,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // Start health check
        let (reporter, health_service) = tonic_health::server::health_reporter();
        tokio::spawn(health_check(reporter, Arc::clone(&self.pool)));

        // Set up gRPC reflection
        let reflection_service = tonic_reflection::server::Builder::configure()
            .register_encoded_file_descriptor_set(GSDX_V1_FILE_DESCRIPTOR_SET)
            .build_v1()?;

        // Setup the GSDX service with gzip compression.
        let repo = Repo::new(Arc::clone(&self.pool));
        let ctx = Service::new(Arc::new(repo));
        let gsdx_service = GsdxServiceServer::new(Gsdx::new(ctx))
            .send_compressed(Gzip)
            .accept_compressed(Gzip);

        // Serve gRPC services
        log::info!("Server listening on {}", grpc_listen_addr);
        TransportServer::builder()
            .add_service(health_service)
            .add_service(reflection_service)
            .add_service(gsdx_service)
            .serve(grpc_listen_addr)
            .await?;

        Ok(())
    }
}
