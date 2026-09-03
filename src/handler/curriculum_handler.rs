use std::sync::Arc;

use axum::{
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
};

use crate::{
    dto::curriculum::{CreateCurriculumRequest, CurriculumResponse},
    error::app_error::AppError,
    service::curriculum_service::CurriculumService,
    state::app_state::AppState,
};

pub async fn create_curriculum(
    State(state): State<Arc<AppState>>,
    Json(request): Json<CreateCurriculumRequest>,
) -> Result<(StatusCode, Json<CurriculumResponse>), AppError> {
    let conn = state
        .pool
        .get()
        .await
        .map_err(|_| AppError::DatabaseError)?;

    let curriculum = conn
        .interact(move |conn| CurriculumService::create(conn, request))
        .await
        .map_err(|_| AppError::DatabaseError)??;

    Ok((StatusCode::CREATED, Json(curriculum)))
}

pub async fn get_curriculums(
    State(state): State<Arc<AppState>>,
    Query(params): Query<std::collections::HashMap<String, String>>,
) -> Result<Json<Vec<CurriculumResponse>>, AppError> {
    let conn = state
        .pool
        .get()
        .await
        .map_err(|_| AppError::DatabaseError)?;

    let curriculums = conn
        .interact(move |conn| CurriculumService::get_all(conn, &params))
        .await
        .map_err(|_| AppError::DatabaseError)??;

    Ok(Json(curriculums))
}

pub async fn get_curriculum(
    State(state): State<Arc<AppState>>,
    Path(curriculum_id): Path<i64>,
) -> Result<Json<CurriculumResponse>, AppError> {
    let conn = state
        .pool
        .get()
        .await
        .map_err(|_| AppError::DatabaseError)?;

    let curriculum = conn
        .interact(move |conn| CurriculumService::get_by_id(conn, curriculum_id))
        .await
        .map_err(|_| AppError::DatabaseError)??;

    Ok(Json(curriculum))
}

pub async fn delete_curriculum(
    State(state): State<Arc<AppState>>,
    Path(curriculum_id): Path<i64>,
) -> Result<StatusCode, AppError> {
    let conn = state
        .pool
        .get()
        .await
        .map_err(|_| AppError::DatabaseError)?;

    conn.interact(move |conn| CurriculumService::delete(conn, curriculum_id))
        .await
        .map_err(|_| AppError::DatabaseError)??;

    Ok(StatusCode::NO_CONTENT)
}
