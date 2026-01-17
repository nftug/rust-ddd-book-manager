use aide::axum::{ApiRouter, routing::get_with};

use crate::{registry::AppRegistry, router::user::handlers::*};

pub fn user_router() -> ApiRouter<AppRegistry> {
    ApiRouter::new().nest(
        "/users",
        ApiRouter::new().api_route(
            "/me",
            get_with(get_me_details, |op| op.tag("Users")),
        ),
    )
}
