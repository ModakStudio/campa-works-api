use std::sync::Arc;

use axum::{Router, routing::*};

use crate::{
    handler::facility_handler::{
        create_facility, delete_facility, get_facilities, get_facility, update_facility,
    },
    state::app_state::AppState,
};

pub fn facility_router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", post(create_facility).get(get_facilities))
        .route(
            "/{id}",
            get(get_facility)
                .patch(update_facility)
                .delete(delete_facility),
        )
}
