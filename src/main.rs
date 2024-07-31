use hyper::{Body, Request, Response, Server};
use hyper::service::{make_service_fn, service_fn};
use std::sync::Arc;
use tokio::sync::Mutex;
use std::error::Error;
use crate::config::load_config;
use crate::backend::BackendServer;
use crate::load_balancer::{LoadBalancer, RoundRobinLoadBalancer, WeightedRoundRobinLoadBalancer, LeastConnectionsLoadBalancer};
use crate::request_handler::RequestHandler;

async fn handle_request(req: Request<Body>, request_handler: Arc<RequestHandler>) -> Result<Response<Body>, hyper::Error> {
    // Utiliser le RequestHandler pour gérer la requête
    request_handler.handle_request().await
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Charger la configuration
    let config = load_config("config/config.toml")?;

    // Créer les serveurs backends
    let backends: Vec<_> = config.backend_servers.iter()
        .map(|b| {
            BackendServer::new(b.address.clone(), b.port)
        })
        .collect();

    // Initialiser le proxy avec le load balancer approprié
    let load_balancer: Arc<dyn LoadBalancer + Send + Sync> = match config.load_balancer.as_str() {
        "round_robin" => Arc::new(RoundRobinLoadBalancer::new(backends.clone())),
        "weighted_round_robin" => {
            let weighted_backends = backends.iter().map(|b| (b.clone(), 1)).collect();
            Arc::new(WeightedRoundRobinLoadBalancer::new(weighted_backends))
        },
        "least_connections" => {
            let least_connections_backends = backends.iter().map(|b| (b.clone(), 0)).collect();
            Arc::new(LeastConnectionsLoadBalancer::new(least_connections_backends))
        },
        _ => panic!("Unknown load balancer strategy"),
    };

    let request_handler = Arc::new(RequestHandler::new(load_balancer));

    // Créer un service HTTP
    let make_svc = make_service_fn(|_conn| {
        let request_handler = request_handler.clone();
        async move {
            Ok::<_, hyper::Error>(service_fn(move |req| handle_request(req, request_handler.clone())))
        }
    });

    // Configurer le serveur
    let addr = ([127, 0, 0, 1], 3000).into();
    let server = Server::bind(&addr).serve(make_svc);

    println!("Listening on http://{}", addr);

    // Démarrer le serveur
    if let Err(e) = server.await {
        eprintln!("Server error: {}", e);
    }

    Ok(())
}
