use serde::Deserialize;
use uuid::Uuid;

use crate::auth::OidcAuthError;

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

impl OidcUserInfo {
    pub fn from_claims(claims: KeycloakClaims) -> Result<Self, OidcAuthError> {
        Ok(OidcUserInfo {
            id: claims.sub,
            roles: claims.realm_access.map(|ra| ra.roles).unwrap_or_default(),
            full_name: claims.name,
            email: claims.email,
            username: claims.preferred_username,
        })
    }
}
