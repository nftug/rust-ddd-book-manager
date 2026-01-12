use axum::Router;

use crate::{registry::AppRegistry, router::user::user_router};

mod user;

pub fn build_router() -> Router<AppRegistry> {
    let routes = Router::new().merge(user_router());

    Router::new().nest("/api", routes)
}
