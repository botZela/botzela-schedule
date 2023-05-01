use routes::router;

mod routes;
mod structs;

#[tokio::main]
async fn main() {
    // build our application with a route
    let app = router();

    let addr = "[::]:8080".parse().unwrap();
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
