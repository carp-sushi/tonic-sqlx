#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

use gsdx::{
    config::Config,
    proto::{GSDX_V1_FILE_DESCRIPTOR_SET, gsdx_service_server::GsdxServiceServer},
    repo::Repo,
    service::{Service, health::health_check},
};

use dotenvy::dotenv;
use sqlx::migrate::Migrator;
use std::{error::Error, sync::Arc};
use tonic::{codec::CompressionEncoding::Gzip, transport::Server};

// Embed migrations into the gRPC server binary.
pub static MIGRATOR: Migrator = sqlx::migrate!();

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Load env vars and init global logger
    dotenv().ok();
    env_logger::init();

    // Load config
    let config = Config::load();
    log::debug!("Loaded config = {:?}", config);

    // Load connection pool and run schema migrations
    let pool = config.db_pool_opts().connect(&config.db_url).await?;
    log::info!("Running migrations");
    MIGRATOR.run(&pool).await?;

    // Arc up connection pool for async sharing across tasks
    let pool = Arc::new(pool);

    // Start health check
    let (reporter, health_service) = tonic_health::server::health_reporter();
    tokio::spawn(health_check(reporter, Arc::clone(&pool)));

    // Set up gRPC reflection
    let reflection_service = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(GSDX_V1_FILE_DESCRIPTOR_SET)
        .build_v1()?;

    // Setup the GSDX service with gzip compression.
    let repo = Repo::new(Arc::clone(&pool));
    let gsdx_service = GsdxServiceServer::new(Service::new(repo))
        .send_compressed(Gzip)
        .accept_compressed(Gzip);

    // Serve gRPC services
    log::info!("Server listening on {}", config.grpc_listen_addr);
    Server::builder()
        .add_service(health_service)
        .add_service(reflection_service)
        .add_service(gsdx_service)
        .serve(config.grpc_listen_addr)
        .await?;

    Ok(())
}
