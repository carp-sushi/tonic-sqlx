use crate::{
    repo::Repo,
    usecase::task::{CreateTask, DeleteTask, ListTasks, UpdateTask},
};
use std::sync::Arc;

/// Task use cases.
pub struct TaskService {
    pub delete: DeleteTask,
    pub list: ListTasks,
    pub create: CreateTask,
    pub update: UpdateTask,
}

impl TaskService {
    /// Constructor
    pub fn new(repo: Arc<Repo>) -> Self {
        Self {
            delete: DeleteTask::new(Arc::clone(&repo)),
            list: ListTasks::new(Arc::clone(&repo)),
            create: CreateTask::new(Arc::clone(&repo)),
            update: UpdateTask::new(Arc::clone(&repo)),
        }
    }
}
