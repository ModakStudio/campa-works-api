use std::sync::Arc;

use axum::{
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
};

use crate::{
    dto::professor_quota::{
        CreateProfessorQuotaRequest, ProfessorQuotaResponse, UpdateProfessorQuotaRequest,
    },
    error::app_error::AppError,
    service::professor_quota_service::ProfessorQuotaService,
    state::app_state::AppState,
};

pub async fn create_professor_quota(
    State(state): State<Arc<AppState>>,
    Json(request): Json<CreateProfessorQuotaRequest>,
) -> Result<(StatusCode, Json<ProfessorQuotaResponse>), AppError> {
    let conn = state
        .pool
        .get()
        .await
        .map_err(|_| AppError::DatabaseError)?;

    let professor_quota = conn
        .interact(move |conn| ProfessorQuotaService::create(conn, request))
        .await
        .map_err(|_| AppError::DatabaseError)??;

    Ok((StatusCode::CREATED, Json(professor_quota)))
}

pub async fn get_professor_quotas(
    State(state): State<Arc<AppState>>,
    Query(params): Query<std::collections::HashMap<String, String>>,
) -> Result<Json<Vec<ProfessorQuotaResponse>>, AppError> {
    let conn = state
        .pool
        .get()
        .await
        .map_err(|_| AppError::DatabaseError)?;

    let professor_quotas = conn
        .interact(move |conn| ProfessorQuotaService::get_all(conn, &params))
        .await
        .map_err(|_| AppError::DatabaseError)??;

    Ok(Json(professor_quotas))
}

pub async fn get_professor_quota(
    State(state): State<Arc<AppState>>,
    Path(professor_quota_id): Path<i64>,
) -> Result<Json<ProfessorQuotaResponse>, AppError> {
    let conn = state
        .pool
        .get()
        .await
        .map_err(|_| AppError::DatabaseError)?;

    let professor_quota = conn
        .interact(move |conn| ProfessorQuotaService::get_by_id(conn, professor_quota_id))
        .await
        .map_err(|_| AppError::DatabaseError)??;

    Ok(Json(professor_quota))
}

pub async fn update_professor_quota(
    State(state): State<Arc<AppState>>,
    Path(professor_quota_id): Path<i64>,
    Json(request): Json<UpdateProfessorQuotaRequest>,
) -> Result<Json<ProfessorQuotaResponse>, AppError> {
    let conn = state
        .pool
        .get()
        .await
        .map_err(|_| AppError::DatabaseError)?;

    let professor_quota = conn
        .interact(move |conn| ProfessorQuotaService::update(conn, professor_quota_id, request))
        .await
        .map_err(|_| AppError::DatabaseError)??;

    Ok(Json(professor_quota))
}

pub async fn delete_professor_quota(
    State(state): State<Arc<AppState>>,
    Path(professor_quota_id): Path<i64>,
) -> Result<StatusCode, AppError> {
    let conn = state
        .pool
        .get()
        .await
        .map_err(|_| AppError::DatabaseError)?;

    conn.interact(move |conn| ProfessorQuotaService::delete(conn, professor_quota_id))
        .await
        .map_err(|_| AppError::DatabaseError)??;

    Ok(StatusCode::NO_CONTENT)
}
