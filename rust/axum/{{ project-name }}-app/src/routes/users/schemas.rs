//! User API schemas with validation.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

use entity::user::Model as UserModel;

/// User response object.
#[derive(Debug, Serialize, ToSchema)]
pub struct UserResponse {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<UserModel> for UserResponse {
    fn from(model: UserModel) -> Self {
        Self {
            id: model.id,
            username: model.username,
            email: model.email,
            created_at: model.created_at.into(),
            updated_at: model.updated_at.into(),
        }
    }
}

/// Paginated list of users.
#[derive(Debug, Serialize, ToSchema)]
pub struct UserListResponse {
    pub items: Vec<UserResponse>,
    pub total: u64,
    pub page: u64,
    pub page_size: u64,
    pub total_pages: u64,
}

/// Request body for creating a user.
#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CreateUserRequest {
    /// Username (3-50 characters)
    #[validate(length(min = 3, max = 50))]
    pub username: String,

    /// Valid email address
    #[validate(email)]
    pub email: String,

    /// Password (min 8 chars, must contain letters and numbers)
    #[validate(length(min = 8))]
    #[validate(regex(
        path = "PASSWORD_REGEX",
        message = "password must contain at least one letter and one number"
    ))]
    pub password: String,
}

// Validator regex for password strength.
static PASSWORD_REGEX: std::sync::LazyLock<regex::Regex> =
    std::sync::LazyLock::new(|| regex::Regex::new(r"^(?=.*[A-Za-z])(?=.*\d).+$").unwrap());

/// Request body for updating a user.
///
/// All fields are optional; only provided fields will be updated.
#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct UpdateUserRequest {
    /// New username (3-50 characters)
    #[validate(length(min = 3, max = 50))]
    pub username: Option<String>,

    /// New email address
    #[validate(email)]
    pub email: Option<String>,

    /// New password (min 8 chars, must contain letters and numbers)
    #[validate(length(min = 8))]
    pub password: Option<String>,
}
