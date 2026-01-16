use application::book::dto::*;
#[allow(unused)]
use application::shared::*;
use axum::{
    Json,
    extract::{Path, Query, State},
    response::IntoResponse,
};

use reqwest::StatusCode;
use uuid::Uuid;

use crate::{auth::OidcUserInfo, error::ApiError, registry::AppRegistry};

#[cfg_attr(
    debug_assertions,
    utoipa::path(
        get,
        path = "/books/{book_id}",
        params(
            ("book_id" = Uuid, Path, description = "The UUID of the book to retrieve"),
        ),
        responses(
            (status = 200, description = "Book details retrieved successfully", body = BookDetailsDTO),
            (status = 401, description = "Unauthorized"),
            (status = 403, description = "Forbidden"),
            (status = 404, description = "Book not found"),
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
        user_id = ?user_info.as_ref().map(|u| u.id),
    ),
    err
)]
pub async fn get_book_details(
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

#[cfg_attr(
    debug_assertions,
    utoipa::path(
        get,
        path = "/books",
        params(
            ("owner_id" = Option<Uuid>, Query, description = "Filter books by owner UUID"),
            ("checked_out" = Option<bool>, Query, description = "Filter books by checked out status"),
            ("page" = Option<u32>, Query, description = "Page number for pagination"),
            ("page_size" = Option<u32>, Query, description = "Number of items per page for pagination"),
        ),
        responses(
            (status = 200, description = "Book list retrieved successfully", body = PaginationDTO<BookListItemDTO>),
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
        user_id = ?user_info.as_ref().map(|u| u.id),
    ),
    err
)]
pub async fn get_book_list(
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

#[cfg_attr(
    debug_assertions,
    utoipa::path(
        post,
        path = "/books",
        request_body = CreateBookRequestDTO,
        responses(
            (status = 201, description = "Book created successfully", body = EntityCreationDTO),
            (status = 400, description = "Invalid request"),
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
pub async fn create_book(
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

#[cfg_attr(
    debug_assertions,
    utoipa::path(
        put,
        path = "/books/{book_id}",
        params(
            ("book_id" = Uuid, Path, description = "The UUID of the book to update"),
        ),
        request_body = UpdateBookRequestDTO,
        responses(
            (status = 204, description = "Book updated successfully"),
            (status = 400, description = "Invalid request"),
            (status = 401, description = "Unauthorized"),
            (status = 403, description = "Forbidden"),
            (status = 404, description = "Book not found"),
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
pub async fn update_book(
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

#[cfg_attr(
    debug_assertions,
    utoipa::path(
        put,
        path = "/books/{book_id}",
        params(
            ("book_id" = Uuid, Path, description = "The UUID of the book to update"),
        ),
        request_body = UpdateBookRequestDTO,
        responses(
            (status = 204, description = "Book updated successfully"),
            (status = 400, description = "Invalid request"),
            (status = 401, description = "Unauthorized"),
            (status = 403, description = "Forbidden"),
            (status = 404, description = "Book not found"),
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
pub async fn delete_book(
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

#[cfg_attr(
    debug_assertions,
    utoipa::path(
        get,
        path = "/books/{book_id}/checkouts",
        params(
            ("book_id" = Uuid, Path, description = "The UUID of the book to retrieve checkout history for"),
            ("page" = Option<u32>, Query, description = "Page number for pagination"),
            ("page_size" = Option<u32>, Query, description = "Number of items per page for pagination"),
        ),
        responses(
            (status = 200, description = "Checkout history retrieved successfully", body = CheckoutHistoryListDTO),
            (status = 401, description = "Unauthorized"),
            (status = 403, description = "Forbidden"),
            (status = 404, description = "Book not found"),
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
pub async fn get_checkout_history(
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

#[cfg_attr(
    debug_assertions,
    utoipa::path(
        post,
        path = "/books/{book_id}/checkout",
        params(
            ("book_id" = Uuid, Path, description = "The UUID of the book to checkout"),
        ),
        responses(
            (status = 204, description = "Book checked out successfully"),
            (status = 400, description = "Invalid request"),
            (status = 401, description = "Unauthorized"),
            (status = 403, description = "Forbidden"),
            (status = 404, description = "Book not found"),
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
pub async fn checkout_book(
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

#[cfg_attr(
    debug_assertions,
    utoipa::path(
        post,
        path = "/books/{book_id}/return",
        params(
            ("book_id" = Uuid, Path, description = "The UUID of the book to return"),
        ),
        responses(
            (status = 204, description = "Book returned successfully"),
            (status = 400, description = "Invalid request"),
            (status = 401, description = "Unauthorized"),
            (status = 403, description = "Forbidden"),
            (status = 404, description = "Book not found"),
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
pub async fn return_book(
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
