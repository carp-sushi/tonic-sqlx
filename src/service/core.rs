use crate::{
    Error,
    domain::{Status, Story, Task},
    proto::gsdx_service_server::GsdxService,
    proto::*,
    repo::Repo,
};

use super::util::mk_prost_timestamp;
use super::validate::Validate;
use futures_util::TryFutureExt;
use tonic::{Request, Response, Status as GrpcStatus};

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
        Self {
            story_id: story.id.to_string(),
            name: story.name,
            created_at: Some(mk_prost_timestamp(story.created_at)),
            updated_at: Some(mk_prost_timestamp(story.updated_at)),
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
        let status = TaskStatus::from(task.status()) as i32;
        Self {
            task_id: task.id.to_string(),
            story_id: task.story_id.to_string(),
            name: task.name,
            status,
            created_at: Some(mk_prost_timestamp(task.created_at)),
            updated_at: Some(mk_prost_timestamp(task.updated_at)),
        }
    }
}

#[tonic::async_trait]
impl GsdxService for Service {
    /// Create a new story.
    async fn create_story(
        &self,
        request: Request<CreateStoryRequest>,
    ) -> Result<Response<CreateStoryResponse>, GrpcStatus> {
        log::debug!("Create story");
        let request = request.get_ref(); // Upack request

        // Validate
        let name = Validate::string_length(&request.name, "name")?;

        // Action
        let story = self.repo.create_story(name).await?;

        // Respond
        Ok(Response::new(CreateStoryResponse {
            story: Some(StoryData::from(story)),
        }))
    }

    /// Delete an existing story.
    async fn delete_story(
        &self,
        request: Request<DeleteStoryRequest>,
    ) -> Result<Response<DeleteStoryResponse>, GrpcStatus> {
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
    ) -> Result<Response<ListStoriesResponse>, GrpcStatus> {
        log::debug!("List stories");
        let request = request.get_ref(); // Upack request

        // Validate
        let (cursor, limit) = Validate::page_bounds(request.cursor, request.limit);

        // Action
        let (next_cursor, stories) = self.repo.list_stories(cursor, limit).await?;

        // Respond
        let stories = stories.into_iter().map(StoryData::from).collect();
        Ok(Response::new(ListStoriesResponse {
            next_cursor,
            stories,
        }))
    }

    /// Update an existing story.
    async fn update_story(
        &self,
        request: Request<UpdateStoryRequest>,
    ) -> Result<Response<UpdateStoryResponse>, GrpcStatus> {
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
            story: Some(StoryData::from(story)),
        }))
    }

    /// List all tasks for a story.
    async fn list_tasks(
        &self,
        request: Request<ListTasksRequest>,
    ) -> Result<Response<ListTasksResponse>, GrpcStatus> {
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
            .map(TaskData::from)
            .collect();

        // Respond
        Ok(Response::new(ListTasksResponse { tasks }))
    }

    /// Create a new task.
    async fn create_task(
        &self,
        request: Request<CreateTaskRequest>,
    ) -> Result<Response<CreateTaskResponse>, GrpcStatus> {
        log::debug!("Create task");
        let request = request.get_ref(); // Upack request

        // Validate
        let story_id = Validate::uuid(&request.story_id)?;
        let name = Validate::string_length(&request.name, "name")?;
        let proto_status = TaskStatus::try_from(request.status).unwrap_or(TaskStatus::Unspecified);
        let status = Status::from(proto_status);

        // Action
        let task = self.repo.create_task(story_id, name, status).await?;

        // Respond
        Ok(Response::new(CreateTaskResponse {
            task: Some(TaskData::from(task)),
        }))
    }

    /// Delete an existing task.
    async fn delete_task(
        &self,
        request: Request<DeleteTaskRequest>,
    ) -> Result<Response<DeleteTaskResponse>, GrpcStatus> {
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
    ) -> Result<Response<UpdateTaskResponse>, GrpcStatus> {
        log::debug!("Update task");
        let request = request.get_ref(); // Upack request

        // Validate
        let task_id = Validate::uuid(&request.task_id)?;
        let proto_status = TaskStatus::try_from(request.status).unwrap_or(TaskStatus::Unspecified);
        let status = Status::from(proto_status);

        // Action
        let task = self
            .repo
            .fetch_task(task_id)
            .and_then(|t| self.repo.update_task(task_id, t.name, status))
            .await?;

        // Respond
        Ok(Response::new(UpdateTaskResponse {
            task: Some(TaskData::from(task)),
        }))
    }
}
