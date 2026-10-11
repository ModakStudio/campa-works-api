use std::sync::Arc;

use axum::{
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
};

use crate::{
    dto::course::{CourseResponse, CreateCourseRequest, UpdateCourseRequest},
    error::app_error::AppError,
    service::course_service::CourseService,
    state::app_state::AppState,
};

pub async fn create_course(
    State(state): State<Arc<AppState>>,
    Json(request): Json<CreateCourseRequest>,
) -> Result<(StatusCode, Json<CourseResponse>), AppError> {
    let conn = state
        .pool
        .get()
        .await
        .map_err(|_| AppError::DatabaseError)?;

    let course = conn
        .interact(move |conn| CourseService::create(conn, request))
        .await
        .map_err(|_| AppError::DatabaseError)??;

    Ok((StatusCode::CREATED, Json(course)))
}

pub async fn create_courses_auto(
    State(state): State<Arc<AppState>>,
    Path(semester_id): Path<i64>,
) -> Result<(StatusCode, Json<Vec<CourseResponse>>), AppError> {
    let conn = state
        .pool
        .get()
        .await
        .map_err(|_| AppError::DatabaseError)?;

    let courses = conn
        .interact(move |conn| CourseService::create_auto(conn, semester_id))
        .await
        .map_err(|_| AppError::DatabaseError)??;

    Ok((StatusCode::CREATED, Json(courses)))
}

pub async fn get_courses(
    State(state): State<Arc<AppState>>,
    Query(params): Query<std::collections::HashMap<String, String>>,
) -> Result<Json<Vec<CourseResponse>>, AppError> {
    let conn = state
        .pool
        .get()
        .await
        .map_err(|_| AppError::DatabaseError)?;

    let courses = conn
        .interact(move |conn| CourseService::get_all(conn, &params))
        .await
        .map_err(|_| AppError::DatabaseError)??;

    Ok(Json(courses))
}

pub async fn get_course(
    State(state): State<Arc<AppState>>,
    Path(course_id): Path<i64>,
) -> Result<Json<CourseResponse>, AppError> {
    let conn = state
        .pool
        .get()
        .await
        .map_err(|_| AppError::DatabaseError)?;

    let course = conn
        .interact(move |conn| CourseService::get_by_id(conn, course_id))
        .await
        .map_err(|_| AppError::DatabaseError)??;

    Ok(Json(course))
}

pub async fn update_course(
    State(state): State<Arc<AppState>>,
    Path(course_id): Path<i64>,
    Json(request): Json<UpdateCourseRequest>,
) -> Result<Json<CourseResponse>, AppError> {
    let conn = state
        .pool
        .get()
        .await
        .map_err(|_| AppError::DatabaseError)?;

    let course = conn
        .interact(move |conn| CourseService::update(conn, course_id, request))
        .await
        .map_err(|_| AppError::DatabaseError)??;

    Ok(Json(course))
}

pub async fn delete_course(
    State(state): State<Arc<AppState>>,
    Path(course_id): Path<i64>,
) -> Result<StatusCode, AppError> {
    let conn = state
        .pool
        .get()
        .await
        .map_err(|_| AppError::DatabaseError)?;

    conn.interact(move |conn| CourseService::delete(conn, course_id))
        .await
        .map_err(|_| AppError::DatabaseError)??;

    Ok(StatusCode::NO_CONTENT)
}
