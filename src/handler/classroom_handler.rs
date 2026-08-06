use std::{collections::HashMap, sync::Arc};

use axum::{
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
};

use crate::{
    dto::classroom::{ClassroomResponse, CreateClassroomRequest, UpdateClassroomRequest},
    error::app_error::AppError,
    service::classroom_service::ClassroomService,
    state::app_state::AppState,
};

pub async fn create_classroom(
    State(state): State<Arc<AppState>>,
    Json(request): Json<CreateClassroomRequest>,
) -> Result<(StatusCode, Json<ClassroomResponse>), AppError> {
    let conn = state
        .pool
        .get()
        .await
        .map_err(|_| AppError::DatabaseError)?;

    let classroom = conn
        .interact(move |conn| ClassroomService::create(conn, request))
        .await
        .map_err(|_| AppError::DatabaseError)??;

    Ok((StatusCode::CREATED, Json(classroom)))
}

pub async fn get_classrooms(
    State(state): State<Arc<AppState>>,
    Query(params): Query<HashMap<String, String>>,
) -> Result<Json<Vec<ClassroomResponse>>, AppError> {
    let conn = state
        .pool
        .get()
        .await
        .map_err(|_| AppError::DatabaseError)?;

    let classrooms = conn
        .interact(move |conn| ClassroomService::get_all(conn, &params))
        .await
        .map_err(|_| AppError::DatabaseError)??;

    Ok(Json(classrooms))
}

pub async fn get_classroom(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i64>,
) -> Result<Json<ClassroomResponse>, AppError> {
    let conn = state
        .pool
        .get()
        .await
        .map_err(|_| AppError::DatabaseError)?;

    let classroom = conn
        .interact(move |conn| ClassroomService::get_by_id(conn, id))
        .await
        .map_err(|_| AppError::DatabaseError)??;

    Ok(Json(classroom))
}

pub async fn update_classroom(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i64>,
    Json(request): Json<UpdateClassroomRequest>,
) -> Result<Json<ClassroomResponse>, AppError> {
    let conn = state
        .pool
        .get()
        .await
        .map_err(|_| AppError::DatabaseError)?;

    let classroom = conn
        .interact(move |conn| ClassroomService::update(conn, id, request))
        .await
        .map_err(|_| AppError::DatabaseError)??;

    Ok(Json(classroom))
}

pub async fn delete_classroom(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i64>,
) -> Result<StatusCode, AppError> {
    let conn = state
        .pool
        .get()
        .await
        .map_err(|_| AppError::DatabaseError)?;

    conn.interact(move |conn| ClassroomService::delete(conn, id))
        .await
        .map_err(|_| AppError::DatabaseError)??;

    Ok(StatusCode::NO_CONTENT)
}
