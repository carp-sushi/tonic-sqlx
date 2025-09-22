use crate::{Result, domain::Story, repo::Repo, usecase::UseCase};
use async_trait::async_trait;
use std::sync::Arc;

pub struct CreateStory {
    repo: Arc<Repo>,
}

impl CreateStory {
    pub fn new(repo: Arc<Repo>) -> Self {
        Self { repo }
    }
}

#[async_trait]
impl UseCase for CreateStory {
    type Req = String;
    type Rep = Result<Story>;
    async fn execute(&self, req: Self::Req) -> Self::Rep {
        self.repo.create_story(req).await
    }
}
