use std::sync::Arc;

use axum::{Router, routing::*};

use crate::{
    handler::curriculum_handler::{
        create_curriculum, delete_curriculum, get_curriculum, get_curriculums,
    },
    state::app_state::AppState,
};

pub fn curriculum_router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", post(create_curriculum).get(get_curriculums))
        .route(
            "/{curriculum_id}",
            get(get_curriculum).delete(delete_curriculum),
        )
}
