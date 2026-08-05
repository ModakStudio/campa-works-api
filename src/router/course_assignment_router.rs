use std::sync::Arc;

use axum::{Router, routing::*};

use crate::{
    handler::course_assignment_handler::{
        create_course_assignment, delete_course_assignment, get_course_assignment,
        get_course_assignments,
    },
    state::app_state::AppState,
};

pub fn course_assignment_router() -> Router<Arc<AppState>> {
    Router::new()
        .route(
            "/",
            post(create_course_assignment).get(get_course_assignments),
        )
        .route(
            "/{id}",
            get(get_course_assignment).delete(delete_course_assignment),
        )
}
