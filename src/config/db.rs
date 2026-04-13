use super::Config;
use sqlx::{Executor, postgres::PgPoolOptions};
use std::sync::Arc;

/// Validate that a schema name contains only safe characters (alphanumeric and underscores).
fn validate_schema(schema: &str) -> &str {
    assert!(
        !schema.is_empty()
            && schema
                .chars()
                .all(|c| c.is_ascii_alphanumeric() || c == '_'),
        "invalid database schema name: {schema}"
    );
    schema
}

impl Config {
    /// Create a new database connection pool options for postgres.
    pub fn db_pool_opts(&self) -> PgPoolOptions {
        let schema: Arc<str> = validate_schema(&self.db_schema).into();
        PgPoolOptions::new()
            .max_connections(self.db_max_connections)
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
