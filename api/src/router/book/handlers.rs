use application::book::dto::*;
use axum::{
    Json,
    extract::{Path, Query, State},
    response::IntoResponse,
};

use reqwest::StatusCode;
use uuid::Uuid;

use crate::{auth::OidcUserInfo, error::ApiError, registry::AppRegistry};

#[tracing::instrument(
    skip(registry, user_info),
    fields(
        user_id = ?user_info.as_ref().map(|u| u.id),
    ),
    err
)]
pub async fn get_book_details_handler(
    user_info: Option<OidcUserInfo>,
    State(registry): State<AppRegistry>,
    Path(book_id): Path<Uuid>,
) -> Result<impl IntoResponse, ApiError> {
    let actor = registry.prepare_optional_actor(user_info).await?;

    let response = registry
        .book_registry()
        .get_book_details(actor.as_ref(), book_id)
        .await?;

    Ok(Json(response))
}

#[tracing::instrument(
    skip(registry, user_info),
    fields(
        user_id = ?user_info.as_ref().map(|u| u.id),
    ),
    err
)]
pub async fn get_book_list_handler(
    State(registry): State<AppRegistry>,
    user_info: Option<OidcUserInfo>,
    Query(query): Query<BookListQueryDTO>,
) -> Result<impl IntoResponse, ApiError> {
    let actor = registry.prepare_optional_actor(user_info).await?;

    let response = registry
        .book_registry()
        .get_book_list(actor.as_ref(), query)
        .await?;

    Ok(Json(response))
}

#[tracing::instrument(
    skip(registry, user_info),
    fields(
        user_id = %user_info.id,
    ),
    err
)]
pub async fn create_book_handler(
    State(registry): State<AppRegistry>,
    user_info: OidcUserInfo,
    Json(request): Json<CreateBookRequestDTO>,
) -> Result<impl IntoResponse, ApiError> {
    let actor = registry.prepare_actor(user_info).await?;

    let response = registry
        .book_registry()
        .create_book(&actor, request)
        .await?;

    Ok((StatusCode::CREATED, Json(response)))
}

#[tracing::instrument(
    skip(registry, user_info),
    fields(
        user_id = %user_info.id,
    ),
    err
)]
pub async fn update_book_handler(
    State(registry): State<AppRegistry>,
    user_info: OidcUserInfo,
    Path(book_id): Path<Uuid>,
    Json(request): Json<UpdateBookRequestDTO>,
) -> Result<impl IntoResponse, ApiError> {
    let actor = registry.prepare_actor(user_info).await?;

    registry
        .book_registry()
        .update_book(&actor, book_id, request)
        .await?;

    Ok(StatusCode::NO_CONTENT)
}

#[tracing::instrument(
    skip(registry, user_info),
    fields(
        user_id = %user_info.id,
    ),
    err
)]
pub async fn delete_book_handler(
    State(registry): State<AppRegistry>,
    user_info: OidcUserInfo,
    Path(book_id): Path<Uuid>,
) -> Result<impl IntoResponse, ApiError> {
    let actor = registry.prepare_actor(user_info).await?;

    registry
        .book_registry()
        .delete_book(&actor, book_id)
        .await?;

    Ok(StatusCode::NO_CONTENT)
}

#[tracing::instrument(
    skip(registry, user_info),
    fields(
        user_id = %user_info.id,
    ),
    err
)]
pub async fn get_checkout_history_handler(
    State(registry): State<AppRegistry>,
    Path(book_id): Path<Uuid>,
    Query(query): Query<CheckoutHistoryQueryDTO>,
    user_info: OidcUserInfo,
) -> Result<impl IntoResponse, ApiError> {
    let actor = registry.prepare_actor(user_info).await?;

    let response = registry
        .book_registry()
        .get_checkout_history(&actor, book_id, query)
        .await?;

    Ok(Json(response))
}

#[tracing::instrument(
    skip(registry, user_info),
    fields(
        user_id = %user_info.id,
    ),
    err
)]
pub async fn checkout_book_handler(
    State(registry): State<AppRegistry>,
    user_info: OidcUserInfo,
    Path(book_id): Path<Uuid>,
) -> Result<impl IntoResponse, ApiError> {
    let actor = registry.prepare_actor(user_info).await?;

    registry
        .book_registry()
        .checkout_book(&actor, book_id)
        .await?;

    Ok(StatusCode::NO_CONTENT)
}

#[tracing::instrument(
    skip(registry, user_info),
    fields(
        user_id = %user_info.id,
    ),
    err
)]
pub async fn return_book_handler(
    State(registry): State<AppRegistry>,
    user_info: OidcUserInfo,
    Path(book_id): Path<Uuid>,
) -> Result<impl IntoResponse, ApiError> {
    let actor = registry.prepare_actor(user_info).await?;

    registry
        .book_registry()
        .return_book(&actor, book_id)
        .await?;

    Ok(StatusCode::NO_CONTENT)
}
