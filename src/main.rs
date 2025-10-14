#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

use gsdx::config::Config;

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
    cmd: Cmd,
}

/// GSDX command line interface subcommands for running the server or migrations.
#[derive(Subcommand, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
enum Cmd {
    Migrate,
    Server,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Load env vars and init structured JSON logging.
    dotenv().ok();
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::from_default_env())
        .with(tracing_subscriber::fmt::layer().json())
        .init();

    // Parse command line arguments
    let cli = Cli::parse();

    // Load config
    let config = Config::load();
    //tracing::debug!("Loaded config = {:?}", config);

    // Load connection pool
    let pool = config.db_pool_opts().connect(&config.db_url).await?;

    // Run schema migrations or start server based on the command line arguments provided.
    match cli.cmd {
        Cmd::Migrate => {
            tracing::info!("Running migrations");
            MIGRATOR.run(&pool).await?;
        }
        Cmd::Server => {
            gsdx::server::serve(Arc::new(pool), config.listen_addr).await?;
        }
    }

    Ok(())
}
