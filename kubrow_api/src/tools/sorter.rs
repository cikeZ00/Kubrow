use serde_json::{Value};
use std::fs::{File};
use std::collections::{BTreeMap};
use axum::Json;

fn write_data(name: &str, data:BTreeMap<String, BTreeMap<String, Value>>) {
    let local_path = format!("data/organised/{}", name);
    let local_file =  File::create(&local_path).expect("Failed to write ExportCustoms.");
    serde_json::to_writer_pretty(local_file, &data).expect("Failed to write data to ExportCustoms json.");
}

// Pls no look, its horrible >:c
pub async fn categorize_data(name: &str, data: Json<BTreeMap<String, Value>>) {
    println!("{}", name);
    match name {
        _ if name.contains("ExportCustoms") => {
            // Handle ExportCustoms case
            let mut transformed_data: BTreeMap<String, BTreeMap<String, Value>> = BTreeMap::new();
            let mut second_data: BTreeMap<String, Value> = BTreeMap::new();

            for (unique_name, _item) in data.iter() {
                if unique_name == "ExportCustoms" {
                    let data = data[unique_name].as_array();
                    for item in data.unwrap() {
                        let item_name: &str = item["uniqueName"].as_str().unwrap();
                        let mut cloned_item = item.clone();
                        cloned_item.as_object_mut().unwrap().remove("uniqueName");
                        second_data.insert(item_name.to_string(), cloned_item);
                        transformed_data.insert(unique_name.to_string(), second_data.clone());
                    }
                }
            }
            write_data(name, transformed_data);
        }
        _ if name.contains("ExportDrones") => {
            // Handle ExportDrones case
            let mut transformed_data: BTreeMap<String, BTreeMap<String, Value>> = BTreeMap::new();
            let mut second_data: BTreeMap<String, Value> = BTreeMap::new();

            for (unique_name, _item) in data.iter() {
                if unique_name == "ExportDrones" {
                    let data = data[unique_name].as_array();
                    for item in data.unwrap() {
                        let item_name: &str = item["uniqueName"].as_str().unwrap();
                        let mut cloned_item = item.clone();
                        cloned_item.as_object_mut().unwrap().remove("uniqueName");
                        second_data.insert(item_name.to_string(), cloned_item);
                        transformed_data.insert(unique_name.to_string(), second_data.clone());
                    }
                }
            }
            write_data(name, transformed_data);

        }
        _ if name.contains("ExportFlavour") => {
            // Handle ExportFlavor case
            let mut transformed_data: BTreeMap<String, BTreeMap<String, Value>> = BTreeMap::new();
            let mut second_data: BTreeMap<String, Value> = BTreeMap::new();

            for (unique_name, _item) in data.iter() {
                if unique_name == "ExportFlavour" {
                    let data = data[unique_name].as_array();
                    for item in data.unwrap() {
                        let item_name: &str = item["uniqueName"].as_str().unwrap();
                        let mut cloned_item = item.clone();
                        cloned_item.as_object_mut().unwrap().remove("uniqueName");
                        second_data.insert(item_name.to_string(), cloned_item);
                        transformed_data.insert(unique_name.to_string(), second_data.clone());
                    }
                }
            }
            write_data(name, transformed_data);

        }
        _ if name.contains("ExportFusionBundles") => {
            // Handle ExportFusionBundles case
            let mut transformed_data: BTreeMap<String, BTreeMap<String, Value>> = BTreeMap::new();
            let mut second_data: BTreeMap<String, Value> = BTreeMap::new();

            for (unique_name, _item) in data.iter() {
                if unique_name == "ExportFusionBundles" {
                    let data = data[unique_name].as_array();
                    for item in data.unwrap() {
                        let item_name: &str = item["uniqueName"].as_str().unwrap();
                        let mut cloned_item = item.clone();
                        cloned_item.as_object_mut().unwrap().remove("uniqueName");
                        second_data.insert(item_name.to_string(), cloned_item);
                        transformed_data.insert(unique_name.to_string(), second_data.clone());
                    }
                }
            }
            write_data(name, transformed_data);

        }
        _ if name.contains("ExportGear") => {
            // Handle ExportGear case
            let mut transformed_data: BTreeMap<String, BTreeMap<String, Value>> = BTreeMap::new();
            let mut second_data: BTreeMap<String, Value> = BTreeMap::new();

            for (unique_name, _item) in data.iter() {
                if unique_name == "ExportGear" {
                    let data = data[unique_name].as_array();
                    for item in data.unwrap() {
                        let item_name: &str = item["uniqueName"].as_str().unwrap();
                        let mut cloned_item = item.clone();
                        cloned_item.as_object_mut().unwrap().remove("uniqueName");
                        second_data.insert(item_name.to_string(), cloned_item);
                        transformed_data.insert(unique_name.to_string(), second_data.clone());
                    }
                }
            }
            write_data(name, transformed_data);

        }
        _ if name.contains("ExportKeys") => {
            // Handle ExportKeys case
            let mut transformed_data: BTreeMap<String, BTreeMap<String, Value>> = BTreeMap::new();
            let mut second_data: BTreeMap<String, Value> = BTreeMap::new();

            for (unique_name, _item) in data.iter() {
                if unique_name == "ExportKeys" {
                    let data = data[unique_name].as_array();
                    for item in data.unwrap() {
                        let item_name: &str = item["uniqueName"].as_str().unwrap();
                        let mut cloned_item = item.clone();
                        cloned_item.as_object_mut().unwrap().remove("uniqueName");
                        second_data.insert(item_name.to_string(), cloned_item);
                        transformed_data.insert(unique_name.to_string(), second_data.clone());
                    }
                }
            }
            write_data(name, transformed_data);

        }
        _ if name.contains("ExportRecipes") => {
            // Handle ExportRecipes case
            let mut transformed_data: BTreeMap<String, BTreeMap<String, Value>> = BTreeMap::new();
            let mut second_data: BTreeMap<String, Value> = BTreeMap::new();

            for (unique_name, _item) in data.iter() {
                if unique_name == "ExportRecipes" {
                    let data = data[unique_name].as_array();
                    for item in data.unwrap() {
                        let item_name: &str = item["uniqueName"].as_str().unwrap();
                        let mut cloned_item = item.clone();
                        cloned_item.as_object_mut().unwrap().remove("uniqueName");
                        second_data.insert(item_name.to_string(), cloned_item);
                        transformed_data.insert(unique_name.to_string(), second_data.clone());
                    }
                }
            }
            write_data(name, transformed_data);

        }
        _ if name.contains("ExportRegions") => {
            // Handle ExportRegions case
            let mut transformed_data: BTreeMap<String, BTreeMap<String, Value>> = BTreeMap::new();
            let mut second_data: BTreeMap<String, Value> = BTreeMap::new();

            for (unique_name, _item) in data.iter() {
                if unique_name == "ExportRegions" {
                    let data = data[unique_name].as_array();
                    for item in data.unwrap() {
                        let item_name: &str = item["uniqueName"].as_str().unwrap();
                        let mut cloned_item = item.clone();
                        cloned_item.as_object_mut().unwrap().remove("uniqueName");
                        second_data.insert(item_name.to_string(), cloned_item);
                        transformed_data.insert(unique_name.to_string(), second_data.clone());
                    }
                }
            }
            write_data(name, transformed_data);

        }
        _ if name.contains("ExportRelicArcane") => {
            // Handle ExportRelicArcane case
            let mut transformed_data: BTreeMap<String, BTreeMap<String, Value>> = BTreeMap::new();
            let mut second_data: BTreeMap<String, Value> = BTreeMap::new();

            for (unique_name, _item) in data.iter() {
                if unique_name == "ExportRelicArcane" {
                    let data = data[unique_name].as_array();
                    for item in data.unwrap() {
                        let item_name: &str = item["uniqueName"].as_str().unwrap();
                        let mut cloned_item = item.clone();
                        cloned_item.as_object_mut().unwrap().remove("uniqueName");
                        second_data.insert(item_name.to_string(), cloned_item);
                        transformed_data.insert(unique_name.to_string(), second_data.clone());
                    }
                }
            }
            write_data(name, transformed_data);

        }
        _ if name.contains("ExportResources") => {
            // Handle ExportResources case
            let mut transformed_data: BTreeMap<String, BTreeMap<String, Value>> = BTreeMap::new();
            let mut second_data: BTreeMap<String, Value> = BTreeMap::new();

            for (unique_name, _item) in data.iter() {
                if unique_name == "ExportResources" {
                    let data = data[unique_name].as_array();
                    for item in data.unwrap() {
                        let item_name: &str = item["uniqueName"].as_str().unwrap();
                        let mut cloned_item = item.clone();
                        cloned_item.as_object_mut().unwrap().remove("uniqueName");
                        second_data.insert(item_name.to_string(), cloned_item);
                        transformed_data.insert(unique_name.to_string(), second_data.clone());
                    }
                }
            }
            write_data(name, transformed_data);

        }
        _ if name.contains("ExportSentinels") => {
            // Handle ExportSentinels case
            let mut transformed_data: BTreeMap<String, BTreeMap<String, Value>> = BTreeMap::new();
            let mut second_data: BTreeMap<String, Value> = BTreeMap::new();

            for (unique_name, _item) in data.iter() {
                if unique_name == "ExportSentinels" {
                    let data = data[unique_name].as_array();
                    for item in data.unwrap() {
                        let item_name: &str = item["uniqueName"].as_str().unwrap();
                        let mut cloned_item = item.clone();
                        cloned_item.as_object_mut().unwrap().remove("uniqueName");
                        second_data.insert(item_name.to_string(), cloned_item);
                        transformed_data.insert(unique_name.to_string(), second_data.clone());
                    }
                }
            }
            write_data(name, transformed_data);

        }
        _ if name.contains("ExportSortieRewards") => {
            // Handle ExportSortieRewards case
            let mut transformed_data: BTreeMap<String, BTreeMap<String, Value>> = BTreeMap::new();
            let mut second_data: BTreeMap<String, Value> = BTreeMap::new();

            for (unique_name, _item) in data.iter() {
                if unique_name == "ExportIntrinsics" {
                    let data = data[unique_name].as_array();
                    for item in data.unwrap() {
                        let item_name: &str = item["name"].as_str().unwrap();
                        let mut cloned_item = item.clone();
                        cloned_item.as_object_mut().unwrap().remove("name");
                        second_data.insert(item_name.to_string(), cloned_item);
                        transformed_data.insert(unique_name.to_string(), second_data.clone());
                    }
                } else if unique_name == "ExportNightwave" {
                    let data = data[unique_name].as_array();
                    for item in data.unwrap() {
                        let item_name: &str = item["uniqueName"].as_str().unwrap();
                        let mut cloned_item = item.clone();
                        cloned_item.as_object_mut().unwrap().remove("uniqueName");
                        second_data.insert(item_name.to_string(), cloned_item);
                        transformed_data.insert(unique_name.to_string(), second_data.clone());
                    }
                } else if unique_name == "ExportIntrinsics" {
                    let data = data[unique_name].as_array();
                    for item in data.unwrap() {
                        let item_name: &str = item["uniqueName"].as_str().unwrap();
                        let mut cloned_item = item.clone();
                        cloned_item.as_object_mut().unwrap().remove("uniqueName");
                        second_data.insert(item_name.to_string(), cloned_item);
                        transformed_data.insert(unique_name.to_string(), second_data.clone());
                    }
                }
            }
            write_data(name, transformed_data);
        }
        _ if name.contains("ExportUpgrades") => {
            // Handle ExportUpgrades case
            let mut transformed_data: BTreeMap<String, BTreeMap<String, Value>> = BTreeMap::new();
            let mut second_data: BTreeMap<String, Value> = BTreeMap::new();

            for (unique_name, _item) in data.iter() {
                if unique_name == "ExportUpgrades" {
                    let data = data[unique_name].as_array();
                    for item in data.unwrap() {
                        let item_name: &str = item["uniqueName"].as_str().unwrap();
                        let mut cloned_item = item.clone();
                        cloned_item.as_object_mut().unwrap().remove("uniqueName");
                        second_data.insert(item_name.to_string(), cloned_item);
                        transformed_data.insert(unique_name.to_string(), second_data.clone());
                    }
                }
            }
            write_data(name, transformed_data);

        }
        _ if name.contains("ExportWarframes") => {
            // Handle ExportWarframes case
            let mut transformed_data: BTreeMap<String, BTreeMap<String, Value>> = BTreeMap::new();
            let mut second_data: BTreeMap<String, Value> = BTreeMap::new();

            for (unique_name, _item) in data.iter() {
                if unique_name == "ExportWarframes" {
                    let data = data[unique_name].as_array();
                    for item in data.unwrap() {
                        let item_name: &str = item["uniqueName"].as_str().unwrap();
                        let mut cloned_item = item.clone();
                        cloned_item.as_object_mut().unwrap().remove("uniqueName");
                        second_data.insert(item_name.to_string(), cloned_item);
                        transformed_data.insert(unique_name.to_string(), second_data.clone());
                    }
                }
            }
            write_data(name, transformed_data);

        }
        _ if name.contains("ExportWeapons") => {
            // Handle ExportWeapons case
            let mut transformed_data: BTreeMap<String, BTreeMap<String, Value>> = BTreeMap::new();
            let mut second_data: BTreeMap<String, Value> = BTreeMap::new();

            for (unique_name, _item) in data.iter() {
                if unique_name == "ExportWeapons" {
                    let data = data[unique_name].as_array();
                    for item in data.unwrap() {
                        let item_name: &str = item["uniqueName"].as_str().unwrap();
                        let mut cloned_item = item.clone();
                        cloned_item.as_object_mut().unwrap().remove("uniqueName");
                        second_data.insert(item_name.to_string(), cloned_item);
                        transformed_data.insert(unique_name.to_string(), second_data.clone());
                    }
                }
            }
            write_data(name, transformed_data);

        }
        _ => {
            println!("Not a filterable json: {}", name)
        }
    }

}
