use axum::{Json, extract::State};

use application::user::dto::UserDetailsDTO;

use crate::{auth::OidcUserInfo, error::ApiError, registry::AppRegistry};

#[tracing::instrument(
    skip(registry, user_info),
    fields(
        user_id = %user_info.id,
    ),
    err
)]
pub async fn get_me_details(
    user_info: OidcUserInfo,
    State(registry): State<AppRegistry>,
) -> Result<Json<UserDetailsDTO>, ApiError> {
    let actor = registry.prepare_actor(&user_info).await?;

    let response = registry
        .user_registry()
        .get_user_details()
        .execute(actor.id())
        .await?;

    Ok(Json(response))
}
