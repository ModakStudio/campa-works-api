use std::sync::Arc;

use axum::{Router, routing::*};

use crate::{
    handler::course_handler::{
        create_course, create_courses_auto, delete_course, get_course, get_courses, update_course,
    },
    state::app_state::AppState,
};

pub fn course_router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", post(create_course).get(get_courses))
        .route(
            "/{course_id}",
            get(get_course).patch(update_course).delete(delete_course),
        )
        .route("/auto/{semester_id}", post(create_courses_auto))
}
