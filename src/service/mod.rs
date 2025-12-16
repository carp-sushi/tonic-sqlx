use crate::repo::Repo;
use std::sync::Arc;

mod story;
mod task;

pub use story::StoryService;
pub use task::TaskService;

/// A container for services.
pub struct Service {
    pub story: StoryService,
    pub task: TaskService,
}

impl Service {
    /// Constructor
    pub fn new(repo: Arc<Repo>) -> Self {
        Self {
            story: StoryService::new(Arc::clone(&repo)),
            task: TaskService::new(Arc::clone(&repo)),
        }
    }
}
