use super::Repo;
use crate::{
    Error, Result,
    domain::{Status, StoryId, Task, TaskId},
};
use chrono::{DateTime, Utc};
use std::str::FromStr;
use uuid::Uuid;

// Put some reasonable upper limit when querying tasks for a story.
const MAX_TASKS: i64 = 100;

/// The task entity object - used for query validation against the database.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct TaskEntity {
    id: Uuid,
    story_id: Uuid,
    name: String,
    status: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

// The repo should map the entity to the domain object in public functions.
impl From<TaskEntity> for Task {
    fn from(te: TaskEntity) -> Self {
        Self {
            id: TaskId(te.id),
            story_id: StoryId(te.story_id),
            name: te.name,
            status: Status::from_str(&te.status).unwrap_or_default(),
            created_at: te.created_at,
            updated_at: te.updated_at,
        }
    }
}

// Extend repo with queries related to tasks.
impl Repo {
    /// Get a task by id
    pub async fn fetch_task(&self, TaskId(task_id): TaskId) -> Result<Task> {
        let query = sqlx::query_as!(
            TaskEntity,
            "SELECT id, story_id, name, status, created_at, updated_at FROM tasks WHERE id = $1",
            task_id,
        );
        match query.fetch_optional(self.db_ref()).await? {
            Some(task) => Ok(Task::from(task)),
            None => Err(Error::not_found(format!("task not found: {task_id}"))),
        }
    }

    /// Select tasks for a story
    pub async fn list_tasks(&self, StoryId(story_id): StoryId) -> Result<Vec<Task>> {
        let query = sqlx::query_as!(
            TaskEntity,
            r#"SELECT id, story_id, name, status, created_at, updated_at FROM tasks
            WHERE story_id = $1 ORDER BY created_at LIMIT $2"#,
            story_id,
            MAX_TASKS,
        );
        let tasks = query.fetch_all(self.db_ref()).await?;
        Ok(tasks.into_iter().map(Task::from).collect())
    }

    /// Insert a new task
    pub async fn create_task(
        &self,
        StoryId(story_id): StoryId,
        name: String,
        status: Status,
    ) -> Result<Task> {
        let query = sqlx::query_as!(
            TaskEntity,
            r#"INSERT INTO tasks (story_id, name, status) VALUES ($1, $2, $3)
            RETURNING id, story_id, name, status, created_at, updated_at"#,
            story_id,
            name,
            status.to_string(),
        );
        let task = query.fetch_one(self.db_ref()).await?;
        Ok(Task::from(task))
    }

    /// Update task name and status.
    pub async fn update_task(
        &self,
        TaskId(task_id): TaskId,
        name: String,
        status: Status,
    ) -> Result<Task> {
        let query = sqlx::query_as!(
            TaskEntity,
            r#"UPDATE tasks SET name = $1, status = $2 WHERE id = $3
            RETURNING id, story_id, name, status, created_at, updated_at"#,
            name,
            status.to_string(),
            task_id,
        );
        let task = query.fetch_one(self.db_ref()).await?;
        Ok(Task::from(task))
    }

    /// Delete a task.
    pub async fn delete_task(&self, TaskId(task_id): TaskId) -> Result<()> {
        sqlx::query!("DELETE FROM tasks WHERE id = $1", task_id)
            .execute(self.db_ref())
            .await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        domain::Status,
        repo::{Repo, tests},
    };
    use std::sync::Arc;

    use testcontainers::{ImageExt, runners::AsyncRunner};
    use testcontainers_modules::postgres::Postgres;

    #[ignore]
    #[tokio::test]
    async fn integration_test() {
        // Set up postgres test container backed repo
        let image = Postgres::default().with_tag("17-alpine");
        let container = image.start().await.unwrap();
        let pool = tests::setup_pg_pool(&container).await;
        let repo = Repo::new(Arc::clone(&pool));

        // Set up a story to put tasks under
        let name = "Books To Read".to_string();
        let story = repo.create_story(name.clone()).await.unwrap();
        let story_id = story.id;

        // Create task, ensuring status is incomplete
        let status = Status::Incomplete;
        let task = repo
            .create_task(story_id.clone(), "Suttree".to_string(), status)
            .await
            .unwrap();
        assert_eq!(task.status, Status::Incomplete);

        // Assert task exists
        assert!(repo.fetch_task(task.id.clone()).await.is_ok());

        // Set task status to complete
        repo.update_task(task.id.clone(), task.name, Status::Complete)
            .await
            .unwrap();

        // Fetch task and assert status was updated
        let task = repo.fetch_task(task.id.clone()).await.unwrap();
        assert_eq!(task.status, Status::Complete);

        // Query tasks for story.
        let tasks = repo.list_tasks(story_id.clone()).await.unwrap();
        assert_eq!(tasks.len(), 1);

        // Delete the task
        repo.delete_task(task.id.clone()).await.unwrap();

        // Assert task was deleted
        assert!(repo.fetch_task(task.id).await.is_err());

        // Cleanup
        repo.delete_story(story_id).await.unwrap();
    }
}
