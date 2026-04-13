use crate::{
    Result,
    domain::{Page, PageParams, Story, StoryId},
    effect::{StoryReader, StoryWriter},
    repo::Repo,
};
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

impl StoryReader for StoryService {
    async fn list(&self, page_params: PageParams) -> Result<Page<Story>> {
        self.repo.list_stories(page_params).await
    }
}

impl StoryWriter for StoryService {
    async fn create(&self, name: String) -> Result<Story> {
        self.repo.create_story(name).await
    }

    async fn update(&self, story_id: StoryId, name: String) -> Result<Story> {
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

    async fn delete(&self, story_id: StoryId) -> Result<()> {
        self.repo
            .fetch_story(&story_id)
            .and_then(|_| self.repo.delete_story(&story_id))
            .await
    }
}
