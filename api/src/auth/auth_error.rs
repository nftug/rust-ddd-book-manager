use axum::response::IntoResponse;
use reqwest::StatusCode;
use serde_json::json;
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

impl IntoResponse for OidcAuthError {
    fn into_response(self) -> axum::response::Response {
        let status = match &self {
            OidcAuthError::JwksFetchError => StatusCode::SERVICE_UNAVAILABLE,
            _ => StatusCode::UNAUTHORIZED,
        };
        let body = axum::Json(json!({ "error": self.to_string() }));
        (status, body).into_response()
    }
}
