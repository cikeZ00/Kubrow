use std::fs::{create_dir_all, File, read_dir};
use std::io::Write;
use std::path::Path;
use reqwest::Client;
use serde_json::Value;

pub(crate) async fn fetch_assets(){
    if !read_dir("data/assets").is_ok() {
        create_dir_all("data/assets").expect("Unable to create directory for assets.");
    }

    if let Ok(local_file) = File::open("data/manifest/ExportManifest.json") {
        let local_json: Result<Value, _> = serde_json::from_reader(local_file);

        if let Ok(local_json) = local_json {
            if let Some(json_array) = local_json["Manifest"].as_array() {
                for item in json_array {
                    if let Some(texture_url) = item["textureLocation"].as_str() {
                        let target_path = format!("data/assets/{}", texture_url);
                        if !File::open(&target_path).is_ok() {
                            let client = Client::new();
                            let url = format!("https://content.warframe.com/PublicExport/{}", texture_url);
                            let response = client.get(url).send().await;
                            let body = response.unwrap().bytes().await.expect("Failed!");

                            if let Some(parent_dir) = Path::new(&target_path).parent() {
                                if let Err(err) = create_dir_all(parent_dir) {
                                    eprintln!("Failed to create directories: {}", err);
                                    return;
                                }
                            }

                            if let Ok(mut local_file) = File::create(&target_path) {
                                if let Err(err) = local_file.write_all(&body) {
                                    eprintln!("Failed to write data to file: {}", err);
                                }
                            } else {
                                eprintln!("Failed to create file.");
                            }
                        }
                    }
                }
                println!("Finished fetching assets.");
            }
        } else {
            eprintln!("Failed to parse JSON content.");
        }
    } else {
        eprintln!("Unable to open ExportManifest file.");
    }

}