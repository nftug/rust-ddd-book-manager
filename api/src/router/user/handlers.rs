use axum::{Json, extract::State, response::IntoResponse};

use domain::shared::id::Id as DomainId;

use crate::{auth::OidcUserInfo, error::ApiError, registry::AppRegistry};

pub async fn me_handler(
    State(registry): State<AppRegistry>,
    user_info: OidcUserInfo,
) -> Result<impl IntoResponse, ApiError> {
    let actor = registry
        .user_registry()
        .get_or_create_actor(user_info.try_into()?)
        .await?;

    let response = registry
        .user_registry()
        .get_user_details(actor.id().raw())
        .await?;

    Ok(Json(response))
}
