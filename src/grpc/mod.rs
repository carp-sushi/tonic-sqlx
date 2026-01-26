use crate::{
    domain::{Status, StoryEffects, TaskEffects},
    proto::gsdx_service_server::GsdxService,
    proto::{
        CreateStoryRequest, CreateStoryResponse, CreateTaskRequest, CreateTaskResponse,
        DeleteStoryRequest, DeleteStoryResponse, DeleteTaskRequest, DeleteTaskResponse,
        ListStoriesRequest, ListStoriesResponse, ListTasksRequest, ListTasksResponse, StoryData,
        TaskData, TaskStatus, UpdateStoryRequest, UpdateStoryResponse, UpdateTaskRequest,
        UpdateTaskResponse,
    },
};
use std::sync::Arc;
use tonic::{Request, Response, Status as GrpcStatus};

// Conversions between grpc and domain types.
mod adapter;

// Stateless validation utility functions.
mod validate;
use validate::{NameValidator, clamp_page_bounds, validate_story_id, validate_task_id};

/// GSDX gRPC implementation.
pub struct Gsdx {
    story_effects: Arc<Box<dyn StoryEffects>>,
    task_effects: Arc<Box<dyn TaskEffects>>,
}

impl Gsdx {
    /// Constructor
    pub fn new(
        story_effects: Arc<Box<dyn StoryEffects>>,
        task_effects: Arc<Box<dyn TaskEffects>>,
    ) -> Self {
        Self {
            story_effects,
            task_effects,
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
        let request = request.into_inner();
        let name = NameValidator::validate(request.name)?;
        let story = self.story_effects.create_story(name).await?;
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
        self.story_effects.delete_story(story_id).await?;
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
        let (next_cursor, stories) = self.story_effects.list_stories(page_params).await?;
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
        let name = NameValidator::validate(&request.name)?;
        let story = self.story_effects.update_story(story_id, name).await?;
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
        let tasks = self.task_effects.list_tasks(story_id).await?;
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
        let name = NameValidator::validate(&request.name)?;
        let task_status = TaskStatus::try_from(request.status).unwrap_or(TaskStatus::Unspecified);
        let status = Status::from(task_status);
        let task = self
            .task_effects
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
        self.task_effects.delete_task(task_id).await?;
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
        let maybe_name = NameValidator::validate_optional(request.name)?;
        let task_status = TaskStatus::try_from(request.status).unwrap_or(TaskStatus::Unspecified);
        let status = Status::from(task_status);
        let task = self
            .task_effects
            .update_task(task_id, maybe_name, status)
            .await?;
        Ok(Response::new(UpdateTaskResponse {
            task: Some(TaskData::from(task)),
        }))
    }
}
