use std::sync::Arc;

use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};

use crate::{
    dto::facility::{CreateFacilityRequest, FacilityResponse, UpdateFacilityRequest},
    error::app_error::AppError,
    service::facility_service::FacilityService,
    state::app_state::AppState,
};

pub async fn create_facility(
    State(state): State<Arc<AppState>>,
    Json(request): Json<CreateFacilityRequest>,
) -> Result<(StatusCode, Json<FacilityResponse>), AppError> {
    let conn = state
        .pool
        .get()
        .await
        .map_err(|_| AppError::DatabaseError)?;

    let facility = conn
        .interact(move |conn| FacilityService::create(conn, request))
        .await
        .map_err(|_| AppError::DatabaseError)??;

    Ok((StatusCode::CREATED, Json(facility)))
}

pub async fn get_facilities(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<FacilityResponse>>, AppError> {
    let conn = state
        .pool
        .get()
        .await
        .map_err(|_| AppError::DatabaseError)?;

    let facilities = conn
        .interact(move |conn| FacilityService::get_all(conn))
        .await
        .map_err(|_| AppError::DatabaseError)??;

    Ok(Json(facilities))
}

pub async fn get_facility(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i64>,
) -> Result<Json<FacilityResponse>, AppError> {
    let conn = state
        .pool
        .get()
        .await
        .map_err(|_| AppError::DatabaseError)?;

    let facility = conn
        .interact(move |conn| FacilityService::get_by_id(conn, id))
        .await
        .map_err(|_| AppError::DatabaseError)??;

    Ok(Json(facility))
}

pub async fn update_facility(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i64>,
    Json(request): Json<UpdateFacilityRequest>,
) -> Result<Json<FacilityResponse>, AppError> {
    let conn = state
        .pool
        .get()
        .await
        .map_err(|_| AppError::DatabaseError)?;

    let facility = conn
        .interact(move |conn| FacilityService::update(conn, id, request))
        .await
        .map_err(|_| AppError::DatabaseError)??;

    Ok(Json(facility))
}

pub async fn delete_facility(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i64>,
) -> Result<StatusCode, AppError> {
    let conn = state
        .pool
        .get()
        .await
        .map_err(|_| AppError::DatabaseError)?;

    conn.interact(move |conn| FacilityService::delete(conn, id))
        .await
        .map_err(|_| AppError::DatabaseError)??;

    Ok(StatusCode::NO_CONTENT)
}
