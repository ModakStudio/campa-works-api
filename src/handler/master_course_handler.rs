use std::sync::Arc;

use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};

use crate::{
    dto::master_course::{
        CreateMasterCourseRequest, MasterCourseResponse, UpdateMasterCourseRequest,
    },
    error::app_error::AppError,
    service::master_course_service::MasterCourseService,
    state::app_state::AppState,
};

pub async fn create_master_course(
    State(state): State<Arc<AppState>>,
    Json(request): Json<CreateMasterCourseRequest>,
) -> Result<(StatusCode, Json<MasterCourseResponse>), AppError> {
    let conn = state
        .pool
        .get()
        .await
        .map_err(|_| AppError::DatabaseError)?;

    let course = conn
        .interact(move |conn| MasterCourseService::create(conn, request))
        .await
        .map_err(|_| AppError::DatabaseError)??;

    Ok((StatusCode::CREATED, Json(course)))
}

pub async fn get_master_courses(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<MasterCourseResponse>>, AppError> {
    let conn = state
        .pool
        .get()
        .await
        .map_err(|_| AppError::DatabaseError)?;

    let courses = conn
        .interact(move |conn| MasterCourseService::get_all(conn))
        .await
        .map_err(|_| AppError::DatabaseError)??;

    Ok(Json(courses))
}

pub async fn get_master_course(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i64>,
) -> Result<Json<MasterCourseResponse>, AppError> {
    let conn = state
        .pool
        .get()
        .await
        .map_err(|_| AppError::DatabaseError)?;

    let course = conn
        .interact(move |conn| MasterCourseService::get_by_id(conn, id))
        .await
        .map_err(|_| AppError::DatabaseError)??;

    Ok(Json(course))
}

pub async fn update_master_course(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i64>,
    Json(request): Json<UpdateMasterCourseRequest>,
) -> Result<Json<MasterCourseResponse>, AppError> {
    let conn = state
        .pool
        .get()
        .await
        .map_err(|_| AppError::DatabaseError)?;

    let course = conn
        .interact(move |conn| MasterCourseService::update(conn, id, request))
        .await
        .map_err(|_| AppError::DatabaseError)??;

    Ok(Json(course))
}

pub async fn delete_master_course(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i64>,
) -> Result<StatusCode, AppError> {
    let conn = state
        .pool
        .get()
        .await
        .map_err(|_| AppError::DatabaseError)?;

    conn.interact(move |conn| MasterCourseService::delete(conn, id))
        .await
        .map_err(|_| AppError::DatabaseError)??;

    Ok(StatusCode::NO_CONTENT)
}
