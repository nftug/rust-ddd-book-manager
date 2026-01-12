use thiserror::Error;

#[derive(Debug, Error)]
pub enum OidcAuthError {
    #[error("Missing authorization token")]
    MissingToken,
    #[error("Invalid token: {0}")]
    InvalidToken(String),
    #[error("Expired token")]
    Expired,
    #[error("JWKS fetch error")]
    JwksFetchError,
}
