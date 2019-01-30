extern crate dirs;
extern crate reqwest;
extern crate serde_json;

fn main() {
    let mut ids: Vec<String> = std::env::args().skip(1).collect();

    if ids.len() == 0 {
        ids = parse_appid_directories();
    }

    use std::collections::BTreeMap;
    let mut map: BTreeMap<String, String> = BTreeMap::new();
    for id in ids.iter() {
        let json: serde_json::Value;
        match query_appid(id) {
            Some(response) => match serde_json::from_str(&response) {
                Ok(j) => json = j,
                Err(_) => {
                    println!("Unable to parse JSON for appid {}", id);
                    continue
                }
            }
            None => continue
        }

        match extract_name_from_json(json, id) {
            Some(name) => map.insert(name, id.to_string()),
            None => continue
        };
    }

    for (name, appid) in map.iter() { 
        println!("{:6} - {}", appid, name);
    }
}

fn extract_name_from_json(json: serde_json::Value, id: &str) -> Option<String> {
    match json.get(id) {
        Some(json_id) => match json_id.get("data") {
            Some(json_data) => match json_data.get("name") {
                Some(json_name) => match json_name.as_str() {
                    Some(name) => return Some(name.to_string()),
                    None => return None,
                }
                None => return None,
            }
            None => return None,
        }
        None => return None,
    };
}

fn query_appid(appid: &String) -> Option<String> {
    let url = format!("https://store.steampowered.com/api/appdetails/?appids={}", appid);

    let mut attempt = 0;
    while attempt < 3 {
        match reqwest::get(&url) {
            Ok(mut response) => {
                if response.status() == reqwest::StatusCode::OK {
                    match response.text() {
                        Ok(s) => return Some(s),
                        Err(_) => return None
                    }
                }
            }
            Err(_) => attempt += 1
        }
    }
    return None;
}

fn get_steam_library_path() -> Option<std::path::PathBuf> {
    use std::env;
    use std::path::PathBuf;
    match env::var_os("STEAM_DIR") {
        Some(val) => return Some(PathBuf::from(val)),
        None => {
            let home = dirs::home_dir().expect("Unable to determine $HOME");
            let home_str = home.to_str().expect("Unable to convert $HOME to str");
            
            let mut path = PathBuf::from(format!("{}/.steam/steam/steamapps", home_str));
            if path.exists() {
                return Some(path);
            }
            
            path = PathBuf::from(format!("{}/.steam/steam/SteamApps", home_str));
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
        None => println!("Steam library not found at ~/.steam/steam/steamapps or ~/.steam/steam/SteamApps\nYou can set environment variable STEAM_DIR if you have another steam directory or\nuse steamname <appid> to manually check appids.")
    }
    return ids;
}