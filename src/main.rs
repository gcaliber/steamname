extern crate reqwest;
extern crate serde_json;

fn main() {
    get_stuff("8930".to_string());
}

fn get_stuff(appid: String) {
    let url = format!("https://store.steampowered.com/api/appdetails/?appids={}", appid);
    
    match reqwest::get(&url) {
        Ok(mut response) => {
            if response.status() == reqwest::StatusCode::OK {
                match response.text() {
                    Ok(text) => println!("{}", text),
                    Err(_) => println!("Unable to read response text")
                }
            }
            else {
                println!("Response NOT 200 Ok");
            }
        }
        Err(_) => println!("Error: Did not get a response")
    }
}