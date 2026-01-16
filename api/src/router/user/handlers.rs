#[allow(unused)]
use application::user::dto::*;
use axum::{Json, extract::State, response::IntoResponse};

use crate::{auth::OidcUserInfo, error::ApiError, registry::AppRegistry};

#[cfg_attr(
    debug_assertions,
        utoipa::path(
            get,
            path = "/users/me",
            responses(
                (status = 200, description = "User details retrieved successfully", body = UserDetailsDTO),
                (status = 401, description = "Unauthorized"),
                (status = 403, description = "Forbidden"),
                (status = 500, description = "Internal server error"),
            ),
            security(
                ("bearerAuth" = [])
            )
    )
)]
#[tracing::instrument(
    skip(registry, user_info),
    fields(
        user_id = %user_info.id,
    ),
    err
)]
pub async fn get_me_details(
    State(registry): State<AppRegistry>,
    user_info: OidcUserInfo,
) -> Result<impl IntoResponse, ApiError> {
    let actor = registry.prepare_actor(user_info).await?;

    let response = registry
        .user_registry()
        .get_user_details(actor.raw_id())
        .await?;

    Ok(Json(response))
}
