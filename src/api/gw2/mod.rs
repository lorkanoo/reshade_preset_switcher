use crate::addon::Addon;
use crate::api::get_sync;
use function_name::named;
use log::{debug, warn};
use serde::Deserialize;
use std::collections::HashMap;
use std::thread;

const GW2_URL: &str = "https://api.guildwars2.com";

#[derive(Deserialize)]
struct Map {
    id: String,
    name: String,
}

fn map_names_path() -> String {
    format!("{}/v1/map_names.json?lang=en", GW2_URL)
}

#[named]
pub fn fetch_map_names_thread() {
    Addon::threads().push(thread::spawn(|| {
        debug!("[{}] started", function_name!());
        match get_sync(map_names_path()) {
            Ok(response) => match response.json() {
                Ok(json) => {
                    let map_list: Vec<Map> = json;
                    let map_hashmap: HashMap<String, String> = map_list
                        .into_iter()
                        .map(|element| (element.id, element.name))
                        .collect();
                    Addon::lock().context.ui.map_names = map_hashmap;
                }
                Err(_) => warn!("[{}] failed to fetch json", function_name!()),
            },
            Err(_) => warn!("[{}] could not fetch map names", function_name!()),
        }
    }));
}
