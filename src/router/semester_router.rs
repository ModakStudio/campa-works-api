use std::sync::Arc;

use axum::{Router, routing::*};

use crate::{
    handler::semester_handler::{
        create_semester, delete_semester, get_semester, get_semesters, update_semester,
    },
    state::app_state::AppState,
};

pub fn semester_router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", post(create_semester).get(get_semesters))
        .route(
            "/{id}",
            get(get_semester)
                .patch(update_semester)
                .delete(delete_semester),
        )
}
