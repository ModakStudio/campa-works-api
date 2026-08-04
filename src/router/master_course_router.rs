use std::sync::Arc;

use axum::{
    Router,
    routing::{delete, get, patch, post},
};

use crate::{
    handler::master_course_handler::{
        create_master_course, delete_master_course, get_master_course, get_master_courses,
        update_master_course,
    },
    state::app_state::AppState,
};

pub fn master_course_router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", post(create_master_course).get(get_master_courses))
        .route(
            "/{id}",
            get(get_master_course)
                .patch(update_master_course)
                .delete(delete_master_course),
        )
}
