use crate::{
    context::Context,
    health::health_check,
    proto::{GSDX_V1_FILE_DESCRIPTOR_SET, gsdx_service_server::GsdxServiceServer},
    repo::Repo,
    service::Service,
};

use sqlx::postgres::PgPool;
use std::{net::SocketAddr, sync::Arc};
use tonic::{codec::CompressionEncoding::Gzip, transport::Server};

/// Start the gRPC server.
pub async fn serve(
    pool: Arc<PgPool>,
    grpc_listen_addr: SocketAddr,
) -> Result<(), Box<dyn std::error::Error>> {
    // Start health check
    let (reporter, health_service) = tonic_health::server::health_reporter();
    tokio::spawn(health_check(reporter, Arc::clone(&pool)));

    // Set up gRPC reflection
    let reflection_service = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(GSDX_V1_FILE_DESCRIPTOR_SET)
        .build_v1()?;

    // Setup the GSDX service with gzip compression.
    let repo = Repo::new(Arc::clone(&pool));
    let ctx = Context::new(Arc::new(repo));
    let gsdx_service = GsdxServiceServer::new(Service::new(ctx))
        .send_compressed(Gzip)
        .accept_compressed(Gzip);

    // Serve gRPC services
    log::info!("Server listening on {}", grpc_listen_addr);
    Server::builder()
        .add_service(health_service)
        .add_service(reflection_service)
        .add_service(gsdx_service)
        .serve(grpc_listen_addr)
        .await?;

    Ok(())
}
