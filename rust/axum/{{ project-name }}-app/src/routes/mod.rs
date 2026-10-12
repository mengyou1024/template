//! API route definitions.

use axum::{Router, routing::get};
use utoipa::OpenApi;

use crate::state::AppState;

mod health;
mod users;

/// Main API router.
pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/health", get(health::health_check))
        .nest("/api/users", users::router())
        .with_state(state)
}

/// OpenAPI documentation.
#[derive(OpenApi)]
#[openapi(
    paths(
        health::health_check,
        users::list_users,
        users::get_user,
        users::create_user,
        users::update_user,
        users::delete_user,
    ),
    components(schemas(
        health::HealthResponse,
        users::UserResponse,
        users::CreateUserRequest,
        users::UpdateUserRequest,
        users::UserListResponse,
    )),
    tags(
        (name = "health", description = "Health check endpoints"),
        (name = "users", description = "User management endpoints"),
    ),
    info(
        title = "{{project-name}} API",
        version = "0.1.0",
        description = "An Axum web application with Sea-ORM and validator.",
    )
)]
pub struct ApiDoc;
