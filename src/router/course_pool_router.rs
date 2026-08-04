use std::sync::Arc;

use axum::{Router, routing::*};

use crate::{
    handler::course_pool_handler::{
        create_course_pool, delete_course_pool, get_course_pool, get_course_pools,
    },
    state::app_state::AppState,
};

pub fn course_pool_router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", post(create_course_pool).get(get_course_pools))
        .route("/{id}", get(get_course_pool).delete(delete_course_pool))
}
