use std::sync::Arc;

use axum::{
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
};

use crate::{
    dto::classroom_facility::{ClassroomFacilityResponse, CreateClassroomFacilityRequest},
    error::app_error::AppError,
    service::classroom_facility_service::ClassroomFacilityService,
    state::app_state::AppState,
};

pub async fn create_classroom_facility(
    State(state): State<Arc<AppState>>,
    Json(request): Json<CreateClassroomFacilityRequest>,
) -> Result<(StatusCode, Json<ClassroomFacilityResponse>), AppError> {
    let conn = state
        .pool
        .get()
        .await
        .map_err(|_| AppError::DatabaseError)?;

    let classroom_facility = conn
        .interact(move |conn| ClassroomFacilityService::create(conn, request))
        .await
        .map_err(|_| AppError::DatabaseError)??;

    Ok((StatusCode::CREATED, Json(classroom_facility)))
}

pub async fn get_classroom_facilities(
    State(state): State<Arc<AppState>>,
    Query(params): Query<std::collections::HashMap<String, String>>,
) -> Result<Json<Vec<ClassroomFacilityResponse>>, AppError> {
    let conn = state
        .pool
        .get()
        .await
        .map_err(|_| AppError::DatabaseError)?;

    let classroom_facilities = conn
        .interact(move |conn| ClassroomFacilityService::get_all(conn, &params))
        .await
        .map_err(|_| AppError::DatabaseError)??;

    Ok(Json(classroom_facilities))
}

pub async fn get_classroom_facility(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i64>,
) -> Result<Json<ClassroomFacilityResponse>, AppError> {
    let conn = state
        .pool
        .get()
        .await
        .map_err(|_| AppError::DatabaseError)?;

    let classroom_facility = conn
        .interact(move |conn| ClassroomFacilityService::get_by_id(conn, id))
        .await
        .map_err(|_| AppError::DatabaseError)??;

    Ok(Json(classroom_facility))
}

pub async fn delete_classroom_facility(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i64>,
) -> Result<StatusCode, AppError> {
    let conn = state
        .pool
        .get()
        .await
        .map_err(|_| AppError::DatabaseError)?;

    conn.interact(move |conn| ClassroomFacilityService::delete(conn, id))
        .await
        .map_err(|_| AppError::DatabaseError)??;

    Ok(StatusCode::NO_CONTENT)
}
