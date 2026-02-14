use std::net::{Ipv4Addr, SocketAddr};

use api::{
    logger::{build_trace_layer, init_logger},
    registry::AppRegistry,
    router::build_router,
};
use tracing::info;
#[cfg(debug_assertions)]
use {api::router::export_openapi_schema, std::path::Path, tracing::warn};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let registry = AppRegistry::build_runtime().await?;
    let config = registry.config();

    init_logger(&config)?;

    #[cfg(debug_assertions)]
    {
        let repo_root = Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .ok_or("failed to resolve repository root")?;

        if let Err(err) = export_openapi_schema(&config.oidc, repo_root) {
            warn!("Failed to export OpenAPI schema at startup: {}", err);
        }
    }

    let app = build_router(&config.oidc)
        .layer(build_trace_layer())
        .with_state(registry);

    let addr = SocketAddr::from((Ipv4Addr::UNSPECIFIED, config.server.port));

    info!("Starting server at http://{}", addr);
    info!("Environment: {:?}", config.environment);

    axum::serve(tokio::net::TcpListener::bind(addr).await?, app).await?;

    Ok(())
}
