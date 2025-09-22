use async_trait::async_trait;

/// Story use cases.
pub mod story;

/// Defines a single unit of business logic.
/// Inspired by Finagle's Service type: `trait Service[Req, Rep] extends (Req => Future[Rep])`
/// TODO: Look at using `Fn` (requires unstable feature "fn_traits").
#[async_trait]
pub trait UseCase: Sized + Send + Sync + 'static {
    /// Input type
    type Req: Send + 'static;

    /// Output type
    type Rep: Send + 'static;

    /// Business logic
    async fn execute(&self, req: Self::Req) -> Self::Rep;
}
