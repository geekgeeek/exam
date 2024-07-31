use thiserror::Error;
use std::io;
use serde_json;

// Définir des erreurs personnalisées pour l'application
#[derive(Debug, Error)]
pub enum AppError {
    #[error("Network error: {0}")]
    NetworkError(String),

    #[error("Backend server error: {0}")]
    BackendServerError(String),

    #[error("Configuration error: {0}")]
    ConfigError(String),

    #[error("I/O error: {0}")]
    IoError(#[from] io::Error),

    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),

    #[error("Unknown error occurred")]
    Unknown,
}
