use crate::{
    repo::Repo,
    usecase::story::{CreateStory, DeleteStory, ListStories, UpdateStory},
};
use std::sync::Arc;

/// Story use cases.
pub struct StoryService {
    pub create_story: CreateStory,
    pub list_stories: ListStories,
    pub delete_story: DeleteStory,
    pub update_story: UpdateStory,
}

impl StoryService {
    /// Constructor
    pub fn new(repo: Arc<Repo>) -> Self {
        Self {
            create_story: CreateStory::new(Arc::clone(&repo)),
            list_stories: ListStories::new(Arc::clone(&repo)),
            delete_story: DeleteStory::new(Arc::clone(&repo)),
            update_story: UpdateStory::new(Arc::clone(&repo)),
        }
    }
}
