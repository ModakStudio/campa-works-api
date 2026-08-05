use std::sync::Arc;

use axum::{Router, routing::*};

use crate::{
    handler::course_preference_bookmark_handler::{
        create_course_preference_bookmark, delete_course_preference_bookmark,
        get_course_preference_bookmark, get_course_preference_bookmarks,
    },
    state::app_state::AppState,
};

pub fn course_preference_bookmark_router() -> Router<Arc<AppState>> {
    Router::new()
        .route(
            "/",
            post(create_course_preference_bookmark).get(get_course_preference_bookmarks),
        )
        .route(
            "/{id}",
            get(get_course_preference_bookmark).delete(delete_course_preference_bookmark),
        )
}
