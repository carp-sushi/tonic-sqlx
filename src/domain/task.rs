use super::Status;
use chrono::{DateTime, Utc};
use std::str::FromStr;
use uuid::Uuid;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Task {
    pub id: Uuid,
    pub story_id: Uuid,
    pub name: String,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Task {
    pub fn status(&self) -> Status {
        Status::from_str(&self.status).unwrap_or_default()
    }
}
