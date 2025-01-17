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
        .route("/api/schedule", post(schedule::post))
        .fallback_service(ServeDir::new(env!("CLIENT_DIST")))
        .with_state(database)
        .layer(TraceLayer::new_for_http())
}
