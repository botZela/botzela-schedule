use axum::{routing::get, Router};

#[tokio::main]
async fn main() {
    // build our application with a route
    let app = Router::new().route("/", get(root));

    let addr = "[::]:8080".parse().unwrap();
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// basic handler that responds with a static string
async fn root() -> &'static str {
    "Hello, World!"
}
