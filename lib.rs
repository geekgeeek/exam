// src/lib.rs

pub mod backend;
pub mod client;
pub mod config;
pub mod load_balancer;
pub mod request_handler;
pub mod health;
pub mod error;
pub use backend::BackendServer;
pub use client::Client;
pub use config::Config;
pub use load_balancer::{LoadBalancer, RoundRobinLoadBalancer, WeightedRoundRobinLoadBalancer, LeastConnectionsLoadBalancer};
pub use request_handler::RequestHandler;
pub use health::HealthChecker;
pub use error::AppError;
