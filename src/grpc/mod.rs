use crate::{
    domain::{Page, Status},
    effect::{StoryEffects, TaskEffects},
    proto::gsdx_service_server::GsdxService,
    proto::{
        CreateStoryRequest, CreateStoryResponse, CreateTaskRequest, CreateTaskResponse,
        DeleteStoryRequest, DeleteStoryResponse, DeleteTaskRequest, DeleteTaskResponse,
        ListStoriesRequest, ListStoriesResponse, ListTasksRequest, ListTasksResponse, StoryData,
        TaskData, TaskStatus, UpdateStoryRequest, UpdateStoryResponse, UpdateTaskRequest,
        UpdateTaskResponse,
    },
};
use tonic::{Request, Response, Status as GrpcStatus};

// Conversions between grpc and domain types.
mod adapter;

// Stateless validation utility functions.
mod validate;
use validate::{
    clamp_page_bounds, validate_name, validate_optional_name, validate_story_id, validate_task_id,
};

/// GSDX gRPC implementation.
pub struct Gsdx<S, T> {
    stories: S,
    tasks: T,
}

impl<S: StoryEffects, T: TaskEffects> Gsdx<S, T> {
    /// Constructor
    pub fn new(stories: S, tasks: T) -> Self {
        Self { stories, tasks }
    }
}

#[tonic::async_trait]
impl<S, T> GsdxService for Gsdx<S, T>
where
    S: StoryEffects + 'static,
    T: TaskEffects + 'static,
{
    /// Create a new story.
    async fn create_story(
        &self,
        request: Request<CreateStoryRequest>,
    ) -> Result<Response<CreateStoryResponse>, GrpcStatus> {
        log::debug!("Create story");
        let request = request.into_inner();
        let name = validate_name(request.name)?;
        let story = self.stories.create(name).await?;
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
        self.stories.delete(story_id).await?;
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
        let Page(next_cursor, stories) = self.stories.list(page_params).await?;
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
        let name = validate_name(&request.name)?;
        let story = self.stories.update(story_id, name).await?;
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
        let tasks = self.tasks.list(story_id).await?;
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
        let name = validate_name(&request.name)?;
        let task_status = TaskStatus::try_from(request.status).unwrap_or(TaskStatus::Unspecified);
        let status = Status::from(task_status);
        let task = self.tasks.create(story_id, name, status).await?;
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
        self.tasks.delete(task_id).await?;
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
        let maybe_name = validate_optional_name(request.name)?;
        let task_status = TaskStatus::try_from(request.status).unwrap_or(TaskStatus::Unspecified);
        let status = Status::from(task_status);
        let task = self.tasks.update(task_id, maybe_name, status).await?;
        Ok(Response::new(UpdateTaskResponse {
            task: Some(TaskData::from(task)),
        }))
    }
}
