use sqlx::postgres::PgPool;
use std::sync::Arc;
use tokio::time::{self, Duration};
use tonic_health::{
    ServingStatus::{NotServing, Serving},
    server::HealthReporter,
};

/// Health check for the gRPC server. Makes sure the database is accessible.
pub async fn health_check(reporter: HealthReporter, db: Arc<PgPool>) {
    log::info!("Starting health check loop");
    loop {
        time::sleep(Duration::from_secs(2)).await;
        log::debug!("Running health check query");
        let query_fut = sqlx::query("SELECT 1").fetch_one(db.as_ref());
        let status = match query_fut.await {
            Ok(_) => Serving,
            Err(err) => {
                log::error!("Health check failed: {}", err);
                NotServing
            }
        };
        reporter.set_service_status("gsdx", status).await;
    }
}
