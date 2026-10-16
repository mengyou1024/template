//! User management routes.

use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    routing::get,
};
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter, Set};
use serde::Deserialize;
use utoipa::IntoParams;
use uuid::Uuid;
use validator::Validate;

use crate::error::AppResult;
use crate::state::AppState;
use entity::prelude::*;
use entity::user;

mod schemas;
pub use schemas::*;

/// User routes.
pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(list_users).post(create_user))
        .route("/{id}", get(get_user).put(update_user).delete(delete_user))
}

/// Query parameters for listing users.
#[derive(Debug, Deserialize, IntoParams)]
pub struct ListUserParams {
    /// Page number (1-based)
    #[param(example = 1)]
    pub page: Option<u64>,
    /// Items per page
    #[param(example = 20)]
    pub page_size: Option<u64>,
}

/// List all users with pagination.
#[utoipa::path(
    get,
    path = "/api/users",
    tag = "users",
    params(ListUserParams),
    responses(
        (status = 200, description = "List of users", body = UserListResponse),
    ),
)]
pub async fn list_users(
    State(state): State<AppState>,
    axum::extract::Query(params): axum::extract::Query<ListUserParams>,
) -> AppResult<Json<UserListResponse>> {
    let page = params.page.unwrap_or(1);
    let page_size = params.page_size.unwrap_or(20);

    let paginator = User::find().paginate(&state.db, page_size);
    let total = paginator.num_items().await?;
    let total_pages = paginator.num_pages().await?;
    let items: Vec<UserModel> = paginator.fetch_page(page.saturating_sub(1)).await?;

    Ok(Json(UserListResponse {
        items: items.into_iter().map(UserResponse::from).collect(),
        total,
        page,
        page_size,
        total_pages,
    }))
}

/// Get a single user by ID.
#[utoipa::path(
    get,
    path = "/api/users/{id}",
    tag = "users",
    params(
        ("id" = Uuid, Path, description = "User ID"),
    ),
    responses(
        (status = 200, description = "User found", body = UserResponse),
        (status = 404, description = "User not found"),
    ),
)]
pub async fn get_user(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> AppResult<Json<UserResponse>> {
    let user = User::find_by_id(id)
        .one(&state.db)
        .await?
        .ok_or(crate::error::AppError::NotFound)?;

    Ok(Json(UserResponse::from(user)))
}

/// Create a new user.
#[utoipa::path(
    post,
    path = "/api/users",
    tag = "users",
    request_body = CreateUserRequest,
    responses(
        (status = 201, description = "User created", body = UserResponse),
        (status = 400, description = "Validation error"),
        (status = 409, description = "Username or email already exists"),
    ),
)]
pub async fn create_user(
    State(state): State<AppState>,
    Json(body): Json<CreateUserRequest>,
) -> AppResult<Json<UserResponse>> {
    // Validate input
    body.validate()?;

    // Check for existing username or email
    let existing = User::find()
        .filter(UserColumn::Username.eq(&body.username))
        .one(&state.db)
        .await?;
    if existing.is_some() {
        return Err(crate::error::AppError::Conflict(
            "Username already exists".into(),
        ));
    }

    let existing = User::find()
        .filter(UserColumn::Email.eq(&body.email))
        .one(&state.db)
        .await?;
    if existing.is_some() {
        return Err(crate::error::AppError::Conflict(
            "Email already exists".into(),
        ));
    }

    // Hash password
    let password_hash = hash_password(&body.password)?;

    let user = user::ActiveModel {
        id: Set(Uuid::new_v4()),
        username: Set(body.username.clone()),
        email: Set(body.email.clone()),
        password_hash: Set(password_hash),
        created_at: Set(chrono::Utc::now().into()),
        updated_at: Set(chrono::Utc::now().into()),
    };

    let user = user.insert(&state.db).await?;

    Ok(Json(UserResponse::from(user)))
}

/// Update an existing user.
#[utoipa::path(
    put,
    path = "/api/users/{id}",
    tag = "users",
    params(
        ("id" = Uuid, Path, description = "User ID"),
    ),
    request_body = UpdateUserRequest,
    responses(
        (status = 200, description = "User updated", body = UserResponse),
        (status = 400, description = "Validation error"),
        (status = 404, description = "User not found"),
    ),
)]
pub async fn update_user(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(body): Json<UpdateUserRequest>,
) -> AppResult<Json<UserResponse>> {
    // Validate input
    body.validate()?;

    let user = User::find_by_id(id)
        .one(&state.db)
        .await?
        .ok_or(crate::error::AppError::NotFound)?;

    let mut active: user::ActiveModel = user.into();

    if let Some(username) = body.username {
        active.username = Set(username);
    }
    if let Some(email) = body.email {
        active.email = Set(email);
    }
    if let Some(password) = body.password {
        let password_hash = hash_password(&password)?;
        active.password_hash = Set(password_hash);
    }

    active.updated_at = Set(chrono::Utc::now().into());
    let user = active.update(&state.db).await?;

    Ok(Json(UserResponse::from(user)))
}

/// Delete a user.
#[utoipa::path(
    delete,
    path = "/api/users/{id}",
    tag = "users",
    params(
        ("id" = Uuid, Path, description = "User ID"),
    ),
    responses(
        (status = 204, description = "User deleted"),
        (status = 404, description = "User not found"),
    ),
)]
pub async fn delete_user(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> AppResult<StatusCode> {
    let user = User::find_by_id(id)
        .one(&state.db)
        .await?
        .ok_or(crate::error::AppError::NotFound)?;

    let active: user::ActiveModel = user.into();
    active.delete(&state.db).await?;

    Ok(StatusCode::NO_CONTENT)
}

fn hash_password(password: &str) -> Result<String, crate::error::AppError> {
    use argon2::{Algorithm, Argon2, Params, Version};
    use base64ct::{Base64Unpadded, Encoding};
    use rand::RngExt;

    // Generate a 16-byte random salt using rand 0.10
    let salt: [u8; 16] = rand::rng().random();

    // Argon2id parameters (OWASP recommended minimums)
    let m_cost = 19 * 1024; // 19 MiB in KiB
    let t_cost = 2;
    let p_cost = 1;
    let output_len = 32;

    let params = Params::new(m_cost, t_cost, p_cost, Some(output_len))
        .map_err(|e| crate::error::AppError::Internal(anyhow::anyhow!(e)))?;

    let argon2 = Argon2::new(Algorithm::Argon2id, Version::V0x13, params);

    let mut output = vec![0u8; output_len];
    argon2
        .hash_password_into(password.as_bytes(), &salt, &mut output)
        .map_err(|e| crate::error::AppError::Internal(anyhow::anyhow!(e)))?;

    // Encode to PHC format: $argon2id$v=19$m=19456,t=2,p=1$<salt>$<hash>
    // Note: v=19 is the decimal value of 0x13
    let salt_b64 = Base64Unpadded::encode_string(&salt);
    let hash_b64 = Base64Unpadded::encode_string(&output);

    Ok(format!(
        "$argon2id$v=19$m={},t={},p={}${}${}",
        m_cost, t_cost, p_cost, salt_b64, hash_b64
    ))
}
