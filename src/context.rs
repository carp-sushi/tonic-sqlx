use crate::{
    repo::Repo,
    usecase::{
        story::{CreateStory, DeleteStory, ListStories, UpdateStory},
        task::{CreateTask, DeleteTask, ListTasks, UpdateTask},
    },
};
use std::sync::Arc;

/// A container for use cases.
pub struct Context {
    pub stories: StoryContext,
    pub tasks: TaskContext,
}

/// Story use cases.
pub struct StoryContext {
    pub create_story: CreateStory,
    pub list_stories: ListStories,
    pub delete_story: DeleteStory,
    pub update_story: UpdateStory,
}

/// Task use cases.
pub struct TaskContext {
    pub delete_task: DeleteTask,
    pub list_tasks: ListTasks,
    pub create_task: CreateTask,
    pub update_task: UpdateTask,
}

impl Context {
    /// Constructor
    pub fn new(repo: Arc<Repo>) -> Self {
        Self {
            stories: StoryContext::new(Arc::clone(&repo)),
            tasks: TaskContext::new(Arc::clone(&repo)),
        }
    }
}

impl StoryContext {
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

impl TaskContext {
    /// Constructor
    pub fn new(repo: Arc<Repo>) -> Self {
        Self {
            delete_task: DeleteTask::new(Arc::clone(&repo)),
            list_tasks: ListTasks::new(Arc::clone(&repo)),
            create_task: CreateTask::new(Arc::clone(&repo)),
            update_task: UpdateTask::new(Arc::clone(&repo)),
        }
    }
}
