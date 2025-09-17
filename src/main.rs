#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

use gsdx::config::Config;

use clap::{Parser, Subcommand};
use dotenvy::dotenv;
use sqlx::migrate::Migrator;
use std::{error::Error, sync::Arc};

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
    // Load env vars and init global logger
    dotenv().ok();
    env_logger::init();

    // Parse command line arguments
    let cli = Cli::parse();

    // Load config
    let config = Config::load();
    log::debug!("Loaded config = {:?}", config);

    // Load connection pool
    let pool = config.db_pool_opts().connect(&config.db_url).await?;

    // Run schema migrations or start server based on the command line arguments provided.
    match cli.cmd {
        Cmd::Migrate => {
            log::info!("Running migrations");
            MIGRATOR.run(&pool).await?;
        }
        Cmd::Server => {
            gsdx::server::serve(Arc::new(pool), config.grpc_listen_addr).await?;
        }
    }

    Ok(())
}
