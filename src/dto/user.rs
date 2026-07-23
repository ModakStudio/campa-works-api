use serde::{Deserialize, Serialize};

use crate::models::{
    enums::{UserRole, UserStatus},
    user::User,
};

#[derive(Debug, Deserialize)]
pub struct CreateUserRequest {
    pub email: String,
    pub password: String,
    pub name: String,
    pub role: UserRole,
}

#[derive(Debug, Deserialize)]
pub struct UpdateUserRequest {
    pub name: Option<String>,
    pub status: Option<UserStatus>,
}

#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub id: i64,
    pub email: String,
    pub name: String,
    pub role: UserRole,
    pub status: UserStatus,
}

impl From<User> for UserResponse {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            email: user.email,
            name: user.name,
            role: user.role,
            status: user.status,
        }
    }
}
