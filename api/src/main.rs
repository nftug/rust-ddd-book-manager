use std::net::{Ipv4Addr, SocketAddr};

use api::{
    logging::{build_trace_layer, init_logger},
    registry::AppRegistry,
    router::build_router,
};
use tracing::info;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let registry = AppRegistry::build_runtime().await?;
    let config = registry.config();

    init_logger(&config)?;

    let app = build_router()
        .layer(build_trace_layer())
        .with_state(registry);

    let addr = SocketAddr::from((Ipv4Addr::UNSPECIFIED, config.server.port));

    info!("Starting server at http://{}", addr);
    info!("Environment: {:?}", config.environment);

    axum::serve(tokio::net::TcpListener::bind(addr).await?, app).await?;

    Ok(())
}
