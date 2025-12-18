use crate::{
    domain::Status,
    proto::gsdx_service_server::GsdxService,
    proto::{
        CreateStoryRequest, CreateStoryResponse, CreateTaskRequest, CreateTaskResponse,
        DeleteStoryRequest, DeleteStoryResponse, DeleteTaskRequest, DeleteTaskResponse,
        ListStoriesRequest, ListStoriesResponse, ListTasksRequest, ListTasksResponse, StoryData,
        TaskData, TaskStatus, UpdateStoryRequest, UpdateStoryResponse, UpdateTaskRequest,
        UpdateTaskResponse,
    },
    service::{StoryService, TaskService},
};
use tonic::{Request, Response, Status as GrpcStatus};

// Conversions between grpc and domain types.
mod adapter;

// Stateless validation utility functions.
mod validate;
use validate::{
    clamp_page_bounds, validate_optional_string_length, validate_story_id, validate_string_length,
    validate_task_id,
};

/// GSDX gRPC implementation.
pub struct Gsdx {
    story_service: StoryService,
    task_service: TaskService,
}

impl Gsdx {
    /// Constructor
    pub fn new(story_service: StoryService, task_service: TaskService) -> Self {
        Self {
            story_service,
            task_service,
        }
    }
}

#[tonic::async_trait]
impl GsdxService for Gsdx {
    /// Create a new story.
    async fn create_story(
        &self,
        request: Request<CreateStoryRequest>,
    ) -> Result<Response<CreateStoryResponse>, GrpcStatus> {
        log::debug!("Create story");
        let request = request.get_ref();
        let name = validate_string_length(&request.name, "name")?;
        let story = self.story_service.create_story(name).await?;
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
        let request = request.get_ref();
        let story_id = validate_story_id(&request.story_id)?;
        self.story_service.delete_story(story_id).await?;
        Ok(Response::new(DeleteStoryResponse {}))
    }

    /// Get a page of stories.
    async fn list_stories(
        &self,
        request: Request<ListStoriesRequest>,
    ) -> Result<Response<ListStoriesResponse>, GrpcStatus> {
        log::debug!("List stories");
        let request = request.get_ref();
        let page_params = clamp_page_bounds(request.cursor, request.limit);
        let (next_cursor, stories) = self.story_service.list_stories(page_params).await?;
        Ok(Response::new(ListStoriesResponse {
            next_cursor,
            stories: stories.into_iter().map(StoryData::from).collect(),
        }))
    }

    /// Update an existing story.
    async fn update_story(
        &self,
        request: Request<UpdateStoryRequest>,
    ) -> Result<Response<UpdateStoryResponse>, GrpcStatus> {
        log::debug!("Update story");
        let request = request.get_ref();
        let story_id = validate_story_id(&request.story_id)?;
        let name = validate_string_length(&request.name, "name")?;
        let story = self.story_service.update_story(story_id, name).await?;
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
        let request = request.get_ref();
        let story_id = validate_story_id(&request.story_id)?;
        let tasks = self.task_service.list_tasks(story_id).await?;
        Ok(Response::new(ListTasksResponse {
            tasks: tasks.into_iter().map(TaskData::from).collect(),
        }))
    }

    /// Create a new task.
    async fn create_task(
        &self,
        request: Request<CreateTaskRequest>,
    ) -> Result<Response<CreateTaskResponse>, GrpcStatus> {
        log::debug!("Create task");
        let request = request.get_ref();
        let story_id = validate_story_id(&request.story_id)?;
        let name = validate_string_length(&request.name, "name")?;
        let task_status = TaskStatus::try_from(request.status).unwrap_or(TaskStatus::Unspecified);
        let status = Status::from(task_status);
        let task = self
            .task_service
            .create_task(story_id, name, status)
            .await?;
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
        let request = request.get_ref();
        let task_id = validate_task_id(&request.task_id)?;
        self.task_service.delete_task(task_id).await?;
        Ok(Response::new(DeleteTaskResponse {}))
    }

    /// Update an existing task.
    async fn update_task(
        &self,
        request: Request<UpdateTaskRequest>,
    ) -> Result<Response<UpdateTaskResponse>, GrpcStatus> {
        log::debug!("Update task");
        let request = request.into_inner();
        let task_id = validate_task_id(&request.task_id)?;
        let maybe_name = validate_optional_string_length(request.name, "name")?;
        let task_status = TaskStatus::try_from(request.status).unwrap_or(TaskStatus::Unspecified);
        let status = Status::from(task_status);
        let task = self
            .task_service
            .update_task(task_id, maybe_name, status)
            .await?;
        Ok(Response::new(UpdateTaskResponse {
            task: Some(TaskData::from(task)),
        }))
    }
}
