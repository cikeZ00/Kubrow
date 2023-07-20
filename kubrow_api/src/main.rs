// main.rs

use axum::{Server};
use std::net::SocketAddr;

mod api;

#[tokio::main]
async fn main() {
    // Address that server will bind to.
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    // Create the router by calling the `routes` function from the api module.
    let app = api::routes::routes();

    // Use `hyper::server::Server` which is re-exported through `axum::Server` to serve the app.
    Server::bind(&addr)
        // Hyper server takes a make service.
        .serve(app.into_make_service())
        .await
        .unwrap();
}
