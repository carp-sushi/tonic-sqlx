use crate::{
    Result,
    domain::{Status, StoryId, Task, TaskId},
    effect::TaskEffects,
    repo::Repo,
};
use async_trait::async_trait;
use futures_util::TryFutureExt;
use std::sync::Arc;

/// Task service
pub struct TaskService {
    repo: Arc<Repo>,
}

impl TaskService {
    /// Constructor
    pub fn new(repo: Arc<Repo>) -> Self {
        Self { repo }
    }
}

#[async_trait]
impl TaskEffects for TaskService {
    /// Fetch all tasks for a story
    async fn list_tasks(&self, story_id: StoryId) -> Result<Vec<Task>> {
        self.repo
            .fetch_story(&story_id)
            .and_then(|_| self.repo.list_tasks(&story_id))
            .await
    }

    /// Create a new task
    async fn create_task(&self, story_id: StoryId, name: String, status: Status) -> Result<Task> {
        self.repo.create_task(&story_id, name, status).await
    }

    /// Update an existing task
    async fn update_task(
        &self,
        task_id: TaskId,
        maybe_name: Option<String>,
        status: Status,
    ) -> Result<Task> {
        self.repo
            .fetch_task(&task_id)
            .and_then(|t| {
                let name = maybe_name.unwrap_or(t.name);
                self.repo.update_task(&task_id, name, status)
            })
            .await
    }

    /// Delete an existing task.
    async fn delete_task(&self, task_id: TaskId) -> Result<()> {
        self.repo
            .fetch_task(&task_id)
            .and_then(|_| self.repo.delete_task(&task_id))
            .await
    }
}
