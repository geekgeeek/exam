use hyper::{Body, Response}; // Importation des types nécessaires de la bibliothèque hyper pour les réponses HTTP
use crate::backend::BackendServer; // Importation de la structure BackendServer pour représenter les serveurs backend

/// Structure qui représente un gestionnaire de requêtes.
pub struct RequestHandler {
    backend: BackendServer, // Serveur backend associé à ce gestionnaire de requêtes
}

impl RequestHandler {
    /// Crée une nouvelle instance de RequestHandler avec un serveur backend spécifié.
    ///
    /// Une instance de RequestHandler initialisée avec le serveur backend fourni.
    pub fn new(backend: BackendServer) -> Self {
        Self { backend } // Initialise et retourne une nouvelle instance de RequestHandler
    }

    /// Gère une requête HTTP et retourne une réponse.
    
    /// Une réponse HTTP avec un message indiquant le serveur backend utilisé pour le traitement.
    pub async fn handle_request(&self) -> Result<Response<Body>, hyper::Error> {
        // Crée le corps de la réponse en incluant les informations du serveur backend
        let response_body = format!(
            "Handling request with backend: {}:{}",
            self.backend.address(), // Adresse du serveur backend
            self.backend.port() // Port du serveur backend
        );

        // Crée une nouvelle réponse HTTP avec le corps de la réponse et la retourne
        Ok(Response::new(Body::from(response_body)))
    }
}
