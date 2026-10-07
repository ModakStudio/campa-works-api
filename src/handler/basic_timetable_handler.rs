use std::sync::Arc;

use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};

use crate::{
    dto::basic_timetable::{
        BasicTimetableResponse, CreateBasicTimetableRequest, UpdateBasicTimetableRequest,
    },
    error::app_error::AppError,
    service::basic_timetable_service::BasicTimetableService,
    state::app_state::AppState,
};

pub async fn get_basic_timetables(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<BasicTimetableResponse>>, AppError> {
    let conn = state
        .pool
        .get()
        .await
        .map_err(|_| AppError::DatabaseError)?;
    let models = conn
        .interact(BasicTimetableService::get_all)
        .await
        .map_err(|_| AppError::DatabaseError)??;

    Ok(Json(models))
}

pub async fn get_basic_timetable(
    State(state): State<Arc<AppState>>,
    Path(model_id): Path<i64>,
) -> Result<Json<BasicTimetableResponse>, AppError> {
    let conn = state
        .pool
        .get()
        .await
        .map_err(|_| AppError::DatabaseError)?;
    let model = conn
        .interact(move |conn| BasicTimetableService::get_by_id(conn, model_id))
        .await
        .map_err(|_| AppError::DatabaseError)??;

    Ok(Json(model))
}

pub async fn create_basic_timetable(
    State(state): State<Arc<AppState>>,
    Json(request): Json<CreateBasicTimetableRequest>,
) -> Result<(StatusCode, Json<BasicTimetableResponse>), AppError> {
    let conn = state
        .pool
        .get()
        .await
        .map_err(|_| AppError::DatabaseError)?;
    let model = conn
        .interact(move |conn| BasicTimetableService::create(conn, request))
        .await
        .map_err(|_| AppError::DatabaseError)??;

    Ok((StatusCode::CREATED, Json(model)))
}

pub async fn update_basic_timetable(
    State(state): State<Arc<AppState>>,
    Path(model_id): Path<i64>,
    Json(request): Json<UpdateBasicTimetableRequest>,
) -> Result<Json<BasicTimetableResponse>, AppError> {
    let conn = state
        .pool
        .get()
        .await
        .map_err(|_| AppError::DatabaseError)?;
    let model = conn
        .interact(move |conn| BasicTimetableService::update(conn, model_id, request))
        .await
        .map_err(|_| AppError::DatabaseError)??;

    Ok(Json(model))
}

pub async fn delete_basic_timetable(
    State(state): State<Arc<AppState>>,
    Path(model_id): Path<i64>,
) -> Result<StatusCode, AppError> {
    let conn = state
        .pool
        .get()
        .await
        .map_err(|_| AppError::DatabaseError)?;
    conn.interact(move |conn| BasicTimetableService::delete(conn, model_id))
        .await
        .map_err(|_| AppError::DatabaseError)??;

    Ok(StatusCode::NO_CONTENT)
}
