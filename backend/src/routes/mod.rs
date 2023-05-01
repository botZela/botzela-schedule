mod root;
pub mod schedule;

use axum::{
    routing::{get, post},
    Router,
};

pub fn router(database: mongodb::Database) -> Router {
    Router::new()
        .route("/", get(root::root))
        .route("/schedule", post(schedule::post))
        .with_state(database)
}
