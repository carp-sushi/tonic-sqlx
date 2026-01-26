use crate::{
    Result,
    domain::{Cursor, PageParams, Story},
    repo::Repo,
    usecase::UseCase,
};
use async_trait::async_trait;
use std::sync::Arc;

/// Get pages of stories.
pub struct ListStories {
    repo: Arc<Repo>,
}

impl ListStories {
    /// Constructor
    pub fn new(repo: Arc<Repo>) -> Self {
        Self { repo }
    }
}

#[async_trait]
impl UseCase for ListStories {
    type Req = PageParams;
    type Rep = Result<(Cursor, Vec<Story>)>;
    async fn execute(&self, req: Self::Req) -> Self::Rep {
        let (cursor, limit) = req;
        self.repo.list_stories(cursor, limit).await
    }
}
