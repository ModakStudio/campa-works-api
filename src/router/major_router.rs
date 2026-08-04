use std::sync::Arc;

use axum::{Router, routing::*};

use crate::{
    handler::major_handler::{create_major, delete_major, get_major, get_majors, update_major},
    state::app_state::AppState,
};

pub fn major_router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", post(create_major).get(get_majors))
        .route(
            "/{id}",
            get(get_major).patch(update_major).delete(delete_major),
        )
}
