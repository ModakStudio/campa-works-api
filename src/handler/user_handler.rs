use axum::{
    Json,
    extract::{Path, State},
};

use crate::{
    dto::{
        response::{ApiResponse, EmptyResponse},
        user::{CreateUserRequest, UpdateUserRequest, UserResponse},
    },
    error::app_error::AppError,
    service::user_service::UserService,
    state::app_state::AppState,
};

pub async fn create_user(
    State(state): State<AppState>,
    Json(request): Json<CreateUserRequest>,
) -> Result<Json<ApiResponse<UserResponse>>, AppError> {
    let mut conn = state.pool.get().map_err(|_| AppError::DatabaseError)?;

    let user = UserService::create(&mut conn, request)?;

    Ok(Json(ApiResponse::ok(user)))
}

pub async fn get_users(
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<Vec<UserResponse>>>, AppError> {
    let mut conn = state.pool.get().map_err(|_| AppError::DatabaseError)?;

    let users = UserService::get_all(&mut conn)?;

    Ok(Json(ApiResponse::ok(users)))
}

pub async fn get_user(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Json<ApiResponse<UserResponse>>, AppError> {
    let mut conn = state.pool.get().map_err(|_| AppError::DatabaseError)?;

    let user = UserService::get_by_id(&mut conn, id)?;

    Ok(Json(ApiResponse::ok(user)))
}

pub async fn update_user(
    State(state): State<AppState>,
    Path(id): Path<i64>,
    Json(request): Json<UpdateUserRequest>,
) -> Result<Json<ApiResponse<UserResponse>>, AppError> {
    let mut conn = state.pool.get().map_err(|_| AppError::DatabaseError)?;

    let user = UserService::update(&mut conn, id, request)?;

    Ok(Json(ApiResponse::ok(user)))
}

pub async fn delete_user(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Json<EmptyResponse>, AppError> {
    let mut conn = state.pool.get().map_err(|_| AppError::DatabaseError)?;

    UserService::delete(&mut conn, id)?;

    Ok(Json(EmptyResponse { success: true }))
}
