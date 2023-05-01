mod root;

use axum::{routing::get, Router};

pub fn router() -> Router {
    Router::new().route("/", get(root::root))
}
