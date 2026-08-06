use std::sync::Arc;

use axum::{
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
};

use crate::{
    dto::course_assignment::{CourseAssignmentResponse, CreateCourseAssignmentRequest},
    error::app_error::AppError,
    service::course_assignment_service::CourseAssignmentService,
    state::app_state::AppState,
};

pub async fn create_course_assignment(
    State(state): State<Arc<AppState>>,
    Json(request): Json<CreateCourseAssignmentRequest>,
) -> Result<(StatusCode, Json<CourseAssignmentResponse>), AppError> {
    let conn = state
        .pool
        .get()
        .await
        .map_err(|_| AppError::DatabaseError)?;

    let course_assignment = conn
        .interact(move |conn| CourseAssignmentService::create(conn, request))
        .await
        .map_err(|_| AppError::DatabaseError)??;

    Ok((StatusCode::CREATED, Json(course_assignment)))
}

pub async fn get_course_assignments(
    State(state): State<Arc<AppState>>,
    Query(params): Query<std::collections::HashMap<String, String>>,
) -> Result<Json<Vec<CourseAssignmentResponse>>, AppError> {
    let conn = state
        .pool
        .get()
        .await
        .map_err(|_| AppError::DatabaseError)?;

    let course_assignments = conn
        .interact(move |conn| CourseAssignmentService::get_all(conn, &params))
        .await
        .map_err(|_| AppError::DatabaseError)??;

    Ok(Json(course_assignments))
}

pub async fn get_course_assignment(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i64>,
) -> Result<Json<CourseAssignmentResponse>, AppError> {
    let conn = state
        .pool
        .get()
        .await
        .map_err(|_| AppError::DatabaseError)?;

    let course_assignment = conn
        .interact(move |conn| CourseAssignmentService::get_by_id(conn, id))
        .await
        .map_err(|_| AppError::DatabaseError)??;

    Ok(Json(course_assignment))
}

pub async fn delete_course_assignment(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i64>,
) -> Result<StatusCode, AppError> {
    let conn = state
        .pool
        .get()
        .await
        .map_err(|_| AppError::DatabaseError)?;

    conn.interact(move |conn| CourseAssignmentService::delete(conn, id))
        .await
        .map_err(|_| AppError::DatabaseError)??;

    Ok(StatusCode::NO_CONTENT)
}
