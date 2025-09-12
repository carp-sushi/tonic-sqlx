use chrono::{DateTime, Utc};
use strum_macros::{Display, EnumString};
use uuid::Uuid;

/// The newtype story id.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StoryId(pub Uuid);

// Display the inner uuid.
impl std::fmt::Display for StoryId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// The story domain object.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Story {
    pub id: StoryId,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// The newtype task id.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TaskId(pub Uuid);

// Display the inner uuid.
impl std::fmt::Display for TaskId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// The task domain object.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Task {
    pub id: TaskId,
    pub story_id: StoryId,
    pub name: String,
    pub status: Status,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// The task status domain object.
#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, EnumString, Display)]
#[strum(serialize_all = "snake_case")]
pub enum Status {
    #[default]
    Incomplete,
    Complete,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn status_from_string() {
        let result = Status::from_str("incomplete").unwrap();
        assert_eq!(result, Status::Incomplete);
        let result = Status::from_str("complete").unwrap();
        assert_eq!(result, Status::Complete);
    }

    #[test]
    fn status_from_string_error() {
        let err = Status::from_str("xomplete").unwrap_err();
        assert_eq!(err.to_string(), "Matching variant not found");
    }

    #[test]
    fn status_to_string() {
        assert_eq!(Status::Complete.to_string(), "complete");
        assert_eq!(Status::Incomplete.to_string(), "incomplete");
    }
}
