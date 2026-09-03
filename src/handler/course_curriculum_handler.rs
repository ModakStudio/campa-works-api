use std::sync::Arc;

use axum::{
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
};

use crate::{
    dto::course_curriculum::{CourseCurriculumResponse, CreateCourseCurriculumRequest},
    error::app_error::AppError,
    service::course_curriculum_service::CourseCurriculumService,
    state::app_state::AppState,
};

pub async fn create_course_curriculum(
    State(state): State<Arc<AppState>>,
    Json(request): Json<CreateCourseCurriculumRequest>,
) -> Result<(StatusCode, Json<CourseCurriculumResponse>), AppError> {
    let conn = state
        .pool
        .get()
        .await
        .map_err(|_| AppError::DatabaseError)?;

    let course_curriculum = conn
        .interact(move |conn| CourseCurriculumService::create(conn, request))
        .await
        .map_err(|_| AppError::DatabaseError)??;

    Ok((StatusCode::CREATED, Json(course_curriculum)))
}

pub async fn get_course_curriculums(
    State(state): State<Arc<AppState>>,
    Query(params): Query<std::collections::HashMap<String, String>>,
) -> Result<Json<Vec<CourseCurriculumResponse>>, AppError> {
    let conn = state
        .pool
        .get()
        .await
        .map_err(|_| AppError::DatabaseError)?;

    let course_curriculums = conn
        .interact(move |conn| CourseCurriculumService::get_all(conn, &params))
        .await
        .map_err(|_| AppError::DatabaseError)??;

    Ok(Json(course_curriculums))
}

pub async fn get_course_curriculum(
    State(state): State<Arc<AppState>>,
    Path(course_curriculum_id): Path<i64>,
) -> Result<Json<CourseCurriculumResponse>, AppError> {
    let conn = state
        .pool
        .get()
        .await
        .map_err(|_| AppError::DatabaseError)?;

    let course_curriculum = conn
        .interact(move |conn| CourseCurriculumService::get_by_id(conn, course_curriculum_id))
        .await
        .map_err(|_| AppError::DatabaseError)??;

    Ok(Json(course_curriculum))
}

pub async fn delete_course_curriculum(
    State(state): State<Arc<AppState>>,
    Path(course_curriculum_id): Path<i64>,
) -> Result<StatusCode, AppError> {
    let conn = state
        .pool
        .get()
        .await
        .map_err(|_| AppError::DatabaseError)?;

    conn.interact(move |conn| CourseCurriculumService::delete(conn, course_curriculum_id))
        .await
        .map_err(|_| AppError::DatabaseError)??;

    Ok(StatusCode::NO_CONTENT)
}
