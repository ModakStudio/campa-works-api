use std::sync::Arc;

use axum::{Router, routing::*};

use crate::{
    handler::user_handler::{create_user, delete_user, get_user, get_users, update_user},
    state::app_state::AppState,
};

pub fn user_router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", post(create_user).get(get_users))
        .route(
            "/{id}",
            get(get_user).patch(update_user).delete(delete_user),
        )
}
