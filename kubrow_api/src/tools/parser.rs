use std::collections::BTreeMap;
use std::fs::File;
use axum::Json;
use reqwest::{Client, Error, Url};
use serde_json::Value;
use regex::Regex;
use std::io::prelude::*;
use lzma_rs::lzma_decompress;
use tokio::io::AsyncReadExt;
use xz2::read::XzDecoder;
use tempfile::NamedTempFile;

use std::io::{self, Write};


async fn fetch_data_json(url: &str) -> Result<Json<BTreeMap<String, Value>>, Error> {
    println!("{}", url);
    let response = reqwest::get(url).await?;
    let data = response.text().await?;
    let manifest: BTreeMap<String, Value> = serde_json::from_str(&data).unwrap();
    Ok(Json(manifest))
}

async fn fetch_data(url: &str) -> Result<Vec<u8>, Error> {
    let client = reqwest::Client::builder()
        .danger_accept_invalid_certs(true)
        .build()?;

    let response = client.get(url).send().await?;
    let body = response.bytes().await?;

    Ok(body.to_vec())
}

async fn uncompress_lzma(data: Vec<u8>) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    // Create a temporary file to store the LZMA data
    let mut temp_file = NamedTempFile::new()?;

    // Write the LZMA data to the temporary file
    temp_file.write_all(&data)?;

    // Create an XzDecoder to decompress the LZMA data from the temporary file
    let mut f = std::io::BufReader::new(temp_file);

    // Create a buffer to store the decompressed data
    let mut decompressed_data = Vec::new();
    lzma_decompress(&mut f, &mut decompressed_data).expect("TODO: panic message");

    Ok(decompressed_data)
}

async fn fetch_file(url: &str) -> Result<File, Box<dyn std::error::Error>> {
    // Create a reqwest client
    let client = Client::new();

    // Parse the URL
    let url = Url::parse(url)?;

    // Send the GET request and await the response
    let response = client.get(url).send().await?;

    // Check if the request was successful (status code 2xx)
    if !response.status().is_success() {
        return Err(format!("Request failed with status: {}", response.status()).into());
    }
    let mut tmpfile = tempfile::tempfile().unwrap();

    // Open the output file for writing
    //let mut output_file = std::fs::File::create(output_path)?;

    // Download the file and write it to the output file
    let content = response.bytes().await?;
    tmpfile.write_all(&content)?;

    Ok(tmpfile)
}

async fn origin_server_available() -> bool {
    match fetch_data("https://origin.warframe.com/PublicExport/index_en.txt.lzma").await {
        Ok(_) => true,
        Err(_) => false,
    }
}

pub async fn parse_manifest() {
    let origin_server_available = origin_server_available().await;
    let locale = "en";

    let origin = format!(
        "https://{}/PublicExport/index_{}.txt.lzma",
        if origin_server_available { "origin.warframe.com" } else { "content.warframe.com" },
        locale
    );

    let file = fetch_file(&*origin).await.expect("TODO: panic message");
    let mut f = std::io::BufReader::new(file);

    let mut decomp: Vec<u8> = Vec::new();
    //lzma_rs::lzma_decompress(&mut f, &mut decomp).unwrap();

    let mut decompressor = XzDecoder::new(f);
    let mut contents = String::new();
    decompressor.read_to_string(&mut contents).unwrap();
    println!("{}", contents);

    let decompressed_str = String::from_utf8(decomp).unwrap();
    println!("{}", decompressed_str);

    //let manifest_regex = r"ExportManifest.*";
    //let re = Regex::new(manifest_regex).unwrap();
    //let manifest_endpoint = re.find(&decompressed_str).unwrap().as_str();

    //let manifest_url = format!("https://content.warframe.com/PublicExport/Manifest/{}", manifest_endpoint);
    //let manifest_data = fetch_data_json(&manifest_url).await.unwrap();
    //println!("Manifest Data: {:?}", manifest_data);
}
