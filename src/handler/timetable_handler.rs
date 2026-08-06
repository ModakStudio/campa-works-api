use std::sync::Arc;

use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};

use crate::{
    dto::timetable::{CreateTimetableRequest, TimetableResponse, UpdateTimetableRequest},
    error::app_error::AppError,
    service::timetable_service::TimetableService,
    state::app_state::AppState,
};

pub async fn create_timetable(
    State(state): State<Arc<AppState>>,
    Json(request): Json<CreateTimetableRequest>,
) -> Result<(StatusCode, Json<TimetableResponse>), AppError> {
    let conn = state
        .pool
        .get()
        .await
        .map_err(|_| AppError::DatabaseError)?;

    let timetable = conn
        .interact(move |conn| TimetableService::create(conn, request))
        .await
        .map_err(|_| AppError::DatabaseError)??;

    Ok((StatusCode::CREATED, Json(timetable)))
}

pub async fn get_timetables(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<TimetableResponse>>, AppError> {
    let conn = state
        .pool
        .get()
        .await
        .map_err(|_| AppError::DatabaseError)?;

    let timetables = conn
        .interact(move |conn| TimetableService::get_all(conn))
        .await
        .map_err(|_| AppError::DatabaseError)??;

    Ok(Json(timetables))
}

pub async fn get_timetable(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i64>,
) -> Result<Json<TimetableResponse>, AppError> {
    let conn = state
        .pool
        .get()
        .await
        .map_err(|_| AppError::DatabaseError)?;

    let timetable = conn
        .interact(move |conn| TimetableService::get_by_id(conn, id))
        .await
        .map_err(|_| AppError::DatabaseError)??;

    Ok(Json(timetable))
}

pub async fn update_timetable(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i64>,
    Json(request): Json<UpdateTimetableRequest>,
) -> Result<Json<TimetableResponse>, AppError> {
    let conn = state
        .pool
        .get()
        .await
        .map_err(|_| AppError::DatabaseError)?;

    let timetable = conn
        .interact(move |conn| TimetableService::update(conn, id, request))
        .await
        .map_err(|_| AppError::DatabaseError)??;

    Ok(Json(timetable))
}

pub async fn delete_timetable(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i64>,
) -> Result<StatusCode, AppError> {
    let conn = state
        .pool
        .get()
        .await
        .map_err(|_| AppError::DatabaseError)?;

    conn.interact(move |conn| TimetableService::delete(conn, id))
        .await
        .map_err(|_| AppError::DatabaseError)??;

    Ok(StatusCode::NO_CONTENT)
}
