use serde_json::{Value};
use std::fs::{File, remove_file};
use std::collections::{BTreeMap};
use axum::Json;
use serde::Serialize;

fn write_data<K, V>(name: &str, data: BTreeMap<K, V>)
    where
        K: Serialize,
        V: Serialize,
{
    let local_path = format!("data/organised/{}", name);

    if let Ok(_local_file) = File::open(&local_path) {
        remove_file(format!("{}", &local_path)).expect(&*format!("Unable to remove outdated {} json!", name));
    }
    let local_file = File::create(&local_path).expect("Failed to write data.");
    serde_json::to_writer_pretty(local_file, &data).expect("Failed to write data to JSON.");
}

// Pls no look, its horrible >:c
// *Lord forgive me, for I have sinned*
// TODO: Un-fuck whatever the fuck I wrote here
pub async fn categorize_data(name: &str, data: Json<BTreeMap<String, Value>>) {
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


            for (unique_name, _item) in data.iter() {
                if unique_name == "ExportIntrinsics" {
                    let mut transformed_data = BTreeMap::new();
                    let data2 = data.0[unique_name].as_array();
                    let mut second_data = BTreeMap::new();
                    for item in data2.unwrap() {
                        let item_name: &str = item["name"].as_str().unwrap();
                        let mut cloned_item = item.clone();
                        cloned_item.as_object_mut().unwrap().remove("name");
                        second_data.insert(item_name.to_string(), cloned_item);
                        transformed_data.insert(unique_name.to_string(), second_data.clone());
                    }
                    write_data(format!("{}.json",unique_name).as_str(), transformed_data);

                } else if unique_name == "ExportNightwave" {
                    let mut transformed_data = BTreeMap::new();
                    let data2 = data.0[unique_name].as_object().unwrap();
                    let mut second_data: BTreeMap<String, BTreeMap<String, Value>> = BTreeMap::new();
                    let mut third_data = BTreeMap::new();
                    for _item in data2 {
                        for key in data.0[unique_name]["challenges"].as_array().unwrap() {
                            let item_name = key["uniqueName"].as_str().unwrap();
                            let mut cloned_item = key.clone();
                            cloned_item.as_object_mut().unwrap().remove("uniqueName");
                            third_data.insert(item_name.to_string(), cloned_item);
                            second_data.insert("challenges".parse().unwrap(), third_data.clone());
                            transformed_data.insert(unique_name.to_string(), second_data.clone());
                        }
                    }
                    write_data(format!("{}.json",unique_name).as_str(), transformed_data);

                } else if unique_name == "ExportOther" {
                    let mut transformed_data = BTreeMap::new();
                    let data = data[unique_name].as_array().clone();
                    let mut second_data = BTreeMap::new();
                    for item in data.unwrap() {
                        let item_name: &str = item["uniqueName"].as_str().unwrap();
                        let mut cloned_item = item.clone();
                        cloned_item.as_object_mut().unwrap().remove("uniqueName");
                        second_data.insert(item_name.to_string(), cloned_item);
                        transformed_data.insert(unique_name.to_string(), second_data.clone());
                    }
                    write_data(format!("{}.json",unique_name).as_str(), transformed_data);

                } else if unique_name == "ExportRailjack" {
                    let mut transformed_data = BTreeMap::new();
                    let data2 = data.0[unique_name].as_object().unwrap();
                    let mut second_data: BTreeMap<String, BTreeMap<String, Value>> = BTreeMap::new();
                    let mut third_data = BTreeMap::new();
                    for _item in data2 {
                        for key in data.0[unique_name]["nodes"].as_array().unwrap() {
                            let item_name = key["uniqueName"].as_str().unwrap();
                            let mut cloned_item = key.clone();
                            cloned_item.as_object_mut().unwrap().remove("uniqueName");
                            third_data.insert(item_name.to_string(), cloned_item);
                            second_data.insert("nodes".parse().unwrap(), third_data.clone());
                            transformed_data.insert(unique_name.to_string(), second_data.clone());
                        }
                    }
                    write_data(format!("{}.json",unique_name).as_str(), transformed_data);

                } else if unique_name == "ExportSortieRewards" {
                    let mut transformed_data = BTreeMap::new();
                    let data = data[unique_name].as_array().clone();
                    let mut second_data = BTreeMap::new();
                    for item in data.unwrap() {
                        let item_name: &str = item["rewardName"].as_str().unwrap();
                        let mut cloned_item = item.clone();
                        cloned_item.as_object_mut().unwrap().remove("rewardName");
                        second_data.insert(item_name.to_string(), cloned_item);
                        transformed_data.insert(unique_name.to_string(), second_data.clone());
                    }
                    write_data(format!("{}.json",unique_name).as_str(), transformed_data);

                }
            }
        }
        _ if name.contains("ExportUpgrades") => {
            // Handle ExportUpgrades case
            for (unique_name, _item) in data.iter() {
                if unique_name == "ExportAvionics" {
                    let mut transformed_data = BTreeMap::new();
                    let data2 = data.0[unique_name].as_array();
                    let mut second_data = BTreeMap::new();
                    for item in data2.unwrap() {
                        let item_name: &str = item["uniqueName"].as_str().unwrap();
                        let mut cloned_item = item.clone();
                        cloned_item.as_object_mut().unwrap().remove("uniqueName");
                        second_data.insert(item_name.to_string(), cloned_item);
                        transformed_data.insert(unique_name.to_string(), second_data.clone());
                    }
                    write_data(format!("{}.json", unique_name).as_str(), transformed_data);

                } else if unique_name == "ExportFocusUpgrades" {
                    let mut transformed_data = BTreeMap::new();
                    let data2 = data.0[unique_name].as_array();
                    let mut second_data = BTreeMap::new();
                    for item in data2.unwrap() {
                        let item_name: &str = item["uniqueName"].as_str().unwrap();
                        let mut cloned_item = item.clone();
                        cloned_item.as_object_mut().unwrap().remove("uniqueName");
                        second_data.insert(item_name.to_string(), cloned_item);
                        transformed_data.insert(unique_name.to_string(), second_data.clone());
                    }
                    write_data(format!("{}.json", unique_name).as_str(), transformed_data);

                } else if unique_name == "ExportModSet" {
                    let mut transformed_data = BTreeMap::new();
                    let data2 = data.0[unique_name].as_array();
                    let mut second_data = BTreeMap::new();
                    for item in data2.unwrap() {
                        let item_name: &str = item["uniqueName"].as_str().unwrap();
                        let mut cloned_item = item.clone();
                        cloned_item.as_object_mut().unwrap().remove("uniqueName");
                        second_data.insert(item_name.to_string(), cloned_item);
                        transformed_data.insert(unique_name.to_string(), second_data.clone());
                    }
                    write_data(format!("{}.json", unique_name).as_str(), transformed_data);

                } else if unique_name == "ExportUpgrades" {
                    let mut transformed_data = BTreeMap::new();
                    let data2 = data.0[unique_name].as_array();
                    let mut second_data = BTreeMap::new();
                    for item in data2.unwrap() {
                        let item_name: &str = item["uniqueName"].as_str().unwrap();
                        let mut cloned_item = item.clone();
                        cloned_item.as_object_mut().unwrap().remove("uniqueName");
                        second_data.insert(item_name.to_string(), cloned_item);
                        transformed_data.insert(unique_name.to_string(), second_data.clone());
                    }
                    write_data(format!("{}.json", unique_name).as_str(), transformed_data);
                }
            }

        }
        _ if name.contains("ExportWarframes") => {
            // Handle ExportWarframes case
            for (unique_name, _item) in data.iter() {
                if unique_name == "ExportAbilities" {
                    let mut transformed_data = BTreeMap::new();
                    let data2 = data.0[unique_name].as_array();
                    let mut second_data = BTreeMap::new();
                    for item in data2.unwrap() {
                        let item_name: &str = item["abilityUniqueName"].as_str().unwrap();
                        let mut cloned_item = item.clone();
                        cloned_item.as_object_mut().unwrap().remove("abilityUniqueName");
                        second_data.insert(item_name.to_string(), cloned_item);
                        transformed_data.insert(unique_name.to_string(), second_data.clone());
                    }
                    write_data(format!("{}.json", unique_name).as_str(), transformed_data);

                } else if unique_name == "ExportWarframes" {
                    let mut transformed_data = BTreeMap::new();
                    let data2 = data.0[unique_name].as_array();
                    let mut second_data = BTreeMap::new();
                    for item in data2.unwrap() {
                        let item_name: &str = item["uniqueName"].as_str().unwrap();
                        let mut cloned_item = item.clone();
                        cloned_item.as_object_mut().unwrap().remove("uniqueName");
                        second_data.insert(item_name.to_string(), cloned_item);
                        transformed_data.insert(unique_name.to_string(), second_data.clone());
                    }
                    write_data(format!("{}.json", unique_name).as_str(), transformed_data);
                }
            }

        }
        _ if name.contains("ExportWeapons") => {
            // Handle ExportWeapons case

            for (unique_name, _item) in data.iter() {
                if unique_name == "ExportRailjackWeapons" {
                    let mut transformed_data = BTreeMap::new();
                    let data2 = data.0[unique_name].as_array();
                    let mut second_data = BTreeMap::new();
                    for item in data2.unwrap() {
                        let item_name: &str = item["uniqueName"].as_str().unwrap();
                        let mut cloned_item = item.clone();
                        cloned_item.as_object_mut().unwrap().remove("uniqueName");
                        second_data.insert(item_name.to_string(), cloned_item);
                        transformed_data.insert(unique_name.to_string(), second_data.clone());
                    }
                    write_data(format!("{}.json", unique_name).as_str(), transformed_data);
                } else if unique_name == "ExportWeapons" {
                    let mut transformed_data = BTreeMap::new();
                    let data2 = data.0[unique_name].as_array();
                    let mut second_data = BTreeMap::new();
                    for item in data2.unwrap() {
                        let item_name: &str = item["uniqueName"].as_str().unwrap();
                        let mut cloned_item = item.clone();
                        cloned_item.as_object_mut().unwrap().remove("uniqueName");
                        second_data.insert(item_name.to_string(), cloned_item);
                        transformed_data.insert(unique_name.to_string(), second_data.clone());
                    }
                    write_data(format!("{}.json", unique_name).as_str(), transformed_data);
                }
            }
        }
        _ if name.contains("ExportManifest") => {
            // Handle ExportSentinels case
            let mut transformed_data: BTreeMap<String, BTreeMap<String, Value>> = BTreeMap::new();
            let mut second_data: BTreeMap<String, Value> = BTreeMap::new();

            for (unique_name, _item) in data.iter() {
                if unique_name == "Manifest" {
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
