use crate::{
    Result,
    domain::{Status, StoryId, Task, TaskId},
    effect::TaskEffects,
    repo::Repo,
    usecase::UseCase,
    usecase::task::{CreateTask, DeleteTask, ListTasks, UpdateTask},
};
use async_trait::async_trait;
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
        ListTasks::new(self.repo.clone()).execute(story_id).await
    }

    /// Create a new task
    async fn create_task(&self, story_id: StoryId, name: String, status: Status) -> Result<Task> {
        CreateTask::new(self.repo.clone())
            .execute(CreateTask::args(story_id, name, status))
            .await
    }

    /// Update an existing task
    async fn update_task(
        &self,
        task_id: TaskId,
        name: Option<String>,
        status: Status,
    ) -> Result<Task> {
        UpdateTask::new(self.repo.clone())
            .execute(UpdateTask::args(task_id, name, status))
            .await
    }

    /// Delete an existing task.
    async fn delete_task(&self, task_id: TaskId) -> Result<()> {
        DeleteTask::new(self.repo.clone()).execute(task_id).await
    }
}
