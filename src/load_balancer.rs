use std::sync::{Arc, Mutex}; // Importation de Arc pour le partage sécurisé entre threads et Mutex pour la synchronisation
use std::sync::atomic::{AtomicUsize, Ordering}; // Importation de AtomicUsize pour les opérations atomiques sur les indices
use crate::backend::BackendServer; // Importation de la structure BackendServer depuis le module backend

/// Trait pour les algorithmes de répartition de charge.
/// Définit une interface commune pour sélectionner un serveur backend.
pub trait LoadBalancer {
    /// Sélectionne un serveur backend parmi ceux disponibles.
    /// Une référence partagée au serveur backend sélectionné.
    fn select_backend(&self) -> Arc<BackendServer>;
}

/// Répartition de charge Round Robin.
/// Cet algorithme sélectionne les serveurs backend de manière circulaire.
pub struct RoundRobinLoadBalancer {
    backends: Vec<Arc<BackendServer>>, // Liste des serveurs backend disponibles
    current: AtomicUsize, // Indice actuel pour la sélection du serveur backend
}

impl RoundRobinLoadBalancer {
    /// Crée une nouvelle instance de `RoundRobinLoadBalancer`.
    /// Une nouvelle instance de `RoundRobinLoadBalancer`.
    pub fn new(backends: Vec<Arc<BackendServer>>) -> Self {
        Self {
            backends,
            current: AtomicUsize::new(0), // Initialise l'indice courant à 0
        }
    }
}

impl LoadBalancer for RoundRobinLoadBalancer {
    /// Sélectionne un serveur backend en utilisant l'algorithme Round Robin.

    /// Une référence partagée au serveur backend sélectionné.
    fn select_backend(&self) -> Arc<BackendServer> {
        // Récupère l'indice du serveur backend à sélectionner
        let index = self.current.fetch_add(1, Ordering::SeqCst) % self.backends.len();
        // Retourne une copie du serveur backend à l'indice sélectionné
        self.backends[index].clone()
    }
}

/// Répartition de charge Round Robin pondéré.
/// Cet algorithme sélectionne les serveurs backend en fonction de poids attribués à chaque serveur.
pub struct WeightedRoundRobinLoadBalancer {
    backends: Arc<Mutex<Vec<(Arc<BackendServer>, u32)>>>, // Liste des serveurs backend avec leurs poids respectifs
    current: AtomicUsize, // Indice actuel pour la sélection du serveur backend
}

impl WeightedRoundRobinLoadBalancer {
    /// Crée une nouvelle instance de `WeightedRoundRobinLoadBalancer`.
    /// Une nouvelle instance de `WeightedRoundRobinLoadBalancer`.
    pub fn new(backends: Vec<(Arc<BackendServer>, u32)>) -> Self {
        Self {
            backends: Arc::new(Mutex::new(backends)), // Enveloppe la liste des backends dans un Mutex pour la synchronisation
            current: AtomicUsize::new(0), // Initialise l'indice courant à 0
        }
    }
}

impl LoadBalancer for WeightedRoundRobinLoadBalancer {
    /// Sélectionne un serveur backend en utilisant l'algorithme Round Robin pondéré.
    
    /// Une référence partagée au serveur backend sélectionné.
    fn select_backend(&self) -> Arc<BackendServer> {
        // Verrouille l'accès à la liste des serveurs backend pour une lecture sécurisée
        let backends = self.backends.lock().unwrap();
        // Récupère l'indice du serveur backend à sélectionner
        let index = self.current.fetch_add(1, Ordering::SeqCst) % backends.len();
        // Retourne une copie du serveur backend à l'indice sélectionné
        backends[index].0.clone()
    }
}

/// Répartition de charge basée sur le nombre de connexions.
/// Cet algorithme sélectionne le serveur backend avec le moins de connexions actuelles.
pub struct LeastConnectionsLoadBalancer {
    backends: Arc<Mutex<Vec<(Arc<BackendServer>, usize)>>>, // Liste des serveurs backend avec le nombre actuel de connexions
}

impl LeastConnectionsLoadBalancer {
    /// Crée une nouvelle instance de `LeastConnectionsLoadBalancer`.
    
    /// Une nouvelle instance de `LeastConnectionsLoadBalancer`.
    pub fn new(backends: Vec<(Arc<BackendServer>, usize)>) -> Self {
        Self {
            backends: Arc::new(Mutex::new(backends)), // Enveloppe la liste des backends dans un Mutex pour la synchronisation
        }
    }
}

impl LoadBalancer for LeastConnectionsLoadBalancer {
    /// Sélectionne un serveur backend en utilisant l'algorithme de la moindre connexion.
    
    /// Une référence partagée au serveur backend sélectionné.
    fn select_backend(&self) -> Arc<BackendServer> {
        // Verrouille l'accès à la liste des serveurs backend pour une lecture sécurisée
        let mut backends = self.backends.lock().unwrap();
        // Trouve le serveur backend avec le moins de connexions
        let (backend, connections) = backends.iter_mut().min_by_key(|(_, connections)| *connections).unwrap();
        // Incrémente le nombre de connexions pour le serveur sélectionné
        *connections += 1;
        // Retourne une copie du serveur backend sélectionné
        backend.clone()
    }
}
