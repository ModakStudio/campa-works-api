use std::sync::Arc;

use axum::{Router, routing::*};

use crate::{
    handler::classroom_handler::{
        create_classroom, delete_classroom, get_classroom, get_classrooms, update_classroom,
    },
    state::app_state::AppState,
};

pub fn classroom_router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", post(create_classroom).get(get_classrooms))
        .route(
            "/{id}",
            get(get_classroom)
                .patch(update_classroom)
                .delete(delete_classroom),
        )
}
