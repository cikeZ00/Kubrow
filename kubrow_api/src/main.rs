// main.rs

extern crate core;

use axum::{Server};
use axum::http::{
    header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
    HeaderValue, Method,
};
use std::net::SocketAddr;
use tower_http::cors::CorsLayer;
use std::thread;

mod api;
mod tools;

#[tokio::main]
async fn main() {
    // Address that server will bind to.
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    // CORS
    let cors = CorsLayer::new()
        .allow_origin(format!("http://{}", addr.to_string()).parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
        .allow_credentials(true)
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE]);

    // Create the router by calling the `routes` function from the api module.
    let app = api::routes::routes()
        .layer(cors);

    // Thread for running background tools
    let thread_handle = thread::spawn(|| {
        loop {
            tokio::runtime::Runtime::new()
                .unwrap()
                .block_on(async {
                    tools::parser::parse_manifest().await;
                    tools::assets::fetch_assets().await;
                    // Adjust sleep duration as needed
                    tokio::time::sleep(tokio::time::Duration::from_secs(86400)).await;
                });
        }
    });

    println!("Starting server on http://{}/", addr.to_string());
    // Use `hyper::server::Server` which is re-exported through `axum::Server` to serve the app.
    Server::bind(&addr)
        // Hyper server takes a make service.
        .serve(app.into_make_service())
        .await
        .unwrap();

    thread_handle.join().expect("Thread panicked");
}

