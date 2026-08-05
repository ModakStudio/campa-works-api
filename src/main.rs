use std::sync::Arc;

use axum::Router;

use tokio::net::TcpListener;

mod config;
mod db;
mod dto;
mod error;
mod handler;
mod models;
mod repository;
mod router;
mod schema;
mod service;
mod state;
mod utils;

use db::pool::create_pool;
use state::app_state::AppState;

use crate::router::{
    course_assignment_router::course_assignment_router, course_pool_router::course_pool_router,
    course_preference_bookmark_router::course_preference_bookmark_router,
    course_preference_router::course_preference_router, course_router::course_router,
    facility_router::facility_router, major_router::major_router,
    master_course_router::master_course_router, professor_credit_router::professor_credit_router,
    professor_router::professor_router, semester_router::semester_router, user_router::user_router,
};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let pool = create_pool();
    let state = Arc::new(AppState { pool });

    let app = Router::new()
        .nest("/api/users", user_router())
        .nest("/api/professors", professor_router())
        .nest("/api/semesters", semester_router())
        .nest("/api/majors", major_router())
        .nest("/api/master-courses", master_course_router())
        .nest("/api/courses", course_router())
        .nest("/api/course-pools", course_pool_router())
        .nest("/api/course-preferences", course_preference_router())
        .nest("/api/course-assignments", course_assignment_router())
        .nest("/api/professor-credits", professor_credit_router())
        .nest(
            "/api/course-preference-bookmarks",
            course_preference_bookmark_router(),
        )
        .nest("/api/facilities", facility_router())
        .with_state(state);

    let listener = TcpListener::bind("0.0.0.0:8080").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
