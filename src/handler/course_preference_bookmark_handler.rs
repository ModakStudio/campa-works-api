use std::sync::Arc;

use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};

use crate::{
    dto::course_preference_bookmark::{
        CoursePreferenceBookmarkResponse, CreateCoursePreferenceBookmarkRequest,
    },
    error::app_error::AppError,
    service::course_preference_bookmark_service::CoursePreferenceBookmarkService,
    state::app_state::AppState,
};

pub async fn create_course_preference_bookmark(
    State(state): State<Arc<AppState>>,
    Json(request): Json<CreateCoursePreferenceBookmarkRequest>,
) -> Result<(StatusCode, Json<CoursePreferenceBookmarkResponse>), AppError> {
    let conn = state
        .pool
        .get()
        .await
        .map_err(|_| AppError::DatabaseError)?;

    let bookmark = conn
        .interact(move |conn| CoursePreferenceBookmarkService::create(conn, request))
        .await
        .map_err(|_| AppError::DatabaseError)??;

    Ok((StatusCode::CREATED, Json(bookmark)))
}

pub async fn get_course_preference_bookmarks(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<CoursePreferenceBookmarkResponse>>, AppError> {
    let conn = state
        .pool
        .get()
        .await
        .map_err(|_| AppError::DatabaseError)?;

    let bookmarks = conn
        .interact(move |conn| CoursePreferenceBookmarkService::get_all(conn))
        .await
        .map_err(|_| AppError::DatabaseError)??;

    Ok(Json(bookmarks))
}

pub async fn get_course_preference_bookmark(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i64>,
) -> Result<Json<CoursePreferenceBookmarkResponse>, AppError> {
    let conn = state
        .pool
        .get()
        .await
        .map_err(|_| AppError::DatabaseError)?;

    let bookmark = conn
        .interact(move |conn| CoursePreferenceBookmarkService::get_by_id(conn, id))
        .await
        .map_err(|_| AppError::DatabaseError)??;

    Ok(Json(bookmark))
}

pub async fn delete_course_preference_bookmark(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i64>,
) -> Result<StatusCode, AppError> {
    let conn = state
        .pool
        .get()
        .await
        .map_err(|_| AppError::DatabaseError)?;

    conn.interact(move |conn| CoursePreferenceBookmarkService::delete(conn, id))
        .await
        .map_err(|_| AppError::DatabaseError)??;

    Ok(StatusCode::NO_CONTENT)
}
