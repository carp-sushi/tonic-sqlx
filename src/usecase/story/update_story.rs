use crate::{
    Result,
    domain::{Story, StoryId},
    repo::Repo,
    usecase::UseCase,
};
use async_trait::async_trait;
use futures_util::TryFutureExt;
use std::sync::Arc;

pub struct Args {
    pub story_id: StoryId,
    pub name: String,
}

pub struct UpdateStory {
    repo: Arc<Repo>,
}

impl UpdateStory {
    pub fn new(repo: Arc<Repo>) -> Self {
        Self { repo }
    }

    pub fn args(story_id: StoryId, name: String) -> Args {
        Args { story_id, name }
    }
}

#[async_trait]
impl UseCase for UpdateStory {
    type Req = Args;
    type Rep = Result<Story>;
    async fn execute(&self, req: Self::Req) -> Self::Rep {
        self.repo
            .fetch_story(&req.story_id)
            .and_then(|_| self.repo.update_story(&req.story_id, req.name))
            .await
    }
}
