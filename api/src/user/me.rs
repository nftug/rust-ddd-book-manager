use std::str::FromStr;

use axum::{Json, extract::State, response::IntoResponse};
use uuid::Uuid;

use application::user::dto::GetOrCreateUserRequestDTO;
use domain::{shared::id::Id as DomainId, user::enums::UserRole};

use crate::{
    auth::{OidcAuthError, OidcUserInfo},
    registry::AppRegistry,
};

pub fn me_router() -> axum::Router<AppRegistry> {
    axum::Router::new().route("/me", axum::routing::get(me_handler))
}

#[derive(serde::Serialize)]
pub struct MeResponse {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub role: UserRole,
}

fn map_roles_to_user_role(roles: &[String]) -> UserRole {
    if roles
        .iter()
        .any(|r| UserRole::from_str(r).is_ok_and(|r| r == UserRole::Admin))
    {
        UserRole::Admin
    } else {
        UserRole::Regular
    }
}

pub async fn me_handler(
    State(registry): State<AppRegistry>,
    user_info: OidcUserInfo,
) -> Result<impl IntoResponse, OidcAuthError> {
    let full_name = user_info
        .full_name
        .clone()
        .ok_or(OidcAuthError::InvalidToken("missing full name".to_string()))?;

    let email = user_info
        .email
        .clone()
        .ok_or(OidcAuthError::InvalidToken("missing email".to_string()))?;

    let role = map_roles_to_user_role(&user_info.roles);

    let dto = GetOrCreateUserRequestDTO::new(user_info.id, full_name, email.clone(), role.clone());

    let actor = registry
        .get_or_create_user()
        .execute(dto)
        .await
        .map_err(|_| OidcAuthError::InvalidToken("failed to get or create user".to_string()))?;

    let response = MeResponse {
        id: actor.id().raw(),
        name: actor.username().to_string(),
        email,
        role,
    };

    Ok(Json(response))
}
