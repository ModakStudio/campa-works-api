use std::sync::Arc;

use axum::{Router, routing::*};

use crate::{
    handler::professor_credit_handler::{
        create_professor_credit, delete_professor_credit, get_professor_credit,
        get_professor_credits, update_professor_credit,
    },
    state::app_state::AppState,
};

pub fn professor_credit_router() -> Router<Arc<AppState>> {
    Router::new()
        .route(
            "/",
            post(create_professor_credit).get(get_professor_credits),
        )
        .route(
            "/{id}",
            get(get_professor_credit)
                .patch(update_professor_credit)
                .delete(delete_professor_credit),
        )
}
