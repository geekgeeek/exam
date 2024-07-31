use std::sync::Arc;
use crate::backend::BackendServer;

pub struct Client {
    name: String,
    backend: Arc<BackendServer>,
}

impl Client {
    pub fn new(name: &str, backend: Arc<BackendServer>) -> Self {
        Self {
            name: name.to_string(),
            backend,
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_backend(&self) -> Arc<BackendServer> {
        Arc::clone(&self.backend)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_creation() {
        let backend1 = Arc::new(BackendServer::new("127.0.0.1".to_string(), 8080));
        let client1 = Client::new("Client1", Arc::clone(&backend1));
        
        assert_eq!(client1.get_name(), "Client1");
        assert_eq!(client1.get_backend().address(), "127.0.0.1");
        assert_eq!(client1.get_backend().port(), 8080);
    }

    #[test]
    fn test_client_backend() {
        let backend2 = Arc::new(BackendServer::new("192.168.1.1".to_string(), 9090));
        let client2 = Client::new("Client2", Arc::clone(&backend2));

        assert_eq!(client2.get_name(), "Client2");
        assert_eq!(client2.get_backend().address(), "192.168.1.1");
        assert_eq!(client2.get_backend().port(), 9090);
    }
}
