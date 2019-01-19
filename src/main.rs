extern crate reqwest;

fn main() -> Result<(), Box<std::error::Error>> {
    use std::io::Read;

    let mut resp = reqwest::get("https://store.steampowered.com/api/appdetails/?appids=391220")?;
    assert!(resp.status().is_success());

    let mut content = String::new();

    resp.read_to_string(&mut content).expect("could not read file");
    
    println!("{:#?}", content);
    Ok(())
}