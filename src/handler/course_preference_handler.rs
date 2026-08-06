use std::sync::Arc;

use axum::{
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
};

use crate::{
    dto::course_preference::{
        CoursePreferenceResponse, CreateCoursePreferenceRequest, UpdateCoursePreferenceRequest,
    },
    error::app_error::AppError,
    service::course_preference_service::CoursePreferenceService,
    state::app_state::AppState,
};

pub async fn create_course_preference(
    State(state): State<Arc<AppState>>,
    Json(request): Json<CreateCoursePreferenceRequest>,
) -> Result<(StatusCode, Json<CoursePreferenceResponse>), AppError> {
    let conn = state
        .pool
        .get()
        .await
        .map_err(|_| AppError::DatabaseError)?;

    let course_preference = conn
        .interact(move |conn| CoursePreferenceService::create(conn, request))
        .await
        .map_err(|_| AppError::DatabaseError)??;

    Ok((StatusCode::CREATED, Json(course_preference)))
}

pub async fn get_course_preferences(
    State(state): State<Arc<AppState>>,
    Query(params): Query<std::collections::HashMap<String, String>>,
) -> Result<Json<Vec<CoursePreferenceResponse>>, AppError> {
    let conn = state
        .pool
        .get()
        .await
        .map_err(|_| AppError::DatabaseError)?;

    let course_preferences = conn
        .interact(move |conn| CoursePreferenceService::get_all(conn, &params))
        .await
        .map_err(|_| AppError::DatabaseError)??;

    Ok(Json(course_preferences))
}

pub async fn get_course_preference(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i64>,
) -> Result<Json<CoursePreferenceResponse>, AppError> {
    let conn = state
        .pool
        .get()
        .await
        .map_err(|_| AppError::DatabaseError)?;

    let course_preference = conn
        .interact(move |conn| CoursePreferenceService::get_by_id(conn, id))
        .await
        .map_err(|_| AppError::DatabaseError)??;

    Ok(Json(course_preference))
}

pub async fn update_course_preference(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i64>,
    Json(request): Json<UpdateCoursePreferenceRequest>,
) -> Result<Json<CoursePreferenceResponse>, AppError> {
    let conn = state
        .pool
        .get()
        .await
        .map_err(|_| AppError::DatabaseError)?;

    let course_preference = conn
        .interact(move |conn| CoursePreferenceService::update(conn, id, request))
        .await
        .map_err(|_| AppError::DatabaseError)??;

    Ok(Json(course_preference))
}

pub async fn delete_course_preference(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i64>,
) -> Result<StatusCode, AppError> {
    let conn = state
        .pool
        .get()
        .await
        .map_err(|_| AppError::DatabaseError)?;

    conn.interact(move |conn| CoursePreferenceService::delete(conn, id))
        .await
        .map_err(|_| AppError::DatabaseError)??;

    Ok(StatusCode::NO_CONTENT)
}
