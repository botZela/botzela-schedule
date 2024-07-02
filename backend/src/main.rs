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

    let addr = "[::]:8080";
    tracing::debug!("listening on {}..", addr);
    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
