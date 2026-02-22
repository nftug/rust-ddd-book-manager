use application::{book::dto::*, shared::EntityCreationDTO};
use axum::{
    Json,
    extract::{Path, Query, State},
    response::NoContent,
};

use reqwest::StatusCode;

use crate::{auth::OidcUserInfo, error::ApiError, registry::AppRegistry};

#[tracing::instrument(
    skip(registry, user_info),
    fields(user_id = ?user_info.as_ref().map(|u| u.id)),
    err
)]
pub async fn get_book_details(
    user_info: Option<OidcUserInfo>,
    State(registry): State<AppRegistry>,
    Path(identity): Path<BookIdentity>,
) -> Result<Json<BookDetailsDTO>, ApiError> {
    let actor = registry.prepare_optional_actor(user_info.as_ref()).await?;

    let response = registry
        .book_registry()
        .get_book_details()
        .execute(actor.as_ref(), identity)
        .await?;

    Ok(Json(response))
}

#[tracing::instrument(
    skip(registry, user_info),
    fields(user_id = ?user_info.as_ref().map(|u| u.id)),
    err
)]
pub async fn get_book_list(
    user_info: Option<OidcUserInfo>,
    State(registry): State<AppRegistry>,
    Query(query): Query<BookListQueryDTO>,
) -> Result<Json<BookListResponseDTO>, ApiError> {
    let actor = registry.prepare_optional_actor(user_info.as_ref()).await?;

    let response = registry
        .book_registry()
        .get_book_list()
        .execute(actor.as_ref(), &query)
        .await?;

    Ok(Json(response))
}

#[tracing::instrument(
    skip(registry, user_info),
    fields(user_id = %user_info.id),
    err
)]
pub async fn create_book(
    user_info: OidcUserInfo,
    State(registry): State<AppRegistry>,
    Json(request): Json<CreateBookRequestDTO>,
) -> Result<(StatusCode, Json<EntityCreationDTO>), ApiError> {
    let actor = registry.prepare_actor(&user_info).await?;

    let response = registry
        .book_registry()
        .create_book()
        .execute(&actor, &request)
        .await?;

    Ok((StatusCode::CREATED, Json(response)))
}

#[tracing::instrument(
    skip(registry, user_info),
    fields(user_id = %user_info.id),
    err
)]
pub async fn update_book(
    user_info: OidcUserInfo,
    State(registry): State<AppRegistry>,
    Path(identity): Path<BookIdentity>,
    Json(request): Json<UpdateBookRequestDTO>,
) -> Result<NoContent, ApiError> {
    let actor = registry.prepare_actor(&user_info).await?;

    registry
        .book_registry()
        .update_book()
        .execute(&actor, identity, &request)
        .await?;

    Ok(NoContent)
}

#[tracing::instrument(
    skip(registry, user_info),
    fields(user_id = %user_info.id),
    err
)]
pub async fn delete_book(
    user_info: OidcUserInfo,
    State(registry): State<AppRegistry>,
    Path(identity): Path<BookIdentity>,
) -> Result<NoContent, ApiError> {
    let actor = registry.prepare_actor(&user_info).await?;

    registry
        .book_registry()
        .delete_book()
        .execute(&actor, identity)
        .await?;

    Ok(NoContent)
}

#[tracing::instrument(
    skip(registry, user_info),
    fields(user_id = %user_info.id),
    err
)]
pub async fn get_checkout_history(
    user_info: OidcUserInfo,
    State(registry): State<AppRegistry>,
    Path(identity): Path<BookIdentity>,
    Query(query): Query<CheckoutHistoryQueryDTO>,
) -> Result<Json<CheckoutHistoryListDTO>, ApiError> {
    let actor = registry.prepare_actor(&user_info).await?;

    let response = registry
        .book_registry()
        .get_checkout_history()
        .execute(&actor, identity, &query)
        .await?;

    Ok(Json(response))
}

#[tracing::instrument(
    skip(registry, user_info),
    fields(user_id = %user_info.id),
    err
)]
pub async fn checkout_book(
    user_info: OidcUserInfo,
    State(registry): State<AppRegistry>,
    Path(identity): Path<BookIdentity>,
) -> Result<NoContent, ApiError> {
    let actor = registry.prepare_actor(&user_info).await?;

    registry
        .book_registry()
        .checkout_book()
        .execute(&actor, identity)
        .await?;

    Ok(NoContent)
}

#[tracing::instrument(
    skip(registry, user_info),
    fields(user_id = %user_info.id),
    err
)]
pub async fn return_book(
    user_info: OidcUserInfo,
    State(registry): State<AppRegistry>,
    Path(identity): Path<BookIdentity>,
) -> Result<NoContent, ApiError> {
    let actor = registry.prepare_actor(&user_info).await?;

    registry
        .book_registry()
        .return_book()
        .execute(&actor, identity)
        .await?;

    Ok(NoContent)
}
