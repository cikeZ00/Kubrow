use std::collections::BTreeMap;
use std::fs::{DirBuilder, File, read_dir};
use axum::Json;
use reqwest::{Client, Error, Url};
use serde_json::{to_vec, Value};
use regex::Regex;
use std::io::Write;


async fn fetch_data_json(url: &str) -> Result<Json<BTreeMap<String, Value>>, Error> {
    println!("{}", url);
    let response = reqwest::get(url).await?;
    let data = response.text().await?;

    // Replace or remove problematic control characters
    let cleaned_data = data.chars().filter(|&c| c.is_ascii() && !c.is_ascii_control()).collect::<String>();

    let manifest: BTreeMap<String, Value> = serde_json::from_str(&cleaned_data).unwrap();
    Ok(Json(manifest))
}

async fn fetch_data(url: &str) -> Result<Vec<u8>, Error> {
    let client = Client::builder()
        .danger_accept_invalid_certs(true)
        .build()?;

    let response = client.get(url).send().await?;
    let body = response.bytes().await?;

    Ok(body.to_vec())
}

async fn fetch_file(url: &str) -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let url = Url::parse(url)?;
    let response = client.get(url).send().await?;

    if !response.status().is_success() {
        return Err(format!("Request failed with status: {}", response.status()).into());
    }

    if !read_dir("data").is_ok() {
        DirBuilder::new().recursive(true).create("data").expect("TODO: panic message");
        DirBuilder::new().recursive(true).create("data/manifest").expect("TODO: panic message");
    }
    let mut output_file = std::fs::File::create("data/wf.lzma")?;
    let content = response.bytes().await?;
    output_file.write_all(&content)?;
    Ok(())
}

async fn origin_server_available() -> bool {
    match fetch_data("https://origin.warframe.com/PublicExport/index_en.txt.lzma").await {
        Ok(_) => true,
        Err(_) => false,
    }
}

// TODO: Optimize, dont unwrap everything
pub async fn parse_manifest() {
    let origin_server_available = origin_server_available().await;
    let locale = "en";

    let origin = format!(
        "https://{}/PublicExport/index_{}.txt.lzma",
        if origin_server_available { "origin.warframe.com" } else { "content.warframe.com" },
        locale
    );

    // Fetch compressed manifest from warframe servers
    fetch_file(&*origin).await.expect("");

    // Decompress the manifest we fetched
    let filename = "data/wf.lzma";
    let mut f = std::io::BufReader::new(File::open(filename).unwrap());
    let mut decomp: Vec<u8> = Vec::new();
    lzma_rs::lzma_decompress(&mut f, &mut decomp).unwrap();
    let decompressed_str = String::from_utf8(decomp).unwrap();

    //let manifest_regex = r"ExportManifest.*";
    //let parse_regex = r"Export.*";
    let re = Regex::new(r"Export.*").unwrap();
    for export in re.find_iter(&decompressed_str) {
        let url = format!("https://content.warframe.com/PublicExport/Manifest/{}", export.as_str());
        let data = fetch_data_json(&url).await.unwrap();

        if let Some(index) = export.as_str().find(".json") {
            let filtered_name = &export.as_str()[..index + 5];
            let manifest_file = File::create(format!("data/manifest/{}", filtered_name)).expect("TODO: panic message");
            serde_json::to_writer(manifest_file, &data.0).expect("TODO: panic message")
        }

    }
}
