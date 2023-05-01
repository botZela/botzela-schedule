use mongodb::{options::ClientOptions, Client};
use routes::router;

mod routes;
mod structs;

#[tokio::main]
async fn main() {
    let client_options = ClientOptions::parse(std::env::var("MONGO_URI").unwrap())
        .await
        .unwrap();

    // Get a handle to the deployment.
    let database = Client::with_options(client_options)
        .unwrap()
        .database("botZela");

    let app = router(database);

    let addr = "[::]:8080".parse().unwrap();
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
