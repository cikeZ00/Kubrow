// main.rs

use axum::{Server};
use axum::http::{
    header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
    HeaderValue, Method,
};
use std::net::SocketAddr;
use tower_http::cors::CorsLayer;

mod api;

#[tokio::main]
async fn main() {
    // Address that server will bind to.
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    // CORS
    let cors = CorsLayer::new()
        .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
        .allow_credentials(true)
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE]);

    // Create the router by calling the `routes` function from the api module.
    let app = api::routes::routes()
        .layer(cors);

    // Use `hyper::server::Server` which is re-exported through `axum::Server` to serve the app.
    Server::bind(&addr)
        // Hyper server takes a make service.
        .serve(app.into_make_service())
        .await
        .unwrap();
}

