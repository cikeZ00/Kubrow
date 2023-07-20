// api/routes.rs

use axum::{routing::get, Router};
use crate::api::handlers::handler;

pub fn routes() -> Router {
    Router::new().route("/", get(handler))
}   
