use crate::{
    Result,
    domain::{Cursor, PageParams, Story, StoryEffects, StoryId},
    repo::Repo,
    usecase::UseCase,
    usecase::story::{CreateStory, DeleteStory, ListStories, UpdateStory},
};
use async_trait::async_trait;
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
    async fn list_stories(&self, page_params: PageParams) -> Result<(Cursor, Vec<Story>)> {
        ListStories::new(self.repo.clone())
            .execute(page_params)
            .await
    }

    /// Create a new story
    async fn create_story(&self, name: String) -> Result<Story> {
        CreateStory::new(self.repo.clone()).execute(name).await
    }

    /// Update an existing story
    async fn update_story(&self, story_id: StoryId, name: String) -> Result<Story> {
        UpdateStory::new(self.repo.clone())
            .execute(UpdateStory::args(story_id, name))
            .await
    }

    /// Delete an existing story
    async fn delete_story(&self, story_id: StoryId) -> Result<()> {
        DeleteStory::new(self.repo.clone()).execute(story_id).await
    }
}
