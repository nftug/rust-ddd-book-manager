use axum::Router;
#[cfg(debug_assertions)]
use utoipa::OpenApi;
#[cfg(debug_assertions)]
use utoipa_redoc::{Redoc, Servable};

#[cfg(debug_assertions)]
use crate::openapi::ApiDoc;
use crate::{
    registry::AppRegistry,
    router::{book::book_router, user::user_router},
};

pub mod book;
pub mod user;

pub fn build_router() -> Router<AppRegistry> {
    let api_router = Router::new().merge(user_router()).merge(book_router());

    #[cfg(debug_assertions)]
    let api_router = api_router.merge(Redoc::with_url("/doc", ApiDoc::openapi()));

    Router::new().nest("/api", api_router)
}
