// handlers.rs
use reqwest::Error;
use std::collections::BTreeMap;
use std::fs::File;
use std::io::Read;
use axum::body::Body;
use axum::extract::Path;
use axum::http::{Response, StatusCode};
use axum::Json;
use axum_macros::debug_handler;
use serde_json::Value;

pub async fn handler() -> &'static str {
    "Hello, world!"
}

async fn fetch_worldstate() -> Result<Json<BTreeMap<String, Value>>, Error> {
    let url = "https://content.warframe.com/dynamic/worldState.php"; //software gore for the map
    let response = reqwest::get(url).await?;
    let body = response.text().await?;
    let obj: BTreeMap<String, Value> = serde_json::from_str(&body).unwrap(); //.toString()
    Ok(Json(obj))
}

#[debug_handler]
pub async fn world_state() -> Json<BTreeMap<String, Value>> {
    let mut error: BTreeMap<String, Value> = BTreeMap::new();
    match fetch_worldstate().await {
        Ok(data) => data,
        Err(err) => {
            error.insert("Error".to_string(), err.to_string().parse().unwrap());
            Json(error)
        },
    }
}
fn decode_path(encoded: &str) -> String {
    // Replace %2F with /
    encoded.replace("%2F", "/")
}

#[debug_handler]
pub async fn assets_handler(Path(asset_path): Path<String>) -> Result<Response<Body>, StatusCode> {
    // Assuming the images are stored in the "images" directory
    let decoded_path = decode_path(&asset_path);
    let asset_path = format!("data/assets/{}", decoded_path);

    let mut image_file = match File::open(&asset_path) {
        Ok(file) => file,
        Err(_) => return Err(StatusCode::NOT_FOUND),
    };
    let mut image_content = Vec::new();
    if let Err(_) = image_file.read_to_end(&mut image_content) {
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }

    let content_type = if asset_path.contains(".png") {
        "image/png"
    } else if asset_path.contains(".jpg") || asset_path.contains(".jpeg") {
        "image/jpeg"
    } else {
        "application/octet-stream"
    };

    let response = Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", content_type)
        .body(Body::from(image_content))
        .unwrap();

    Ok(response)
}

#[debug_handler]
pub async fn stinki() -> String {
    let a: u8 = 127;
    let b: u8 = 127;
    println!("aaa");
    format!("{}", a + b)
}


