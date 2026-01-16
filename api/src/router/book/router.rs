use axum::{
    Router,
    routing::{get, post},
};

use crate::{registry::AppRegistry, router::book::handlers::*};

pub fn book_router() -> Router<AppRegistry> {
    Router::new().nest(
        "/books",
        Router::new()
            .route("/", get(get_book_list))
            .route("/", post(create_book))
            .route(
                "/{book_id}",
                get(get_book_details).put(update_book).delete(delete_book),
            )
            .route(
                "/{book_id}/checkouts",
                post(checkout_book).get(get_checkout_history),
            )
            .route("/{book_id}/return", post(return_book)),
    )
}
