use std::collections::{BTreeMap};
use std::fs::{File, read_dir, create_dir_all, remove_file};
use axum::Json;
use reqwest::{Client, Error, Url};
use serde_json::{to_vec, Value};
use regex::Regex;
use std::io::{Read, Write};
use sha2::{Sha256, Digest};


async fn fetch_data_json(url: &str) -> Result<Json<BTreeMap<String, Value>>, Error> {
    let response = reqwest::get(url).await?;
    let data = response.text().await?;

    // Replace or remove problematic control characters
    let cleaned_data = data.chars().filter(|&c| c.is_ascii() && !c.is_ascii_control()).collect::<String>();

    let manifest: BTreeMap<String, Value> = serde_json::from_str(&cleaned_data).unwrap();
    Ok(Json(manifest))
}

async fn fetch_file(url: &str) -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let url = Url::parse(url)?;
    let response = client.get(url).send().await?;

    if !response.status().is_success() {
        return Err(format!("Request failed with status: {}", response.status()).into());
    }

    if !read_dir("data").is_ok() {
        create_dir_all("data")?;
        create_dir_all("data/manifest")?;
    }

    let content = response.bytes().await?;
    let downloaded_hash = compute_hash(&content);

    if let Ok(mut local_file) = File::open("data/wf.lzma") {
        let mut local_content = Vec::new();
        local_file.read_to_end(&mut local_content)?;
        let local_hash = compute_hash(&local_content);

        if downloaded_hash != local_hash {
            let mut output_file = File::create("data/wf.lzma")?;
            output_file.write_all(&content)?;
            println!("File updated.");
        } else {
            println!("File already up to date.");
        }
    } else {
        let mut output_file = File::create("data/wf.lzma")?;
        output_file.write_all(&content)?;
        println!("File downloaded.");
    }

    Ok(())
}

fn compute_hash(data: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);
    format!("{:x}", hasher.finalize())
}

async fn origin_server_available() -> bool {
    match fetch_file("https://origin.warframe.com/PublicExport/index_en.txt.lzma").await {
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
    let mut decompressed_file: Vec<u8> = Vec::new();
    lzma_rs::lzma_decompress(&mut f, &mut decompressed_file).unwrap();
    let decompressed_str = String::from_utf8(decompressed_file).unwrap();
    
    let re = Regex::new(r"Export.*").unwrap();
    for export in re.find_iter(&decompressed_str) {
        let url = format!("https://content.warframe.com/PublicExport/Manifest/{}", export.as_str());
        let data = fetch_data_json(&url).await.unwrap();

        if let Some(index) = export.as_str().find(".json") {
            let filtered_name = &export.as_str()[..index + 5];
            let local_path = format!("data/manifest/{}", filtered_name);

            if let Ok(mut local_file) = File::open(&local_path) {
                let mut local_content = String::new();
                let mut downloaded_hash = String::new();
                let mut local_hash = String::new();

                local_file.read_to_string(&mut local_content).expect("Unable to read local manifest.");
                let local_json: Result<Value, _> = serde_json::from_str(&*local_content);
                match local_json {
                    Ok(parsed_json) => {
                        downloaded_hash = compute_hash(&to_vec(&data.0).unwrap());
                        local_hash = compute_hash(&to_vec(&parsed_json).unwrap());
                    }
                    Err(_e) => {
                        remove_file(format!("{}", &local_path)).expect("Unable to remove corrupted json");
                        let manifest_file = File::create(format!("data/manifest/{}", filtered_name)).expect("TODO: panic message");
                        serde_json::to_writer_pretty(manifest_file, &data.0).expect("TODO: panic message");
                        println!("Re-Downloaded corrupted file.");

                    }
                }

                if downloaded_hash != local_hash {
                    remove_file(format!("{}", &local_path)).expect("Unable to remove corrupted json!");
                    let manifest_file = File::create(format!("data/manifest/{}", filtered_name)).expect("Failed to create manifest json.");
                    serde_json::to_writer_pretty(manifest_file, &data.0).expect("Failed to write data to manifest json.");
                    println!("Updated: {}", filtered_name);
                    println!("Down hash: {}", downloaded_hash.as_str());
                    println!("Local hash: {}", local_hash.as_str())
                } else {
                    println!("Up to date: {}", filtered_name);
                }
            } else {
                let manifest_file = File::create(format!("data/manifest/{}", filtered_name)).expect("TODO: panic message");
                serde_json::to_writer_pretty(manifest_file, &data.0).expect("TODO: panic message");
                println!("Cached: {}", filtered_name);
            }
        }
    }
}
