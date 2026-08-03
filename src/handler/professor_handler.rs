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
    let conn = state
        .pool
        .get()
        .await
        .map_err(|_| AppError::DatabaseError)?;

    let professor = conn
        .interact(move |conn| ProfessorService::create(conn, request))
        .await
        .map_err(|_| AppError::DatabaseError)??;

    Ok((StatusCode::CREATED, Json(professor)))
}

pub async fn get_professors(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<ProfessorResponse>>, AppError> {
    let conn = state
        .pool
        .get()
        .await
        .map_err(|_| AppError::DatabaseError)?;

    let professors = conn
        .interact(move |conn| ProfessorService::get_all(conn))
        .await
        .map_err(|_| AppError::DatabaseError)??;

    Ok(Json(professors))
}

pub async fn get_professor(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i64>,
) -> Result<Json<ProfessorResponse>, AppError> {
    let conn = state
        .pool
        .get()
        .await
        .map_err(|_| AppError::DatabaseError)?;

    let professor = conn
        .interact(move |conn| ProfessorService::get_by_id(conn, id))
        .await
        .map_err(|_| AppError::DatabaseError)??;

    Ok(Json(professor))
}

pub async fn update_professor(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i64>,
    Json(request): Json<UpdateProfessorRequest>,
) -> Result<Json<ProfessorResponse>, AppError> {
    let conn = state
        .pool
        .get()
        .await
        .map_err(|_| AppError::DatabaseError)?;

    let professor = conn
        .interact(move |conn| ProfessorService::update(conn, id, request))
        .await
        .map_err(|_| AppError::DatabaseError)??;

    Ok(Json(professor))
}

pub async fn delete_professor(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i64>,
) -> Result<StatusCode, AppError> {
    let conn = state
        .pool
        .get()
        .await
        .map_err(|_| AppError::DatabaseError)?;

    conn.interact(move |conn| ProfessorService::delete(conn, id))
        .await
        .map_err(|_| AppError::DatabaseError)??;

    Ok(StatusCode::NO_CONTENT)
}
