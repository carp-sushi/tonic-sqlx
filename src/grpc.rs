use crate::{
    Error,
    domain::{Status, Story, StoryId, Task},
    proto::gsdx_service_server::GsdxService,
    proto::{
        CreateStoryRequest, CreateStoryResponse, CreateTaskRequest, CreateTaskResponse,
        DeleteStoryRequest, DeleteStoryResponse, DeleteTaskRequest, DeleteTaskResponse,
        ListStoriesRequest, ListStoriesResponse, ListTasksRequest, ListTasksResponse, StoryData,
        TaskData, TaskStatus, UpdateStoryRequest, UpdateStoryResponse, UpdateTaskRequest,
        UpdateTaskResponse,
    },
    service::Service,
    usecase::{
        UseCase,
        story::UpdateStory,
        task::{CreateTask, UpdateTask},
    },
    util::{
        clamp_page_bounds, mk_prost_ts, validate_optional_string_length, validate_story_id,
        validate_string_length, validate_task_id,
    },
};

use tonic::{Request, Response, Status as GrpcStatus};

/// GSDX gRPC implementation.
pub struct Gsdx {
    ctx: Service,
}

impl Gsdx {
    /// Constructor
    pub fn new(ctx: Service) -> Self {
        Self { ctx }
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
        let StoryId(story_id) = story.id;
        Self {
            story_id: story_id.to_string(),
            name: story.name,
            created_at: mk_prost_ts(story.created_at),
            updated_at: mk_prost_ts(story.updated_at),
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
        let status = TaskStatus::from(task.status) as i32;
        Self {
            task_id: task.id.to_string(),
            story_id: task.story_id.to_string(),
            name: task.name,
            status,
            created_at: mk_prost_ts(task.created_at),
            updated_at: mk_prost_ts(task.updated_at),
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
        let story = self.ctx.stories.create_story.execute(name).await?;
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
        self.ctx.stories.delete_story.execute(story_id).await?;
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
        let (next_cursor, stories) = self.ctx.stories.list_stories.execute(page_params).await?;
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

        let args = UpdateStory::args(story_id, name);
        let story = self.ctx.stories.update_story.execute(args).await?;

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
        let tasks = self.ctx.tasks.list_tasks.execute(story_id).await?;
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
        let status = TaskStatus::try_from(request.status).unwrap_or(TaskStatus::Unspecified);

        let args = CreateTask::args(story_id, name, Status::from(status));
        let task = self.ctx.tasks.create_task.execute(args).await?;

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
        self.ctx.tasks.delete_task.execute(task_id).await?;
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
        let status = TaskStatus::try_from(request.status).unwrap_or(TaskStatus::Unspecified);

        let args = UpdateTask::args(task_id, maybe_name, Status::from(status));
        let task = self.ctx.tasks.update_task.execute(args).await?;

        Ok(Response::new(UpdateTaskResponse {
            task: Some(TaskData::from(task)),
        }))
    }
}
