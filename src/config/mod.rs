use std::{env, net::SocketAddr};

/// Configuration settings
#[derive(Debug)]
pub struct Config {
    pub listen_addr: SocketAddr,
    pub db_max_connections: u32,
    pub db_url: String,
    pub db_schema: String,
}

mod db;

impl Config {
    /// Load config from env vars.
    pub fn load() -> Self {
        // http server settings
        let port = env::var("GRPC_SERVER_PORT").unwrap_or("9090".into());
        let listen_addr = format!("0.0.0.0:{port}")
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
            listen_addr,
            db_max_connections,
            db_url,
            db_schema,
        }
    }
}
