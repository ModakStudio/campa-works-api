use diesel::PgConnection;

use crate::{
    dto::user::{CreateUserRequest, UpdateUserRequest, UserResponse},
    error::app_error::AppError,
    models::user::{NewUser, UpdateUser},
    repository::user_repository::UserRepository,
    utils::password::hash_password,
};

pub struct UserService;

impl UserService {
    pub fn create(
        conn: &mut PgConnection,
        request: CreateUserRequest,
    ) -> Result<UserResponse, AppError> {
        if UserRepository::find_by_email(conn, &request.email).is_ok() {
            return Err(AppError::EmailAlreadyExists);
        }

        let hashed_password = hash_password(&request.password);

        let new_user = NewUser {
            email: request.email,
            password: hashed_password,
            name: request.name,
            role: request.role,
        };

        let user = UserRepository::create(conn, &new_user).map_err(|e| {
            println!("DATABASE ERROR: {:#?}", e);
            AppError::DatabaseError
        })?;

        Ok(user.into())
    }

    pub fn get_by_id(conn: &mut PgConnection, user_id: i64) -> Result<UserResponse, AppError> {
        let user = UserRepository::find_by_id(conn, user_id).map_err(|_| AppError::UserNotFound)?;

        Ok(user.into())
    }

    pub fn get_all(conn: &mut PgConnection) -> Result<Vec<UserResponse>, AppError> {
        let users = UserRepository::find_all(conn).map_err(|_| AppError::DatabaseError)?;

        Ok(users.into_iter().map(Into::into).collect())
    }

    pub fn update(
        conn: &mut PgConnection,
        user_id: i64,
        request: UpdateUserRequest,
    ) -> Result<UserResponse, AppError> {
        let update_user = UpdateUser { name: request.name };

        let user = UserRepository::update(conn, user_id, &update_user).map_err(|e| match e {
            diesel::result::Error::NotFound => AppError::UserNotFound,
            _ => AppError::DatabaseError,
        })?;

        Ok(user.into())
    }

    pub fn delete(conn: &mut PgConnection, user_id: i64) -> Result<(), AppError> {
        UserRepository::find_by_id(conn, user_id).map_err(|_| AppError::UserNotFound)?;

        UserRepository::delete(conn, user_id).map_err(|_| AppError::DatabaseError)?;

        Ok(())
    }
}
