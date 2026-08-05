use std::sync::Arc;

use axum::{Router, routing::*};

use crate::{
    handler::classroom_facility_handler::{
        create_classroom_facility, delete_classroom_facility, get_classroom_facility,
        get_classroom_facilities,
    },
    state::app_state::AppState,
};

pub fn classroom_facility_router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", post(create_classroom_facility).get(get_classroom_facilities))
        .route(
            "/{id}",
            get(get_classroom_facility).delete(delete_classroom_facility),
        )
}