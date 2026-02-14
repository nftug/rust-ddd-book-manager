use crate::{
    registry::AppRegistry,
    router::{book::book_router, user::user_router},
};
use aide::axum::ApiRouter;
use axum::Router;
use infrastructure::config::OidcConfig;
#[cfg(debug_assertions)]
use {
    aide::{openapi::*, redoc::Redoc},
    axum::{Extension, http::header::CONTENT_TYPE, response::IntoResponse, routing::get},
    std::{fs, path::Path},
    tracing::info,
};

pub mod book;
pub mod user;

#[cfg(debug_assertions)]
async fn serve_api(Extension(api): Extension<OpenApi>) -> impl IntoResponse {
    let body = serde_json::to_string_pretty(&api).unwrap_or_else(|_| "{}".to_string());
    ([(CONTENT_TYPE, "application/json")], body)
}

#[cfg(debug_assertions)]
fn build_openapi_schema(oidc_config: &OidcConfig) -> OpenApi {
    let tags = vec![
        Tag {
            name: "Books".to_string(),
            description: Some("Book management endpoints".to_string()),
            ..Tag::default()
        },
        Tag {
            name: "Users".to_string(),
            description: Some("User management endpoints".to_string()),
            ..Tag::default()
        },
    ];

    let mut components = Components::default();
    components.security_schemes.insert(
        "bearerAuth".to_string(),
        ReferenceOr::Item(SecurityScheme::OpenIdConnect {
            open_id_connect_url: format!(
                "{}/.well-known/openid-configuration",
                oidc_config.authority.trim_end_matches('/')
            ),
            description: Some("OpenID Connect discovery endpoint".to_string()),
            extensions: Default::default(),
        }),
    );

    let mut security = SecurityRequirement::default();
    security.insert("bearerAuth".to_string(), Vec::new());

    OpenApi {
        info: Info {
            title: "Book Manager API".to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            ..Info::default()
        },
        tags,
        components: Some(components),
        security: vec![security],
        ..OpenApi::default()
    }
}

#[allow(unused)]
pub fn build_router(oidc_config: &OidcConfig) -> Router<AppRegistry> {
    let api_router = build_api_router();

    #[cfg(debug_assertions)]
    {
        let mut api = build_openapi_schema(oidc_config);
        let api_router = api_router.nest(
            "/api",
            ApiRouter::new()
                .route("/doc", Redoc::new("/api/doc/openapi.json").axum_route())
                .route("/doc/openapi.json", get(serve_api)),
        );

        api_router.finish_api(&mut api).layer(Extension(api))
    }

    #[cfg(not(debug_assertions))]
    {
        Router::from(api_router)
    }
}

#[cfg(debug_assertions)]
pub fn export_openapi_schema(
    oidc_config: &OidcConfig,
    target_dir_path: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    let api_router = build_api_router();
    let mut schema = build_openapi_schema(oidc_config);
    let _ = api_router.finish_api(&mut schema);

    let body = serde_json::to_string_pretty(&schema)?;

    let output_path = target_dir_path.join("openapi.json");

    if let Ok(existing) = fs::read_to_string(&output_path)
        && existing == body
    {
        info!(
            "OpenAPI schema is unchanged; skip writing {}",
            output_path.display()
        );
        return Ok(());
    }

    fs::write(&output_path, body)?;
    info!("Exported OpenAPI schema to {}", output_path.display());

    Ok(())
}

fn build_api_router() -> ApiRouter<AppRegistry> {
    ApiRouter::new().nest("/api", book_router().merge(user_router()))
}
