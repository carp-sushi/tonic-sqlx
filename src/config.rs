use sqlx::{Executor, postgres::PgPoolOptions};
use std::{env, net::SocketAddr, sync::Arc, time::Duration};

/// Configuration settings
#[derive(Debug)]
pub struct Config {
    pub grpc_listen_addr: SocketAddr,
    pub db_max_connections: u32,
    pub db_url: String,
    pub db_schema: String,
}

impl Config {
    /// Load config from env vars.
    pub fn load() -> Self {
        // http server settings
        let port = env::var("GRPC_SERVER_PORT").unwrap_or("9090".into());
        let grpc_listen_addr = format!("0.0.0.0:{port}")
            .parse()
            .expect("grpc_listen_addr could not be parsed");

        // database settings
        let mut db_max_connections = num_cpus::get() as u32;
        if let Ok(s) = env::var("DATABASE_MAX_CONNECTIONS") {
            db_max_connections = s
                .parse()
                .expect("DATABASE_MAX_CONNECTIONS could not be parsed")
        }
        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL not set");
        let db_schema = env::var("DATABASE_SCHEMA").unwrap_or("public".to_string());

        // Create config
        Self {
            grpc_listen_addr,
            db_max_connections,
            db_url,
            db_schema,
        }
    }

    /// Create a new database connection pool options for postgres.
    pub fn db_pool_opts(&self) -> PgPoolOptions {
        let schema = Arc::new(self.db_schema.clone());
        PgPoolOptions::new()
            .max_connections(self.db_max_connections)
            .acquire_timeout(Duration::from_secs(10))
            .after_connect(move |conn, _meta| {
                let schema = Arc::clone(&schema);
                Box::pin(async move {
                    conn.execute(format!("SET search_path = '{schema}';").as_ref())
                        .await?;
                    Ok(())
                })
            })
    }
}
