use hyper::{Body, Request, Response, Server}; // Importation des composants de la bibliothèque hyper pour le serveur HTTP
use hyper::service::{make_service_fn, service_fn}; // Importation des fonctions pour créer des services HTTP
use std::sync::Arc; // Importation de Arc pour la gestion des références partagées entre threads
use tokio::sync::Mutex; // Importation de Mutex pour la synchronisation dans le contexte de Tokio
use std::error::Error; // Importation du trait Error pour le traitement des erreurs
use crate::config::load_config; // Importation de la fonction pour charger la configuration
use crate::backend::BackendServer; // Importation de la structure BackendServer pour représenter les serveurs backend
use crate::load_balancer::{LoadBalancer, RoundRobinLoadBalancer, WeightedRoundRobinLoadBalancer, LeastConnectionsLoadBalancer}; // Importation des algorithmes de répartition de charge
use crate::request_handler::RequestHandler; // Importation du gestionnaire de requêtes

/// Fonction pour gérer les requêtes HTTP entrantes.
/// Utilise un gestionnaire de requêtes pour traiter la requête.
async fn handle_request(req: Request<Body>, request_handler: Arc<RequestHandler>) -> Result<Response<Body>, hyper::Error> {
    // Appelle la méthode handle_request du RequestHandler pour traiter la requête
    request_handler.handle_request().await
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Charger la configuration depuis le fichier config.toml
    let config = load_config("config/config.toml")?;

    // Créer les serveurs backends à partir des informations de la configuration
    let backends: Vec<_> = config.backend_servers.iter()
        .map(|b| {
            BackendServer::new(b.address.clone(), b.port)
        })
        .collect();

    // Initialiser le load balancer en fonction de la stratégie spécifiée dans la configuration
    let load_balancer: Arc<dyn LoadBalancer + Send + Sync> = match config.load_balancer.as_str() {
        "round_robin" => Arc::new(RoundRobinLoadBalancer::new(backends.clone())), // Utilise le Round Robin si spécifié
        "weighted_round_robin" => {
            // Crée des paires de serveurs et de poids pour le Weighted Round Robin
            let weighted_backends = backends.iter().map(|b| (b.clone(), 1)).collect();
            Arc::new(WeightedRoundRobinLoadBalancer::new(weighted_backends))
        },
        "least_connections" => {
            // Crée des paires de serveurs et de connexions initiales pour le Least Connections
            let least_connections_backends = backends.iter().map(|b| (b.clone(), 0)).collect();
            Arc::new(LeastConnectionsLoadBalancer::new(least_connections_backends))
        },
        _ => panic!("Unknown load balancer strategy"), // Panique si la stratégie de load balancer est inconnue
    };

    // Crée un gestionnaire de requêtes en passant le load balancer
    let request_handler = Arc::new(RequestHandler::new(load_balancer));

    // Crée un service HTTP avec hyper
    let make_svc = make_service_fn(|_conn| {
        let request_handler = request_handler.clone();
        // Crée un service pour chaque connexion entrante, en utilisant le gestionnaire de requêtes
        async move {
            Ok::<_, hyper::Error>(service_fn(move |req| handle_request(req, request_handler.clone())))
        }
    });

    // Configure l'adresse du serveur
    let addr = ([127, 0, 0, 1], 3000).into(); // Adresse locale et port 3000
    let server = Server::bind(&addr).serve(make_svc); // Crée le serveur HTTP en liant l'adresse et en servant le service

    println!("Listening on http://{}", addr); // Affiche l'adresse sur laquelle le serveur écoute

    // Démarre le serveur et attend les requêtes
    if let Err(e) = server.await {
        eprintln!("Server error: {}", e); // Affiche une erreur si le serveur rencontre un problème
    }

    Ok(())
}
