extern crate dirs;
extern crate reqwest;
extern crate serde_json;

const MAX_REQUESTS: u32 = 3;

fn main() {
    let mut ids: Vec<String> = std::env::args().skip(1).collect();

    if ids.len() == 0 {
        ids = parse_appid_directories();
    }

    use std::collections::BTreeMap;
    let mut map: BTreeMap<String, String> = BTreeMap::new();
    for id in ids.iter() {
        match query_appid(id) {
            Some(response) => match serde_json::from_str(&response) {
                Ok(json) => { 
                    match extract_name_from_json(json, id) {
                        Some(name) => map.insert(name, id.to_string()),
                        None => continue
                        };
                    }
                Err(_) => {
                    println!("Unable to parse JSON for appid {}", id);
                    continue
                }
            }
            None => continue
        }
    }

    for (name, appid) in map.iter() { 
        println!("{:6} - {}", appid, name);
    }
}

fn extract_name_from_json(json: serde_json::Value, id: &str) -> Option<String> {
    Some(json.get(id)?.get("data")?.get("name")?.as_str()?.to_string())
}

fn query_appid(appid: &String) -> Option<String> {
    let url = format!("https://store.steampowered.com/api/appdetails/?appids={}", appid);

    let mut attempt_count = 0;
    while attempt_count < MAX_REQUESTS {
        match reqwest::get(&url) {
            Ok(mut response) => {
                if response.status() == reqwest::StatusCode::OK {
                    match response.text() {
                        Ok(s) => return Some(s),
                        Err(_) => return None
                    }
                }
            }
            Err(_) => attempt_count += 1
        }
    }
    return None;
}

fn get_steam_library_path() -> Option<std::path::PathBuf> {
    use std::env;
    use std::path::PathBuf;
    match env::var_os("STEAM_COMPATDATA") {
        Some(val) => return Some(PathBuf::from(val)),
        None => {
            let home = dirs::home_dir().expect("Unable to determine $HOME");
            let home_str = home.to_str().expect("Unable to convert $HOME to str");
            
            let mut path = PathBuf::from(format!("{}/.steam/steam/steamapps/compatdata", home_str));
            if path.exists() {
                return Some(path);
            }
            
            path = PathBuf::from(format!("{}/.steam/steam/SteamApps/compatdata", home_str));
            if path.exists() {
                return Some(path);
            }

            path = PathBuf::from(format!("{}/.steam/Steam/SteamApps/compatdata", home_str));
            if path.exists() {
                return Some(path);
            }

            path = PathBuf::from(format!("{}/.steam/Steam/steamapps/compatdata", home_str));
            if path.exists() {
                return Some(path);
            }
            return None;
        }
    }
}

fn parse_appid_directories() -> Vec<String> {
    let mut ids: Vec<String> = Vec::new();

    match get_steam_library_path() {
        Some(mut path) => {
            path.push("compatdata");
            let files = std::fs::read_dir(path).unwrap();
            for file in files {
                let f = file.unwrap();
                if f.file_type().unwrap().is_dir() {
                    let name = f.file_name().into_string().unwrap();
                    if  name.parse::<u32>().is_ok() {
                        ids.push(name);
                    }
                }
            }
        }
        None => println!(
"Steam compatdata folder was not found. Either set environment variable STEAM_COMPATDATA or use
steamname <appid> to manually check appids. You can supply any number of appids seperated by spaces.

Examples: STEAM_COMPATDATA=\"/home/username/some/folder/steam/steamapps/\" steamname
          steamname 8930 570")
    }
    return ids;
}