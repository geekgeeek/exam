use std::sync::Arc;

pub struct BackendServer {
    address: String,
    port: u16,
}

impl BackendServer {
    pub fn new(address: String, port: u16) -> Arc<Self> {
        Arc::new(Self { address, port })
    }

    pub fn address(&self) -> &str {
        &self.address
    }

    pub fn port(&self) -> u16 {
        self.port
    }
}
