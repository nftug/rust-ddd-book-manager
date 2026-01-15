use axum::{
    Router,
    routing::{get, post},
};

use crate::{registry::AppRegistry, router::book::handlers::*};

pub fn book_router() -> Router<AppRegistry> {
    Router::new().nest(
        "/books",
        Router::new()
            .route("/", get(get_book_list_handler))
            .route("/", post(create_book_handler))
            .route(
                "/{book_id}",
                get(get_book_details_handler)
                    .put(update_book_handler)
                    .delete(delete_book_handler),
            )
            .route(
                "/{book_id}/checkouts",
                post(checkout_book_handler).get(get_checkout_history_handler),
            )
            .route("/{book_id}/return", post(return_book_handler)),
    )
}
