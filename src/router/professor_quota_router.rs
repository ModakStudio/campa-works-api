use std::sync::Arc;

use axum::{Router, routing::*};

use crate::{
    handler::professor_quota_handler::{
        create_professor_quota, delete_professor_quota, get_professor_quota, get_professor_quotas,
        update_professor_quota,
    },
    state::app_state::AppState,
};

pub fn professor_quota_router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", post(create_professor_quota).get(get_professor_quotas))
        .route(
            "/{professor_quota_id}",
            get(get_professor_quota)
                .patch(update_professor_quota)
                .delete(delete_professor_quota),
        )
}
