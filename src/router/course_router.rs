use std::sync::Arc;

use axum::{
    Router,
    routing::{delete, get, patch, post},
};

use crate::{
    handler::course_handler::{
        create_course, delete_course, get_course, get_courses, update_course,
    },
    state::app_state::AppState,
};

pub fn course_router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", post(create_course).get(get_courses))
        .route(
            "/{id}",
            get(get_course).patch(update_course).delete(delete_course),
        )
}
