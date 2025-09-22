use crate::{Result, domain::StoryId, repo::Repo, usecase::UseCase};

use async_trait::async_trait;
use futures_util::TryFutureExt;
use std::sync::Arc;

pub struct DeleteStory {
    repo: Arc<Repo>,
}

impl DeleteStory {
    pub fn new(repo: Arc<Repo>) -> Self {
        Self { repo }
    }
}

#[async_trait]
impl UseCase for DeleteStory {
    type Req = StoryId;
    type Rep = Result<()>;
    async fn execute(&self, req: Self::Req) -> Self::Rep {
        self.repo
            .fetch_story(&req)
            .and_then(|_| self.repo.delete_story(&req))
            .await
    }
}
