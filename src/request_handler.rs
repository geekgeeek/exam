use hyper::{Body, Response};
use crate::backend::BackendServer;

pub struct RequestHandler {
    backend: BackendServer,
}

impl RequestHandler {
    pub fn new(backend: BackendServer) -> Self {
        Self { backend }
    }

    pub async fn handle_request(&self) -> Result<Response<Body>, hyper::Error> {
        let response_body = format!(
            "Handling request with backend: {}:{}",
            self.backend.address(),
            self.backend.port()
        );
        Ok(Response::new(Body::from(response_body)))
    }
}
