use std::sync::Arc;

use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};

use crate::{
    dto::semester::{CreateSemesterRequest, SemesterResponse, UpdateSemesterRequest},
    error::app_error::AppError,
    service::semester_service::SemesterService,
    state::app_state::AppState,
};

pub async fn create_semester(
    State(state): State<Arc<AppState>>,
    Json(request): Json<CreateSemesterRequest>,
) -> Result<(StatusCode, Json<SemesterResponse>), AppError> {
    let conn = state
        .pool
        .get()
        .await
        .map_err(|_| AppError::DatabaseError)?;

    let semester = conn
        .interact(move |conn| SemesterService::create(conn, request))
        .await
        .map_err(|_| AppError::DatabaseError)??;

    Ok((StatusCode::CREATED, Json(semester)))
}

pub async fn get_semesters(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<SemesterResponse>>, AppError> {
    let conn = state
        .pool
        .get()
        .await
        .map_err(|_| AppError::DatabaseError)?;

    let semesters = conn
        .interact(move |conn| SemesterService::get_all(conn))
        .await
        .map_err(|_| AppError::DatabaseError)??;

    Ok(Json(semesters))
}

pub async fn get_semester(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i64>,
) -> Result<Json<SemesterResponse>, AppError> {
    let conn = state
        .pool
        .get()
        .await
        .map_err(|_| AppError::DatabaseError)?;

    let semester = conn
        .interact(move |conn| SemesterService::get_by_id(conn, id))
        .await
        .map_err(|_| AppError::DatabaseError)??;

    Ok(Json(semester))
}

pub async fn update_semester(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i64>,
    Json(request): Json<UpdateSemesterRequest>,
) -> Result<Json<SemesterResponse>, AppError> {
    let conn = state
        .pool
        .get()
        .await
        .map_err(|_| AppError::DatabaseError)?;

    let semester = conn
        .interact(move |conn| SemesterService::update(conn, id, request))
        .await
        .map_err(|_| AppError::DatabaseError)??;

    Ok(Json(semester))
}

pub async fn delete_semester(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i64>,
) -> Result<StatusCode, AppError> {
    let conn = state
        .pool
        .get()
        .await
        .map_err(|_| AppError::DatabaseError)?;

    conn.interact(move |conn| SemesterService::delete(conn, id))
        .await
        .map_err(|_| AppError::DatabaseError)??;

    Ok(StatusCode::NO_CONTENT)
}
