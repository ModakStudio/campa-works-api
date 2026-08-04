use std::sync::Arc;

use axum::{Router, routing::*};

use crate::{
    handler::course_preference_handler::{
        create_course_preference, delete_course_preference, get_course_preference,
        get_course_preferences, update_course_preference,
    },
    state::app_state::AppState,
};

pub fn course_preference_router() -> Router<Arc<AppState>> {
    Router::new()
        .route(
            "/",
            post(create_course_preference).get(get_course_preferences),
        )
        .route(
            "/{id}",
            get(get_course_preference)
                .patch(update_course_preference)
                .delete(delete_course_preference),
        )
}
