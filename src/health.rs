use std::sync::Arc; // Importation de Arc pour le partage sécurisé d'objets entre threads
use crate::backend::BackendServer; // Importation de la structure BackendServer depuis le module backend

/// Représente un vérificateur de santé pour les serveurs backend.
/// Ce module contient des méthodes pour vérifier si un serveur backend est en ligne et opérationnel.
pub struct HealthChecker;

impl HealthChecker {
    /// Vérifie la santé d'un serveur backend en envoyant une requête HTTP à son endpoint de santé.
   
    /// La méthode retourne `false` en cas d'erreur de requête HTTP, ce qui inclut les erreurs de réseau ou les réponses
    /// d'erreur du serveur.
    pub async fn check_health(backend: Arc<BackendServer>) -> bool {
        // Crée l'URL de l'endpoint de santé du serveur backend en utilisant son adresse et son port
        let url = format!("http://{}:{}/health", backend.address(), backend.port());

        // Envoie une requête GET asynchrone à l'URL de l'endpoint de santé
        // Vérifie si la réponse est une réponse HTTP valide (status code 200-299)
        reqwest::get(&url).await.is_ok()
    }
}

