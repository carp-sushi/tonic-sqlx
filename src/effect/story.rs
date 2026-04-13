use crate::{
    Result,
    domain::{Page, PageParams, Story, StoryId},
};
use std::future::Future;

/// Abstract type for read-only effects that can be performed on stories.
pub trait StoryReader: Send + Sync {
    /// Fetch a page of stories
    fn list(&self, page_params: PageParams) -> impl Future<Output = Result<Page<Story>>> + Send;
}

/// Abstract type for write effects that can be performed on stories.
pub trait StoryWriter: Send + Sync {
    /// Create a new story
    fn create(&self, name: String) -> impl Future<Output = Result<Story>> + Send;

    /// Update an existing story
    fn update(&self, story_id: StoryId, name: String) -> impl Future<Output = Result<Story>> + Send;

    /// Delete an existing story
    fn delete(&self, story_id: StoryId) -> impl Future<Output = Result<()>> + Send;
}
