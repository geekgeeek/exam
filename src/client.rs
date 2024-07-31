use std::sync::Arc; // Importation de Arc pour le partage sécurisé d'objets entre threads
use crate::backend::BackendServer; // Importation de la structure BackendServer depuis le module backend

/// Représente un client dans le système, avec un nom et une référence à un serveur backend.
/// Le client interagit avec le serveur backend pour envoyer des requêtes ou recevoir des données.
pub struct Client {
    name: String,                // Nom du client
    backend: Arc<BackendServer>, // Référence partagée au serveur backend
}

impl Client {
    /// Crée une nouvelle instance de `Client`.
    /// Une nouvelle instance de `Client`.
    pub fn new(name: &str, backend: Arc<BackendServer>) -> Self {
        Self {
            name: name.to_string(), // Convertit le nom en String et l'assigne au champ `name`
            backend,                // Assigne le backend au champ `backend`
        }
    }

    /// Une référence à une chaîne de caractères représentant le nom du client.
    pub fn get_name(&self) -> &str {
        &self.name
    }

    /// Une copie de l'Arc contenant la référence au serveur backend.
    pub fn get_backend(&self) -> Arc<BackendServer> {
        Arc::clone(&self.backend) // Crée une nouvelle référence partagée au serveur backend
    }
}

#[cfg(test)] // Indique que le module de tests doit être compilé uniquement pour les tests
mod tests {
    use super::*; // Importation des éléments du module parent pour les tests

    /// Teste la création d'un client et la récupération des informations.
    #[test]
    fn test_client_creation() {
        // Crée un serveur backend avec l'adresse 127.0.0.1 et le port 8080
        let backend1 = Arc::new(BackendServer::new("127.0.0.1".to_string(), 8080));
        // Crée un client avec le nom "Client1" et la référence au backend1
        let client1 = Client::new("Client1", Arc::clone(&backend1));
        
        // Vérifie que le nom du client est correct
        assert_eq!(client1.get_name(), "Client1");
        // Vérifie que l'adresse du backend est correcte
        assert_eq!(client1.get_backend().address(), "127.0.0.1");
        // Vérifie que le port du backend est correct
        assert_eq!(client1.get_backend().port(), 8080);
    }

    /// Teste la récupération des informations d'un client et son backend.
    #[test]
    fn test_client_backend() {
        // Crée un serveur backend avec l'adresse 192.168.1.1 et le port 9090
        let backend2 = Arc::new(BackendServer::new("192.168.1.1".to_string(), 9090));
        // Crée un client avec le nom "Client2" et la référence au backend2
        let client2 = Client::new("Client2", Arc::clone(&backend2));

        // Vérifie que le nom du client est correct
        assert_eq!(client2.get_name(), "Client2");
        // Vérifie que l'adresse du backend est correcte
        assert_eq!(client2.get_backend().address(), "192.168.1.1");
        // Vérifie que le port du backend est correct
        assert_eq!(client2.get_backend().port(), 9090);
    }
}
