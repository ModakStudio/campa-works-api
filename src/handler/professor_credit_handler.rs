use std::sync::Arc;

use axum::{
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
};

use crate::{
    dto::professor_credit::{
        CreateProfessorCreditRequest, ProfessorCreditResponse, UpdateProfessorCreditRequest,
    },
    error::app_error::AppError,
    service::professor_credit_service::ProfessorCreditService,
    state::app_state::AppState,
};

pub async fn create_professor_credit(
    State(state): State<Arc<AppState>>,
    Json(request): Json<CreateProfessorCreditRequest>,
) -> Result<(StatusCode, Json<ProfessorCreditResponse>), AppError> {
    let conn = state
        .pool
        .get()
        .await
        .map_err(|_| AppError::DatabaseError)?;

    let professor_credit = conn
        .interact(move |conn| ProfessorCreditService::create(conn, request))
        .await
        .map_err(|_| AppError::DatabaseError)??;

    Ok((StatusCode::CREATED, Json(professor_credit)))
}

pub async fn get_professor_credits(
    State(state): State<Arc<AppState>>,
    Query(params): Query<std::collections::HashMap<String, String>>,
) -> Result<Json<Vec<ProfessorCreditResponse>>, AppError> {
    let conn = state
        .pool
        .get()
        .await
        .map_err(|_| AppError::DatabaseError)?;

    let professor_credits = conn
        .interact(move |conn| ProfessorCreditService::get_all(conn, &params))
        .await
        .map_err(|_| AppError::DatabaseError)??;

    Ok(Json(professor_credits))
}

pub async fn get_professor_credit(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i64>,
) -> Result<Json<ProfessorCreditResponse>, AppError> {
    let conn = state
        .pool
        .get()
        .await
        .map_err(|_| AppError::DatabaseError)?;

    let professor_credit = conn
        .interact(move |conn| ProfessorCreditService::get_by_id(conn, id))
        .await
        .map_err(|_| AppError::DatabaseError)??;

    Ok(Json(professor_credit))
}

pub async fn update_professor_credit(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i64>,
    Json(request): Json<UpdateProfessorCreditRequest>,
) -> Result<Json<ProfessorCreditResponse>, AppError> {
    let conn = state
        .pool
        .get()
        .await
        .map_err(|_| AppError::DatabaseError)?;

    let professor_credit = conn
        .interact(move |conn| ProfessorCreditService::update(conn, id, request))
        .await
        .map_err(|_| AppError::DatabaseError)??;

    Ok(Json(professor_credit))
}

pub async fn delete_professor_credit(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i64>,
) -> Result<StatusCode, AppError> {
    let conn = state
        .pool
        .get()
        .await
        .map_err(|_| AppError::DatabaseError)?;

    conn.interact(move |conn| ProfessorCreditService::delete(conn, id))
        .await
        .map_err(|_| AppError::DatabaseError)??;

    Ok(StatusCode::NO_CONTENT)
}
