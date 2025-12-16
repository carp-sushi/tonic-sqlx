use crate::{
    repo::Repo,
    usecase::story::{CreateStory, DeleteStory, ListStories, UpdateStory},
};
use std::sync::Arc;

/// Story use cases.
pub struct StoryService {
    pub create: CreateStory,
    pub list: ListStories,
    pub delete: DeleteStory,
    pub update: UpdateStory,
}

impl StoryService {
    /// Constructor
    pub fn new(repo: Arc<Repo>) -> Self {
        Self {
            create: CreateStory::new(Arc::clone(&repo)),
            list: ListStories::new(Arc::clone(&repo)),
            delete: DeleteStory::new(Arc::clone(&repo)),
            update: UpdateStory::new(Arc::clone(&repo)),
        }
    }
}
