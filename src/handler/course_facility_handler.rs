use std::sync::Arc;

use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};

use crate::{
    dto::course_facility::{CourseFacilityResponse, CreateCourseFacilityRequest},
    error::app_error::AppError,
    service::course_facility_service::CourseFacilityService,
    state::app_state::AppState,
};

pub async fn create_course_facility(
    State(state): State<Arc<AppState>>,
    Json(request): Json<CreateCourseFacilityRequest>,
) -> Result<(StatusCode, Json<CourseFacilityResponse>), AppError> {
    let conn = state
        .pool
        .get()
        .await
        .map_err(|_| AppError::DatabaseError)?;

    let course_facility = conn
        .interact(move |conn| CourseFacilityService::create(conn, request))
        .await
        .map_err(|_| AppError::DatabaseError)??;

    Ok((StatusCode::CREATED, Json(course_facility)))
}

pub async fn get_course_facilities(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<CourseFacilityResponse>>, AppError> {
    let conn = state
        .pool
        .get()
        .await
        .map_err(|_| AppError::DatabaseError)?;

    let course_facilities = conn
        .interact(move |conn| CourseFacilityService::get_all(conn))
        .await
        .map_err(|_| AppError::DatabaseError)??;

    Ok(Json(course_facilities))
}

pub async fn get_course_facility(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i64>,
) -> Result<Json<CourseFacilityResponse>, AppError> {
    let conn = state
        .pool
        .get()
        .await
        .map_err(|_| AppError::DatabaseError)?;

    let course_facility = conn
        .interact(move |conn| CourseFacilityService::get_by_id(conn, id))
        .await
        .map_err(|_| AppError::DatabaseError)??;

    Ok(Json(course_facility))
}

pub async fn delete_course_facility(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i64>,
) -> Result<StatusCode, AppError> {
    let conn = state
        .pool
        .get()
        .await
        .map_err(|_| AppError::DatabaseError)?;

    conn.interact(move |conn| CourseFacilityService::delete(conn, id))
        .await
        .map_err(|_| AppError::DatabaseError)??;

    Ok(StatusCode::NO_CONTENT)
}
