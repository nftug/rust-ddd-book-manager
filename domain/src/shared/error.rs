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
