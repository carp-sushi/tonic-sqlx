use crate::{
    health::health_check,
    interceptor::RequestInterceptor,
    proto::{GSDX_V1_FILE_DESCRIPTOR_SET, gsdx_service_server::GsdxServiceServer},
    repo::Repo,
    service::Service,
};

use sqlx::postgres::PgPool;
use std::{error::Error, net::SocketAddr, sync::Arc};
use tonic::{
    codec::CompressionEncoding::Gzip, service::interceptor::InterceptedService,
    transport::Server as TransportServer,
};

/// GSDX gRPC server.
pub struct Server {
    pool: Arc<PgPool>,
}

impl Server {
    /// Create a new GSDX gRPC server instance.
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }
}

impl Server {
    /// Start the GSDX gRPC server.
    pub async fn serve(&self, listen_addr: SocketAddr) -> Result<(), Box<dyn Error>> {
        // Start health check
        let (reporter, health_service) = tonic_health::server::health_reporter();
        tokio::spawn(health_check(reporter, Arc::clone(&self.pool)));

        // Set up gRPC reflection
        let reflection_service = tonic_reflection::server::Builder::configure()
            .register_encoded_file_descriptor_set(GSDX_V1_FILE_DESCRIPTOR_SET)
            .build_v1()?;

        // Setup the GSDX service with gzip compression.
        let repo = Repo::new(Arc::clone(&self.pool));
        let gsdx_service_server = GsdxServiceServer::new(Service::new(repo))
            .send_compressed(Gzip)
            .accept_compressed(Gzip);

        // Wrap server with request interceptor.
        let gsdx_service = InterceptedService::new(gsdx_service_server, RequestInterceptor::new());

        // Serve gRPC services
        tracing::info!("Server listening on {}", listen_addr);
        TransportServer::builder()
            .add_service(health_service)
            .add_service(reflection_service)
            .add_service(gsdx_service)
            .serve(listen_addr)
            .await?;

        Ok(())
    }
}
