use crate::Error;
use crate::domain::{Status, Story, StoryId, Task};
use crate::proto::{StoryData, TaskData, TaskStatus};

use chrono::{DateTime, Utc};
use prost_types::Timestamp;
use tonic::Status as GrpcStatus;

/// Map project errors to grpc status.
impl From<Error> for GrpcStatus {
    fn from(err: Error) -> Self {
        match err {
            Error::NotFound { message } => GrpcStatus::not_found(message),
            Error::InvalidArgs { messages } => GrpcStatus::invalid_argument(messages.join(",")),
            Error::Internal { message } => {
                log::error!("Internal error in service: {}", message);
                GrpcStatus::internal(message)
            }
        }
    }
}

/// Map domain story to gRPC response type
impl From<Story> for StoryData {
    fn from(story: Story) -> Self {
        let StoryId(story_id) = story.id;
        Self {
            story_id: story_id.to_string(),
            name: story.name,
            created_at: mk_prost_ts(story.created_at),
            updated_at: mk_prost_ts(story.updated_at),
        }
    }
}

/// Map domain status to gRPC task status
impl From<Status> for TaskStatus {
    fn from(status: Status) -> Self {
        if status == Status::Complete {
            TaskStatus::Complete
        } else {
            TaskStatus::Incomplete
        }
    }
}

/// Map gRPC task status to domain status
impl From<TaskStatus> for Status {
    fn from(status: TaskStatus) -> Self {
        if status == TaskStatus::Complete {
            Status::Complete
        } else {
            Status::Incomplete
        }
    }
}

/// Map domain task to gRPC response type
impl From<Task> for TaskData {
    fn from(task: Task) -> Self {
        let status = TaskStatus::from(task.status) as i32;
        Self {
            task_id: task.id.to_string(),
            story_id: task.story_id.to_string(),
            name: task.name,
            status,
            created_at: mk_prost_ts(task.created_at),
            updated_at: mk_prost_ts(task.updated_at),
        }
    }
}

/// Convert a domain timestamp to a gRPC timestamp.
pub fn mk_prost_ts(dt: DateTime<Utc>) -> Option<Timestamp> {
    Some(Timestamp {
        seconds: dt.timestamp(),
        nanos: dt.timestamp_subsec_nanos() as i32,
    })
}
