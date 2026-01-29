use crate::{
    Result,
    domain::{Cursor, PageParams, Story, StoryId},
};
use async_trait::async_trait;

/// Abstract type for stateful I/O effects that can be performed on stories.
#[async_trait]
pub trait StoryEffects: Send + Sync {
    /// Create a new story
    async fn create_story(&self, name: String) -> Result<Story>;

    /// Fetch a page of stories
    async fn list_stories(&self, page_params: PageParams) -> Result<(Cursor, Vec<Story>)>;

    /// Update an existing story
    async fn update_story(&self, story_id: StoryId, name: String) -> Result<Story>;

    /// Delete an existing story
    async fn delete_story(&self, story_id: StoryId) -> Result<()>;
}
