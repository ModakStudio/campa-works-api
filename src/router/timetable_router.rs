use std::sync::Arc;

use axum::{Router, routing::*};

use crate::{
    handler::timetable_handler::{
        create_auto_in_new_semester, create_timetable, delete_timetable, get_timetable,
        get_timetables, update_timetable,
    },
    state::app_state::AppState,
};

pub fn timetable_router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", post(create_timetable).get(get_timetables))
        .route(
            "/{timetable_id}",
            get(get_timetable)
                .patch(update_timetable)
                .delete(delete_timetable),
        )
        .route("/auto/{new_semester_id}", post(create_auto_in_new_semester))
}
