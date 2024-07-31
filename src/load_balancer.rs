use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicUsize, Ordering};
use crate::backend::BackendServer;

pub trait LoadBalancer {
    fn select_backend(&self) -> Arc<BackendServer>;
}

pub struct RoundRobinLoadBalancer {
    backends: Vec<Arc<BackendServer>>,
    current: AtomicUsize,
}

impl RoundRobinLoadBalancer {
    pub fn new(backends: Vec<Arc<BackendServer>>) -> Self {
        Self {
            backends,
            current: AtomicUsize::new(0),
        }
    }
}

impl LoadBalancer for RoundRobinLoadBalancer {
    fn select_backend(&self) -> Arc<BackendServer> {
        let index = self.current.fetch_add(1, Ordering::SeqCst) % self.backends.len();
        self.backends[index].clone()
    }
}

pub struct WeightedRoundRobinLoadBalancer {
    backends: Arc<Mutex<Vec<(Arc<BackendServer>, u32)>>>,
    current: AtomicUsize,
}

impl WeightedRoundRobinLoadBalancer {
    pub fn new(backends: Vec<(Arc<BackendServer>, u32)>) -> Self {
        Self {
            backends: Arc::new(Mutex::new(backends)),
            current: AtomicUsize::new(0),
        }
    }
}

impl LoadBalancer for WeightedRoundRobinLoadBalancer {
    fn select_backend(&self) -> Arc<BackendServer> {
        let backends = self.backends.lock().unwrap();
        let index = self.current.fetch_add(1, Ordering::SeqCst) % backends.len();
        backends[index].0.clone()
    }
}

pub struct LeastConnectionsLoadBalancer {
    backends: Arc<Mutex<Vec<(Arc<BackendServer>, usize)>>>,
}

impl LeastConnectionsLoadBalancer {
    pub fn new(backends: Vec<(Arc<BackendServer>, usize)>) -> Self {
        Self {
            backends: Arc::new(Mutex::new(backends)),
        }
    }
}

impl LoadBalancer for LeastConnectionsLoadBalancer {
    fn select_backend(&self) -> Arc<BackendServer> {
        let mut backends = self.backends.lock().unwrap();
        let (backend, connections) = backends.iter_mut().min_by_key(|(_, connections)| *connections).unwrap();
        *connections += 1;
        backend.clone()
    }
}
