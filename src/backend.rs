use std::sync::Arc;
/// Représente un serveur backend dans le système de load balancing.
/// Contient l'adresse et le port du serveur backend.

pub struct BackendServer {
    address: String,  // Adresse IP ou nom d'hôte du serveur backend
    port: u16,       // Port sur lequel le serveur backend écoute
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
