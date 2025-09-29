use crate::{
    Result,
    domain::{Status, StoryId, Task},
    repo::Repo,
    usecase::UseCase,
};
use async_trait::async_trait;
use std::sync::Arc;

#[derive(Debug)]
pub struct Args(pub StoryId, pub String, pub Status);

pub struct CreateTask {
    repo: Arc<Repo>,
}

impl CreateTask {
    pub fn new(repo: Arc<Repo>) -> Self {
        Self { repo }
    }

    pub fn args(story_id: StoryId, name: String, status: Status) -> Args {
        Args(story_id, name, status)
    }
}

#[async_trait]
impl UseCase for CreateTask {
    type Req = Args;
    type Rep = Result<Task>;
    async fn execute(&self, req: Self::Req) -> Self::Rep {
        let Args(story_id, name, status) = req;
        self.repo.create_task(&story_id, name, status).await
    }
}
