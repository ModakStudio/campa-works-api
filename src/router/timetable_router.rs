use std::sync::Arc;

use axum::{Router, routing::*};

use crate::{
    handler::timetable_handler::{
        create_timetable, delete_timetable, get_timetable, get_timetables, update_timetable,
    },
    state::app_state::AppState,
};

pub fn timetable_router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", post(create_timetable).get(get_timetables))
        .route(
            "/{id}",
            get(get_timetable)
                .patch(update_timetable)
                .delete(delete_timetable),
        )
}
