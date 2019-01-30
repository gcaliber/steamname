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
        let response = query_appid(id);

        let json: serde_json::Value = serde_json::from_str(&response)
            .expect("Unable to parse, JSON was not well-formatted");

        match json.get(id) {
            Some(json_id) => match json_id.get("data") {
                Some(json_data) => match json_data.get("name") {
                    Some(json_name) => match json_name.as_str() {
                        Some(name) => map.insert(name.to_string(), id.to_string()),
                        None => continue,
                    }
                    None => continue,
                }
                None => continue,
            }
            None => continue,
        };
    }

    for (name, appid) in map.iter() { 
        println!("{:6} - {}", appid, name);
    }
}

fn query_appid(appid: &String) -> String {
    let url = format!("https://store.steampowered.com/api/appdetails/?appids={}", appid);

    reqwest::get(&url)
        .expect(&format!("Unable to get response for {}", appid))
        .text()
            .expect("Unable to parse response")
}

fn get_steam_library_path() -> Option<std::path::PathBuf> {
    use std::env;
    use std::path::PathBuf;
    match env::var_os("STEAM_DIR") {
        Some(val) => return Some(PathBuf::from(val)),
        None => {
            let home = dirs::home_dir().unwrap();
            let home_str = home.to_str().unwrap();
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