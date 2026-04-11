use crate::{
    Result,
    domain::{Page, PageParams, Story, StoryId},
};
use async_trait::async_trait;

/// Abstract type for read-only effects that can be performed on stories.
#[async_trait]
pub trait StoryReader: Send + Sync {
    /// Fetch a page of stories
    async fn list(&self, page_params: PageParams) -> Result<Page<Story>>;
}

/// Abstract type for write effects that can be performed on stories.
#[async_trait]
pub trait StoryWriter: Send + Sync {
    /// Create a new story
    async fn create(&self, name: String) -> Result<Story>;

    /// Update an existing story
    async fn update(&self, story_id: StoryId, name: String) -> Result<Story>;

    /// Delete an existing story
    async fn delete(&self, story_id: StoryId) -> Result<()>;
}
