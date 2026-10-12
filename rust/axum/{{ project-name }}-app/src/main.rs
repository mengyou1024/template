//! {{project-name}} — An Axum web application with Sea-ORM.

mod config;
mod error;
mod middleware;
mod routes;
mod state;

use std::net::SocketAddr;

use anyhow::Result;
use sea_orm::{ConnectOptions, Database};
use time::macros::format_description;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{EnvFilter, fmt::time::LocalTime};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::config::AppConfig;
use crate::state::AppState;
use migration::MigratorTrait;

#[tokio::main]
async fn main() -> Result<()> {
    // Load .env file
    dotenvy::dotenv().ok();

    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .with_timer(LocalTime::new(format_description!(
            "[year]-[month]-[day] [hour]:[minute]:[second].[subsecond digits:3]"
        )))
        .pretty()
        .init();

    // Load configuration
    let config = AppConfig::load()?;
    tracing::info!("Configuration loaded");

    // Connect to database with connection pool settings
    let mut opts = ConnectOptions::new(config.database.url.clone());
    opts.max_connections(config.database.max_connections)
        .min_connections(config.database.min_connections)
        .connect_timeout(config.connect_timeout())
        .idle_timeout(config.idle_timeout())
        .sqlx_logging_level(tracing::log::LevelFilter::Debug);

    let db = Database::connect(opts)
        .await
        .expect("Failed to connect to database");
    tracing::info!("Database connected");

    // Run migrations
    migration::Migrator::up(&db, None)
        .await
        .expect("Failed to run migrations");
    tracing::info!("Migrations applied");

    // Build application state
    let state = AppState::new(db, config.clone());

    // Build OpenAPI docs
    let api_doc = routes::ApiDoc::openapi();

    // Build router
    let app = routes::router(state.clone())
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", api_doc))
        .layer(middleware::cors_layer())
        .layer(TraceLayer::new_for_http())
        .layer(middleware::compression_layer());

    // Start server
    let addr: SocketAddr = format!("{}:{}", config.server.host, config.server.port)
        .parse()
        .expect("Invalid address");

    tracing::info!("Server listening on http://{}", addr);
    tracing::info!("Swagger UI available at http://{}/swagger-ui", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
