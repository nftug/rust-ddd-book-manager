use axum::{Json, extract::State, response::IntoResponse};

use crate::{auth::OidcUserInfo, error::ApiError, registry::AppRegistry};

pub async fn me_handler(
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
