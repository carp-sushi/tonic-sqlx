use crate::{
    repo::Repo,
    usecase::task::{CreateTask, DeleteTask, ListTasks, UpdateTask},
};
use std::sync::Arc;

/// Task use cases.
pub struct TaskService {
    pub delete_task: DeleteTask,
    pub list_tasks: ListTasks,
    pub create_task: CreateTask,
    pub update_task: UpdateTask,
}
impl TaskService {
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
