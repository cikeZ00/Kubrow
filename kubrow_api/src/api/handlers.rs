// handlers.rs
use reqwest::Error;
use std::collections::HashMap;
use axum::Json;
use axum_macros::debug_handler;
use serde_json::Value;

pub async fn handler() -> &'static str {
    "Hello, world!"
}

async fn fetch_worldstate() -> Result<Json<HashMap<String, Value>>, Error> {
    let url = "https://content.warframe.com/dynamic/worldState.php";
    let response = reqwest::get(url).await?;
    let body = response.text().await?;
    let obj: HashMap<String, Value> = serde_json::from_str(&body).unwrap();
    Ok(Json(obj))
}

#[debug_handler]
pub async fn world_state() -> Json<HashMap<String, Value>> {
    let mut error: HashMap<String, Value> = HashMap::new();
    match fetch_worldstate().await {
        Ok(data) => data,
        Err(err) => {
            error.insert("Error".to_string(), err.to_string().parse().unwrap());
            Json(error)
        },
    }
}