use crate::{
    Error,
    domain::{Story, Task},
    proto::gsdx_service_server::GsdxService,
    proto::*,
    repo::Repo,
};

use futures_util::TryFutureExt;
use tonic::{Request, Response, Status};
use validate::Validate;

pub mod health;
mod validate;

/// GSDX gRPC service.
pub struct Service {
    repo: Repo,
}

impl Service {
    /// Constructor
    pub fn new(repo: Repo) -> Self {
        Self { repo }
    }
}

/// Map project errors to grpc status.
impl From<Error> for Status {
    fn from(err: Error) -> Self {
        match err {
            Error::NotFound { message } => Status::not_found(message),
            Error::InvalidArgs { messages } => Status::invalid_argument(messages.join(",")),
            Error::Internal { message } => {
                log::error!("Internal error in service: {}", message);
                Status::internal(message)
            }
        }
    }
}

/// Map domain story to gRPC response type
impl From<Story> for StoryData {
    fn from(story: Story) -> Self {
        Self {
            story_id: story.id.to_string(),
            name: story.name,
        }
    }
}

/// Map domain task to gRPC response type
impl From<Task> for TaskData {
    fn from(task: Task) -> Self {
        Self {
            task_id: task.id.to_string(),
            story_id: task.story_id.to_string(),
            name: task.name,
            status: task.status.to_string(),
        }
    }
}

#[tonic::async_trait]
impl GsdxService for Service {
    /// Create a new story.
    async fn create_story(
        &self,
        request: Request<CreateStoryRequest>,
    ) -> Result<Response<CreateStoryResponse>, Status> {
        log::debug!("Create story");
        let request = request.get_ref(); // Upack request

        // Validate
        let name = Validate::string_length(&request.name, "name")?;

        // Action
        let story = self.repo.create_story(name).await?;

        // Respond
        Ok(Response::new(CreateStoryResponse {
            story: Some(story.into()),
        }))
    }

    /// Delete an existing story.
    async fn delete_story(
        &self,
        request: Request<DeleteStoryRequest>,
    ) -> Result<Response<DeleteStoryResponse>, Status> {
        log::debug!("Delete story");
        let request = request.get_ref(); // Upack request

        // Validate
        let story_id = Validate::uuid(&request.story_id)?;

        // Action
        self.repo
            .fetch_story(story_id)
            .and_then(|_| self.repo.delete_story(story_id))
            .await?;

        // Respond
        Ok(Response::new(DeleteStoryResponse {}))
    }

    /// Get a page of stories.
    async fn list_stories(
        &self,
        request: Request<ListStoriesRequest>,
    ) -> Result<Response<ListStoriesResponse>, Status> {
        log::debug!("List stories");
        let request = request.get_ref(); // Upack request

        // Validate
        let (cursor, limit) = Validate::page_bounds(request.cursor, request.limit);

        // Action
        let (next_cursor, stories) = self.repo.list_stories(cursor, limit).await?;

        // Respond
        let stories = stories.into_iter().map(|s| s.into()).collect();
        Ok(Response::new(ListStoriesResponse {
            next_cursor,
            stories,
        }))
    }

    /// Update an existing story.
    async fn update_story(
        &self,
        request: Request<UpdateStoryRequest>,
    ) -> Result<Response<UpdateStoryResponse>, Status> {
        log::debug!("Update story");
        let request = request.get_ref(); // Upack request

        // Validate
        let story_id = Validate::uuid(&request.story_id)?;
        let name = Validate::string_length(&request.name, "name")?;

        // Action
        let story = self
            .repo
            .fetch_story(story_id)
            .and_then(|_| self.repo.update_story(story_id, name))
            .await?;

        // Respond
        Ok(Response::new(UpdateStoryResponse {
            story: Some(story.into()),
        }))
    }

    /// List all tasks for a story.
    async fn list_tasks(
        &self,
        request: Request<ListTasksRequest>,
    ) -> Result<Response<ListTasksResponse>, Status> {
        log::debug!("List tasks");
        let request = request.get_ref(); // Upack request

        // Validate
        let story_id = Validate::uuid(&request.story_id)?;
        self.repo.fetch_story(story_id).await?;

        // Action
        let tasks = self
            .repo
            .list_tasks(story_id)
            .await?
            .into_iter()
            .map(|t| t.into())
            .collect();

        // Respond
        Ok(Response::new(ListTasksResponse { tasks }))
    }

    /// Create a new task.
    async fn create_task(
        &self,
        request: Request<CreateTaskRequest>,
    ) -> Result<Response<CreateTaskResponse>, Status> {
        log::debug!("Create task");
        let request = request.get_ref(); // Upack request

        // Validate
        let story_id = Validate::uuid(&request.story_id)?;
        let name = Validate::string_length(&request.name, "name")?;
        let status = Validate::status(&request.status)?;

        // Action
        let task = self.repo.create_task(story_id, name, status).await?;

        // Respond
        Ok(Response::new(CreateTaskResponse {
            task: Some(task.into()),
        }))
    }

    /// Delete an existing task.
    async fn delete_task(
        &self,
        request: Request<DeleteTaskRequest>,
    ) -> Result<Response<DeleteTaskResponse>, Status> {
        log::debug!("Delete task");
        let request = request.get_ref(); // Upack request

        // Validate
        let task_id = Validate::uuid(&request.task_id)?;

        // Action
        self.repo
            .fetch_task(task_id)
            .and_then(|_| self.repo.delete_task(task_id))
            .await?;

        // Respond
        Ok(Response::new(DeleteTaskResponse {}))
    }

    /// Update an existing task.
    async fn update_task(
        &self,
        request: Request<UpdateTaskRequest>,
    ) -> Result<Response<UpdateTaskResponse>, Status> {
        log::debug!("Update task");
        let request = request.get_ref(); // Upack request

        // Validate
        let task_id = Validate::uuid(&request.task_id)?;
        let status = Validate::status(&request.status)?;

        // Action
        let task = self
            .repo
            .fetch_task(task_id)
            .and_then(|t| self.repo.update_task(task_id, t.name, status))
            .await?;

        // Respond
        Ok(Response::new(UpdateTaskResponse {
            task: Some(task.into()),
        }))
    }
}
