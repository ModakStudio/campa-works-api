use std::sync::Arc;

use axum::{Router, routing::*};

use crate::{
    handler::course_curriculum_handler::{
        create_course_curriculum, delete_course_curriculum, get_course_curriculum,
        get_course_curriculums,
    },
    state::app_state::AppState,
};

pub fn course_curriculum_router() -> Router<Arc<AppState>> {
    Router::new()
        .route(
            "/",
            post(create_course_curriculum).get(get_course_curriculums),
        )
        .route(
            "/{course_curriculum_id}",
            get(get_course_curriculum).delete(delete_course_curriculum),
        )
}
