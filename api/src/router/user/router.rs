use axum::{Router, routing::get};

use crate::{registry::AppRegistry, router::user::handlers::*};

pub fn user_router() -> Router<AppRegistry> {
    Router::new().nest("/users", Router::new().route("/me", get(me_handler)))
}
