use crate::{Result, domain::TaskId, repo::Repo, usecase::UseCase};
use async_trait::async_trait;
use futures_util::TryFutureExt;
use std::sync::Arc;

pub struct DeleteTask {
    repo: Arc<Repo>,
}

impl DeleteTask {
    pub fn new(repo: Arc<Repo>) -> Self {
        Self { repo }
    }
}

#[async_trait]
impl UseCase for DeleteTask {
    type Req = TaskId;
    type Rep = Result<()>;
    async fn execute(&self, req: Self::Req) -> Self::Rep {
        self.repo
            .fetch_task(&req)
            .and_then(|_| self.repo.delete_task(&req))
            .await
    }
}
