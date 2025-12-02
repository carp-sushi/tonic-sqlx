#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

use gsdx::config::Config;
use gsdx::server::Server;

use clap::{Parser, Subcommand};
use dotenvy::dotenv;
use sqlx::migrate::Migrator;
use std::{error::Error, sync::Arc};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

// Embed migrations into the GSDX binary.
pub static MIGRATOR: Migrator = sqlx::migrate!();

/// GSDX command line interface parser.
#[derive(Parser)]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    cmd: Option<Cmd>,
}

/// GSDX command line interface subcommands for running the server or migrations.
#[derive(Subcommand, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Default)]
enum Cmd {
    #[default]
    Server,
    Migrate,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Load environment variables from .env file.
    dotenv().ok();

    // Initialize structured JSON logging (level is set via RUST_LOG environment variable).
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::from_default_env())
        .with(tracing_subscriber::fmt::layer().json())
        .init();

    // Parse command line arguments
    let cli = Cli::parse();

    // Load config
    let config = Config::load();

    // Load connection pool
    let pool = config.db_pool_opts().connect(&config.db_url).await?;

    // Run schema migrations or start server based on the command line arguments provided.
    match cli.cmd.unwrap_or_default() {
        Cmd::Migrate => {
            tracing::info!("Running migrations");
            MIGRATOR.run(&pool).await?;
        }
        Cmd::Server => {
            let server = Server::new(Arc::new(pool));
            server.serve(config.listen_addr).await?;
        }
    }

    Ok(())
}
