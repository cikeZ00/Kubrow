// handlers.rs
use reqwest::Error;
use std::collections::BTreeMap;
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

#[debug_handler]
pub async fn stinki() -> String {
    let a: u8 = 127;
    let b: u8 = 127;
    format!("{}", a + b)
}
