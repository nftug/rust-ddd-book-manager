use axum::Router;

use crate::{
    registry::AppRegistry,
    router::{book::book_router, user::user_router},
};

mod book;
mod user;

pub fn build_router() -> Router<AppRegistry> {
    let routes = Router::new().merge(user_router()).merge(book_router());

    Router::new().nest("/api", routes)
}
