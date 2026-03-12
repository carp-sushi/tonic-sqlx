use crate::{
    Result,
    domain::{PageParams, Story, StoryId, StoryPage},
};
use async_trait::async_trait;

/// Abstract type for stateful I/O effects that can be performed on stories.
#[async_trait]
pub trait StoryEffects: Send + Sync {
    /// Create a new story
    async fn create(&self, name: String) -> Result<Story>;

    /// Fetch a page of stories
    async fn list(&self, page_params: PageParams) -> Result<StoryPage>;

    /// Update an existing story
    async fn update(&self, story_id: StoryId, name: String) -> Result<Story>;

    /// Delete an existing story
    async fn delete(&self, story_id: StoryId) -> Result<()>;
}
