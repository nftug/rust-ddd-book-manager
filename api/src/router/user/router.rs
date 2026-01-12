use axum::{Router, routing::get};

use crate::registry::AppRegistry;

pub fn user_router() -> Router<AppRegistry> {
    let user_routes = Router::new().route("/me", get(super::handlers::me_handler));

    Router::new().nest("/users", user_routes)
}
