use crate::{
    Result,
    domain::{StoryId, Task},
    repo::Repo,
    usecase::UseCase,
};
use async_trait::async_trait;
use futures_util::TryFutureExt;
use std::sync::Arc;

/// Get tasks for a story.
pub struct ListTasks {
    repo: Arc<Repo>,
}

impl ListTasks {
    /// Constructor
    pub fn new(repo: Arc<Repo>) -> Self {
        Self { repo }
    }
}

#[async_trait]
impl UseCase for ListTasks {
    type Req = StoryId;
    type Rep = Result<Vec<Task>>;
    async fn execute(&self, req: Self::Req) -> Self::Rep {
        self.repo
            .fetch_story(&req)
            .and_then(|_| self.repo.list_tasks(&req))
            .await
    }
}
