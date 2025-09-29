use crate::{
    Result,
    domain::{Status, Task, TaskId},
    repo::Repo,
    usecase::UseCase,
};
use async_trait::async_trait;
use futures_util::TryFutureExt;
use std::sync::Arc;

#[derive(Debug)]
pub struct Args(pub TaskId, pub Option<String>, pub Status);

pub struct UpdateTask {
    repo: Arc<Repo>,
}

impl UpdateTask {
    pub fn new(repo: Arc<Repo>) -> Self {
        Self { repo }
    }

    pub fn args(task_id: TaskId, name: Option<String>, status: Status) -> Args {
        Args(task_id, name, status)
    }
}

#[async_trait]
impl UseCase for UpdateTask {
    type Req = Args;
    type Rep = Result<Task>;
    async fn execute(&self, req: Self::Req) -> Self::Rep {
        let Args(task_id, maybe_name, status) = req;
        self.repo
            .fetch_task(&task_id)
            .and_then(|t| {
                let name = maybe_name.unwrap_or(t.name);
                self.repo.update_task(&task_id, name, status)
            })
            .await
    }
}
