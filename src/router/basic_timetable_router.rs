use std::sync::Arc;

use axum::{Router, routing::*};

use crate::{
    handler::basic_timetable_handler::{
        create_basic_timetable, delete_basic_timetable, get_basic_timetable, get_basic_timetables,
        update_basic_timetable,
    },
    state::app_state::AppState,
};

pub fn basic_timetable_router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", post(create_basic_timetable).get(get_basic_timetables))
        .route(
            "/{model_id}",
            get(get_basic_timetable)
                .patch(update_basic_timetable)
                .delete(delete_basic_timetable),
        )
}
