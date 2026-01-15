use domain::shared::error::{DomainError, PersistenceError};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ApplicationError {
    #[error("{0}")]
    DomainError(#[from] DomainError),
    #[error("Persistence error: {0}")]
    PersistenceError(#[from] PersistenceError),
    #[error("Not found")]
    NotFound,
    #[error("Forbidden")]
    Forbidden,
    #[error("Internal server error: {0}")]
    InternalError(String),
}
