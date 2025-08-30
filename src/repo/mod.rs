use crate::Error;
use sqlx::postgres::PgPool;
use std::sync::Arc;

mod story;
mod task;

/// Database abstraction layer.
pub struct Repo {
    db: Arc<PgPool>,
}

impl Repo {
    /// Constructor
    pub fn new(db: Arc<PgPool>) -> Self {
        Self { db }
    }

    /// Get a ref to the connection pool.
    fn db_ref(&self) -> &PgPool {
        self.db.as_ref()
    }
}

impl From<sqlx::Error> for Error {
    fn from(err: sqlx::Error) -> Self {
        let errs = err.to_string();
        match err {
            sqlx::Error::ColumnNotFound(_) => Error::not_found(errs),
            sqlx::Error::InvalidArgument(errs) => Error::invalid_args(errs),
            sqlx::Error::RowNotFound => Error::not_found(errs),
            _ => Error::internal(errs),
        }
    }
}

#[cfg(test)]
mod tests {
    use sqlx::{
        migrate::Migrator,
        postgres::{PgPool, PgPoolOptions},
    };
    use std::{path::Path, sync::Arc};

    use testcontainers::ContainerAsync as Container;
    use testcontainers_modules::postgres::Postgres;

    /// Given a running Postgres container, set up a connection pool and run migrations.
    pub async fn setup_pg_pool(container: &Container<Postgres>) -> Arc<PgPool> {
        let connection_string = &format!(
            "postgres://postgres:postgres@localhost:{}/postgres",
            container.get_host_port_ipv4(5432).await.unwrap(),
        );

        let pool = PgPoolOptions::new()
            .max_connections(2)
            .min_connections(1)
            .connect(&connection_string)
            .await
            .unwrap();

        let m = Migrator::new(Path::new("./migrations")).await.unwrap();
        m.run(&pool).await.unwrap();

        Arc::new(pool)
    }
}
