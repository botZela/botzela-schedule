mod root;
pub mod schedule;

use axum::{
    routing::{get, post},
    Router,
};
use tower_http::services::ServeDir;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

pub fn router(database: mongodb::Database) -> Router {
    // initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "backend=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    Router::new()
        .route("/hello", get(root::root))
        .nest_service("/", ServeDir::new("dist"))
        .route("/schedule", post(schedule::post))
        .with_state(database)
        .layer(TraceLayer::new_for_http())
}
