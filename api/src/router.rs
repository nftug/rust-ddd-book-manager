use axum::Router;

use crate::{
    registry::AppRegistry,
    router::{book::book_router, user::user_router},
};

mod book;
mod user;

pub fn build_router() -> Router<AppRegistry> {
    Router::new().nest(
        "/api",
        Router::new().merge(user_router()).merge(book_router()),
    )
}
