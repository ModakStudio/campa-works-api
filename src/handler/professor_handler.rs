use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};

use std::sync::Arc;

use crate::{
    dto::professor::{CreateProfessorRequest, ProfessorResponse, UpdateProfessorRequest},
    error::app_error::AppError,
    service::professor_service::ProfessorService,
    state::app_state::AppState,
};

pub async fn create_professor(
    State(state): State<Arc<AppState>>,
    Json(request): Json<CreateProfessorRequest>,
) -> Result<(StatusCode, Json<ProfessorResponse>), AppError> {
    let mut conn = state
        .pool
        .get()
        .await
        .map_err(|_| AppError::DatabaseError)?;

    let professor = ProfessorService::create(conn.as_mut(), request)?;

    Ok((StatusCode::CREATED, Json(professor)))
}

pub async fn get_professors(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<ProfessorResponse>>, AppError> {
    let mut conn = state
        .pool
        .get()
        .await
        .map_err(|_| AppError::DatabaseError)?;

    let professors = ProfessorService::get_all(conn.as_mut())?;

    Ok(Json(professors))
}

pub async fn get_professor(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i64>,
) -> Result<Json<ProfessorResponse>, AppError> {
    let mut conn = state
        .pool
        .get()
        .await
        .map_err(|_| AppError::DatabaseError)?;

    let professor = ProfessorService::get_by_id(conn.as_mut(), id)?;

    Ok(Json(professor))
}

pub async fn update_professor(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i64>,
    Json(request): Json<UpdateProfessorRequest>,
) -> Result<Json<ProfessorResponse>, AppError> {
    let mut conn = state
        .pool
        .get()
        .await
        .map_err(|_| AppError::DatabaseError)?;

    let professor = ProfessorService::update(conn.as_mut(), id, request)?;

    Ok(Json(professor))
}

pub async fn delete_professor(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i64>,
) -> Result<StatusCode, AppError> {
    let mut conn = state
        .pool
        .get()
        .await
        .map_err(|_| AppError::DatabaseError)?;

    ProfessorService::delete(conn.as_mut(), id)?;

    Ok(StatusCode::NO_CONTENT)
}
