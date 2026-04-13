use crate::{
    Result,
    domain::{Status, StoryId, Task, TaskId},
    effect::{TaskReader, TaskWriter},
    repo::Repo,
};
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

impl TaskReader for TaskService {
    async fn list(&self, story_id: StoryId) -> Result<Vec<Task>> {
        self.repo
            .fetch_story(&story_id)
            .and_then(|_| self.repo.list_tasks(&story_id))
            .await
    }
}

impl TaskWriter for TaskService {
    async fn create(&self, story_id: StoryId, name: String, status: Status) -> Result<Task> {
        self.repo.create_task(&story_id, name, status).await
    }

    async fn update(
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

    async fn delete(&self, task_id: TaskId) -> Result<()> {
        self.repo
            .fetch_task(&task_id)
            .and_then(|_| self.repo.delete_task(&task_id))
            .await
    }
}
