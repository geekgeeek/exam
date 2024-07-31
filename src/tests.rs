#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use crate::backend::BackendServer;
    use crate::config::{load_config, Config};
    use crate::load_balancer::{RoundRobinLoadBalancer, LoadBalancer};

    // Tests pour le module backend
    mod backend_tests {
        use super::*;

        #[test]
        fn test_backend_creation() {
            let backend = BackendServer::new("127.0.0.1".to_string(), 8080);
            assert_eq!(backend.address(), "127.0.0.1");
            assert_eq!(backend.port(), 8080);
        }
    }

    // Tests pour le module config
    mod config_tests {
        use super::*;
        use toml;

        #[test]
        fn test_load_config() {
            let config_str = r#"
                [[backends]]
                address = "127.0.0.1"
                port = 8080

                [[backends]]
                address = "127.0.0.2"
                port = 8081

                health_check_interval = 10
            "#;
            let config: Config = toml::from_str(config_str).expect("Failed to parse config");
            assert_eq!(config.backends.len(), 2);
            assert_eq!(config.backends[0].address, "127.0.0.1");
            assert_eq!(config.backends[0].port, 8080);
            assert_eq!(config.backends[1].address, "127.0.0.2");
            assert_eq!(config.backends[1].port, 8081);
            assert_eq!(config.health_check_interval, 10);
        }
    }

    // Tests pour le module load_balancer
    mod load_balancer_tests {
        use super::*;

        #[test]
        fn test_round_robin_load_balancer() {
            let backends = vec![
                Arc::new(BackendServer::new("127.0.0.1".to_string(), 8080)),
                Arc::new(BackendServer::new("127.0.0.2".to_string(), 8081)),
            ];
            let lb = RoundRobinLoadBalancer::new(backends.clone());

            // Test first backend
            let backend1 = lb.next_backend();
            assert_eq!(backend1.address(), "127.0.0.1");
            assert_eq!(backend1.port(), 8080);

            // Test second backend
            let backend2 = lb.next_backend();
            assert_eq!(backend2.address(), "127.0.0.2");
            assert_eq!(backend2.port(), 8081);

            // Test back to the first backend
            let backend1_again = lb.next_backend();
            assert_eq!(backend1_again.address(), "127.0.0.1");
            assert_eq!(backend1_again.port(), 8080);
        }
    }
}
