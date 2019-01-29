extern crate reqwest;
extern crate serde_json;

fn main() {
    let mut ids: Vec<String> = std::env::args().collect();

    if ids.len() == 1 {
        // TODO: parse steam library directories from compatdata
        // should use default directory or env variable if it is set
        // if env variable not set and nothing is found, print msg about setting it
    }
    else {
        ids.swap_remove(0);
    }

    for id in ids.iter() {
        let response = query_appid(id);

        let json: serde_json::Value = serde_json::from_str(&response)
            .expect("JSON was not well-formatted");

        let name = json.get(id).unwrap().get("data").unwrap().get("name").unwrap().as_str().unwrap();
        
        println!("{}", name);
    }
}

fn query_appid(appid: &String) -> String {
    let url = format!("https://store.steampowered.com/api/appdetails/?appids={}", appid);

    reqwest::get(&url)
        .expect(&format!("Unable to get response for {}", appid))
        .text()
            .expect("Unable to parse response")
}