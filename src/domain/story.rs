use chrono::{DateTime, Utc};
use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
pub struct Story {
    pub id: Uuid,
    pub name: String,
    pub seqno: i64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
