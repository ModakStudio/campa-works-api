use std::sync::Arc;

use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};

use crate::{
    dto::major::{CreateMajorRequest, MajorResponse, UpdateMajorRequest},
    error::app_error::AppError,
    service::major_service::MajorService,
    state::app_state::AppState,
};

pub async fn create_major(
    State(state): State<Arc<AppState>>,
    Json(request): Json<CreateMajorRequest>,
) -> Result<(StatusCode, Json<MajorResponse>), AppError> {
    let conn = state
        .pool
        .get()
        .await
        .map_err(|_| AppError::DatabaseError)?;

    let major = conn
        .interact(move |conn| MajorService::create(conn, request))
        .await
        .map_err(|_| AppError::DatabaseError)??;

    Ok((StatusCode::CREATED, Json(major)))
}

pub async fn get_majors(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<MajorResponse>>, AppError> {
    let conn = state
        .pool
        .get()
        .await
        .map_err(|_| AppError::DatabaseError)?;

    let majors = conn
        .interact(move |conn| MajorService::get_all(conn))
        .await
        .map_err(|_| AppError::DatabaseError)??;

    Ok(Json(majors))
}

pub async fn get_major(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i64>,
) -> Result<Json<MajorResponse>, AppError> {
    let conn = state
        .pool
        .get()
        .await
        .map_err(|_| AppError::DatabaseError)?;

    let major = conn
        .interact(move |conn| MajorService::get_by_id(conn, id))
        .await
        .map_err(|_| AppError::DatabaseError)??;

    Ok(Json(major))
}

pub async fn update_major(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i64>,
    Json(request): Json<UpdateMajorRequest>,
) -> Result<Json<MajorResponse>, AppError> {
    let conn = state
        .pool
        .get()
        .await
        .map_err(|_| AppError::DatabaseError)?;

    let major = conn
        .interact(move |conn| MajorService::update(conn, id, request))
        .await
        .map_err(|_| AppError::DatabaseError)??;

    Ok(Json(major))
}

pub async fn delete_major(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i64>,
) -> Result<StatusCode, AppError> {
    let conn = state
        .pool
        .get()
        .await
        .map_err(|_| AppError::DatabaseError)?;

    conn.interact(move |conn| MajorService::delete(conn, id))
        .await
        .map_err(|_| AppError::DatabaseError)??;

    Ok(StatusCode::NO_CONTENT)
}
