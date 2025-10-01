use tonic::service::Interceptor;
use tonic::{Request, Status};

/// Request audit interceptor.
#[derive(Debug, Clone)]
pub struct RequestInterceptor {}

impl RequestInterceptor {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for RequestInterceptor {
    fn default() -> Self {
        Self::new()
    }
}

impl Interceptor for RequestInterceptor {
    fn call(&mut self, request: Request<()>) -> Result<Request<()>, Status> {
        log::info!("Got {:?} from {:?}", request, request.remote_addr());
        Ok(request)
    }
}
