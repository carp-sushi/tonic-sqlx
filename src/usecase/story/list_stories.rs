use crate::{Result, domain::Story, repo::Repo, usecase::UseCase};
use async_trait::async_trait;
use std::sync::Arc;

#[derive(Debug)]
pub struct Args {
    pub cursor: i64,
    pub limit: i64,
}

/// Get pages of stories.
pub struct ListStories {
    repo: Arc<Repo>,
}

impl ListStories {
    /// Constructor
    pub fn new(repo: Arc<Repo>) -> Self {
        Self { repo }
    }

    /// Use case arguments.
    pub fn args(cursor: i64, limit: i64) -> Args {
        Args { cursor, limit }
    }
}

#[async_trait]
impl UseCase for ListStories {
    type Req = Args;
    type Rep = Result<(i64, Vec<Story>)>;
    async fn execute(&self, req: Self::Req) -> Self::Rep {
        self.repo.list_stories(req.cursor, req.limit).await
    }
}
