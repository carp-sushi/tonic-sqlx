use crate::{
    Result,
    domain::{Status, StoryId, Task, TaskId},
    repo::Repo,
    usecase::UseCase,
    usecase::task::{CreateTask, DeleteTask, ListTasks, UpdateTask},
};

use std::sync::Arc;

/// Task service
pub struct TaskService {
    delete_task: DeleteTask,
    list_tasks: ListTasks,
    create_task: CreateTask,
    update_task: UpdateTask,
}

impl TaskService {
    /// Constructor
    pub fn new(repo: Arc<Repo>) -> Self {
        Self {
            delete_task: DeleteTask::new(repo.clone()),
            list_tasks: ListTasks::new(repo.clone()),
            create_task: CreateTask::new(repo.clone()),
            update_task: UpdateTask::new(repo),
        }
    }

    /// Fetch all tasks for a story
    pub async fn list_tasks(&self, story_id: StoryId) -> Result<Vec<Task>> {
        self.list_tasks.execute(story_id).await
    }

    /// Create a new task
    pub async fn create_task(
        &self,
        story_id: StoryId,
        name: String,
        status: Status,
    ) -> Result<Task> {
        self.create_task
            .execute(CreateTask::args(story_id, name, status))
            .await
    }

    /// Update an existing task
    pub async fn update_task(
        &self,
        task_id: TaskId,
        name: Option<String>,
        status: Status,
    ) -> Result<Task> {
        self.update_task
            .execute(UpdateTask::args(task_id, name, status))
            .await
    }

    /// Delete an existing task.
    pub async fn delete_task(&self, task_id: TaskId) -> Result<()> {
        self.delete_task.execute(task_id).await
    }
}
