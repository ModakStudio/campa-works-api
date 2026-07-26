use axum::{
    Router,
    routing::{delete, get, patch, post},
};

use std::sync::Arc;

use crate::{
    handler::professor_handler::{
        create_professor, delete_professor, get_professor, get_professors, update_professor,
    },
    state::app_state::AppState,
};

pub fn professor_router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", post(create_professor).get(get_professors))
        .route(
            "/{id}",
            get(get_professor)
                .patch(update_professor)
                .delete(delete_professor),
        )
}
