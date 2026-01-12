use application::shared::error::ApplicationError;
use axum::response::IntoResponse;
use domain::shared::error::DomainError;
use reqwest::StatusCode;
use thiserror::Error;

use crate::auth::OidcAuthError;

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("Bad request: {0}")]
    BadRequest(String),
    #[error("Not found")]
    NotFound,
    #[error("Forbidden")]
    Forbidden,
    #[error("Unauthorized")]
    Unauthorized,
    #[error("Internal server error")]
    InternalError,
}

impl From<ApplicationError> for ApiError {
    fn from(err: ApplicationError) -> Self {
        match err {
            ApplicationError::DomainError(domain_err) => match domain_err {
                DomainError::NotFound => ApiError::NotFound,
                DomainError::Forbidden => ApiError::Forbidden,
                DomainError::ValidationError(msg) => ApiError::BadRequest(msg),
            },
            ApplicationError::PersistenceError(_) => ApiError::InternalError,
            ApplicationError::NotFound => ApiError::NotFound,
            ApplicationError::Forbidden => ApiError::Forbidden,
            ApplicationError::InternalError => ApiError::InternalError,
        }
    }
}

impl From<OidcAuthError> for ApiError {
    fn from(err: OidcAuthError) -> Self {
        match err {
            OidcAuthError::MissingToken
            | OidcAuthError::InvalidToken(_)
            | OidcAuthError::Expired => ApiError::Unauthorized,
            OidcAuthError::JwksFetchError => ApiError::InternalError,
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        let (status, message) = match &self {
            ApiError::BadRequest(msg) => (StatusCode::BAD_REQUEST, Some(msg.clone())),
            ApiError::NotFound => (StatusCode::NOT_FOUND, None),
            ApiError::Forbidden => (StatusCode::FORBIDDEN, None),
            ApiError::Unauthorized => (StatusCode::UNAUTHORIZED, None),
            ApiError::InternalError => (StatusCode::INTERNAL_SERVER_ERROR, None),
        };

        if let Some(message) = message {
            let body = axum::Json(serde_json::json!({ "error": message }));
            (status, body).into_response()
        } else {
            status.into_response()
        }
    }
}
