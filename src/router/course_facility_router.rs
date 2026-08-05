use std::sync::Arc;

use axum::{Router, routing::*};

use crate::{
    handler::course_facility_handler::{
        create_course_facility, delete_course_facility, get_course_facilities, get_course_facility,
    },
    state::app_state::AppState,
};

pub fn course_facility_router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", post(create_course_facility).get(get_course_facilities))
        .route(
            "/{id}",
            get(get_course_facility).delete(delete_course_facility),
        )
}
