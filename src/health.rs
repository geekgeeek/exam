use std::sync::Arc;
use crate::backend::BackendServer;

pub struct HealthChecker;

impl HealthChecker {
    pub async fn check_health(backend: Arc<BackendServer>) -> bool {
        let url = format!("http://{}:{}/health", backend.address(), backend.port());
        reqwest::get(&url).await.is_ok()
    }
}
