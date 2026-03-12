use crate::{
    Result,
    domain::{Status, StoryId, Task, TaskId},
};
use async_trait::async_trait;

/// Abstract type for stateful I/O effects that can be performed on tasks.
#[async_trait]
pub trait TaskEffects: Send + Sync {
    /// Create a new task
    async fn create(&self, story_id: StoryId, name: String, status: Status) -> Result<Task>;

    /// Fetch all tasks for a story
    async fn list(&self, story_id: StoryId) -> Result<Vec<Task>>;

    /// Update an existing task
    async fn update(&self, task_id: TaskId, name: Option<String>, status: Status) -> Result<Task>;

    /// Delete an existing task.
    async fn delete(&self, task_id: TaskId) -> Result<()>;
}
