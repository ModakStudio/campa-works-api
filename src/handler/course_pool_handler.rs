use std::sync::Arc;

use axum::{
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
};

use crate::{
    dto::course_pool::{CoursePoolResponse, CreateCoursePoolRequest},
    error::app_error::AppError,
    service::course_pool_service::CoursePoolService,
    state::app_state::AppState,
};

pub async fn create_course_pool(
    State(state): State<Arc<AppState>>,
    Json(request): Json<CreateCoursePoolRequest>,
) -> Result<(StatusCode, Json<CoursePoolResponse>), AppError> {
    let conn = state
        .pool
        .get()
        .await
        .map_err(|_| AppError::DatabaseError)?;

    let course_pool = conn
        .interact(move |conn| CoursePoolService::create(conn, request))
        .await
        .map_err(|_| AppError::DatabaseError)??;

    Ok((StatusCode::CREATED, Json(course_pool)))
}

pub async fn get_course_pools(
    State(state): State<Arc<AppState>>,
    Query(params): Query<std::collections::HashMap<String, String>>,
) -> Result<Json<Vec<CoursePoolResponse>>, AppError> {
    let conn = state
        .pool
        .get()
        .await
        .map_err(|_| AppError::DatabaseError)?;

    let course_pools = conn
        .interact(move |conn| CoursePoolService::get_all(conn, &params))
        .await
        .map_err(|_| AppError::DatabaseError)??;

    Ok(Json(course_pools))
}

pub async fn get_course_pool(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i64>,
) -> Result<Json<CoursePoolResponse>, AppError> {
    let conn = state
        .pool
        .get()
        .await
        .map_err(|_| AppError::DatabaseError)?;

    let course_pool = conn
        .interact(move |conn| CoursePoolService::get_by_id(conn, id))
        .await
        .map_err(|_| AppError::DatabaseError)??;

    Ok(Json(course_pool))
}

pub async fn delete_course_pool(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i64>,
) -> Result<StatusCode, AppError> {
    let conn = state
        .pool
        .get()
        .await
        .map_err(|_| AppError::DatabaseError)?;

    conn.interact(move |conn| CoursePoolService::delete(conn, id))
        .await
        .map_err(|_| AppError::DatabaseError)??;

    Ok(StatusCode::NO_CONTENT)
}
