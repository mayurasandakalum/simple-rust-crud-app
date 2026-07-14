use axum::{
    routing::{delete, get, post, put},
    Router,
};
use crate::models::task::Db;
use crate::handlers;

pub fn create_router(db: Db) -> Router {
    Router::new()
        .route("/tasks", post(handlers::task::create_task))
        .route("/tasks", get(handlers::task::list_tasks))
        .route("/tasks/:id", get(handlers::task::get_task))
        .route("/tasks/:id", put(handlers::task::update_task))
        .route("/tasks/:id", delete(handlers::task::delete_task))
        .with_state(db)
}
