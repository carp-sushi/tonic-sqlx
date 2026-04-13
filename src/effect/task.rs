use crate::{
    Result,
    domain::{Status, StoryId, Task, TaskId},
};
use std::future::Future;

/// Abstract type for read-only effects that can be performed on tasks.
pub trait TaskReader: Send + Sync {
    /// Fetch all tasks for a story
    fn list(&self, story_id: StoryId) -> impl Future<Output = Result<Vec<Task>>> + Send;
}

/// Abstract type for write effects that can be performed on tasks.
pub trait TaskWriter: Send + Sync {
    /// Create a new task
    fn create(
        &self,
        story_id: StoryId,
        name: String,
        status: Status,
    ) -> impl Future<Output = Result<Task>> + Send;

    /// Update an existing task
    fn update(
        &self,
        task_id: TaskId,
        name: Option<String>,
        status: Status,
    ) -> impl Future<Output = Result<Task>> + Send;

    /// Delete an existing task.
    fn delete(&self, task_id: TaskId) -> impl Future<Output = Result<()>> + Send;
}
