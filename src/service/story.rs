use crate::{
    Result,
    domain::{Cursor, PageParams, Story, StoryId},
    effect::StoryEffects,
    repo::Repo,
};
use async_trait::async_trait;
use futures_util::TryFutureExt;
use std::sync::Arc;

/// Story service
pub struct StoryService {
    repo: Arc<Repo>,
}

impl StoryService {
    /// Constructor
    pub fn new(repo: Arc<Repo>) -> Self {
        Self { repo }
    }
}

#[async_trait]
impl StoryEffects for StoryService {
    /// Fetch a page of stories
    async fn list_stories(&self, (cursor, limit): PageParams) -> Result<(Cursor, Vec<Story>)> {
        self.repo.list_stories(cursor, limit).await
    }

    /// Create a new story
    async fn create_story(&self, name: String) -> Result<Story> {
        self.repo.create_story(name).await
    }

    /// Update an existing story
    async fn update_story(&self, story_id: StoryId, name: String) -> Result<Story> {
        self.repo
            .fetch_story(&story_id)
            .and_then(async |s| {
                if s.name == name {
                    log::debug!("Story name is the same, skipping update");
                    Ok(s)
                } else {
                    log::debug!("Updating story name");
                    self.repo.update_story(&story_id, name).await
                }
            })
            .await
    }

    /// Delete an existing story
    async fn delete_story(&self, story_id: StoryId) -> Result<()> {
        self.repo
            .fetch_story(&story_id)
            .and_then(|_| self.repo.delete_story(&story_id))
            .await
    }
}
