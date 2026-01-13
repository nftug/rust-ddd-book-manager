use std::str::FromStr;

use application::user::dto::GetOrCreateUserRequestDTO;
use domain::user::enums::UserRole;
use serde::Deserialize;
use uuid::Uuid;

use crate::error::ApiError;

#[derive(Debug, Deserialize)]
pub struct KeycloakRealmAccess {
    pub roles: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct KeycloakClaims {
    pub sub: Uuid,
    #[serde(default)]
    pub preferred_username: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub email: Option<String>,
    #[serde(default)]
    pub realm_access: Option<KeycloakRealmAccess>,
    pub exp: usize,
}

#[derive(Debug, Clone)]
pub struct OidcUserInfo {
    pub id: Uuid,
    pub roles: Vec<String>,
    pub full_name: Option<String>,
    pub email: Option<String>,
    pub username: Option<String>,
}

impl From<KeycloakClaims> for OidcUserInfo {
    fn from(claims: KeycloakClaims) -> Self {
        OidcUserInfo {
            id: claims.sub,
            roles: claims.realm_access.map(|ra| ra.roles).unwrap_or_default(),
            full_name: claims.name,
            email: claims.email,
            username: claims.preferred_username,
        }
    }
}

impl TryFrom<OidcUserInfo> for GetOrCreateUserRequestDTO {
    type Error = ApiError;

    fn try_from(user_info: OidcUserInfo) -> Result<Self, Self::Error> {
        let full_name = user_info
            .full_name
            .ok_or(ApiError::BadRequest("missing full name".to_string()))?;

        let email = user_info
            .email
            .ok_or(ApiError::BadRequest("missing email".to_string()))?;
        let role = if user_info
            .roles
            .iter()
            .any(|v| UserRole::from_str(v).is_ok_and(|r| r == UserRole::Admin))
        {
            domain::user::enums::UserRole::Admin
        } else {
            domain::user::enums::UserRole::Regular
        };

        Ok(GetOrCreateUserRequestDTO::new(
            user_info.id,
            full_name,
            email,
            role,
        ))
    }
}
