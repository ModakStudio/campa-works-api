use serde::{Deserialize, Serialize};

use crate::models::{enums::UserRole, user::User};

#[derive(Debug, Deserialize)]
pub struct CreateUserRequest {
    pub email: String,
    pub password: String,
    pub username: String,

    pub role: UserRole,

    pub is_super: bool,
}

#[derive(Debug, Deserialize)]
pub struct UpdateUserRequest {
    pub email: Option<String>,
    pub username: Option<String>,
    pub password: Option<String>,

    pub role: Option<UserRole>,

    pub is_super: Option<bool>,
}

#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub id: i64,

    pub email: String,
    pub username: String,

    pub role: UserRole,

    pub is_super: bool,
}

impl From<User> for UserResponse {
    fn from(user: User) -> Self {
        Self {
            id: user.id,

            email: user.email,
            username: user.username,

            role: user.role,

            is_super: user.is_super,
        }
    }
}

impl From<&str> for UserRole {
    fn from(role: &str) -> Self {
        match role {
            "ADMIN" => UserRole::Admin,
            "PROFESSOR" => UserRole::Professor,
            "ASSISTANT" => UserRole::Assistant,
            "STAFF" => UserRole::Staff,
            _ => UserRole::Staff,
        }
    }
}
