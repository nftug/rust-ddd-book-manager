use thiserror::Error;

#[derive(Error, Debug)]
pub enum DomainError {
    #[error("{0}")]
    ValidationError(String),
    #[error("Not found")]
    NotFound,
    #[error("Forbidden")]
    Forbidden,
}

#[derive(Error, Debug)]
pub enum PersistenceError {
    #[error("Database operation failed")]
    OperationError,
    #[error("Transaction error")]
    TransactionError,
    #[error("Entity not found")]
    NotFound,
    #[error("{0}")]
    EntityConversionError(String),
}
