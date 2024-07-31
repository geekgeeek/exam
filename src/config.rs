use serde::Deserialize; // Importation de Deserialize pour la désérialisation des données depuis le format TOML
use std::fs; // Importation de la bibliothèque pour les opérations sur le système de fichiers

/// Représente la configuration d'un serveur backend.
/// Cette structure est utilisée pour désérialiser les données du serveur backend depuis le fichier de configuration.
#[derive(Deserialize)]
pub struct BackendConfig {
    pub address: String, // Adresse IP ou nom d'hôte du serveur backend
    pub port: u16,       // Port sur lequel le serveur backend écoute
}

/// Représente la configuration globale de l'application.
/// Contient des informations sur le load balancer et une liste de serveurs backend.
#[derive(Deserialize)]
pub struct Config {
    pub load_balancer: String,           // Type d'algorithme de load balancing (par exemple, RoundRobin)
    pub backend_servers: Vec<BackendConfig>, // Liste des serveurs backend à utiliser
}

/// Charge la configuration depuis un fichier TOML et retourne un objet `Config`.

/// Cette fonction retourne une erreur si le fichier ne peut pas être lu ou si la désérialisation échoue.
/// Elle retourne également une erreur si aucune configuration de serveur backend n'est spécifiée.
pub fn load_config(filename: &str) -> Result<Config, Box<dyn std::error::Error>> {
    // Lit le contenu du fichier de configuration en tant que chaîne de caractères
    let config_str = fs::read_to_string(filename)?;
    
    // Désérialise la chaîne de caractères en un objet `Config` en utilisant la bibliothèque TOML
    let config: Config = toml::from_str(&config_str)?;
    
    // Vérifie si la liste des serveurs backend est vide
    if config.backend_servers.is_empty() {
        return Err("No backend servers specified".into()); // Retourne une erreur si aucun serveur backend n'est spécifié
    }
    
    // Retourne la configuration chargée si tout est correct
    Ok(config)
}
