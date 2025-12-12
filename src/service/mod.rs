use crate::repo::Repo;
use std::sync::Arc;

mod story;
mod task;

pub use story::StoryService;
pub use task::TaskService;

/// A container for services.
pub struct Service {
    pub stories: StoryService,
    pub tasks: TaskService,
}

impl Service {
    /// Constructor
    pub fn new(repo: Arc<Repo>) -> Self {
        Self {
            stories: StoryService::new(Arc::clone(&repo)),
            tasks: TaskService::new(Arc::clone(&repo)),
        }
    }
}
