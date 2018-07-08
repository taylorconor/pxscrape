#[macro_use]
extern crate serde_derive;

extern crate regex;
extern crate reqwest;
extern crate serde_json;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        print_help();
        return;
    }
    let username = args.get(1).unwrap();
    let output_file = args.get(2).unwrap();

    let url = format!("https://500px.com/{}", username);
    let body = reqwest::get(&url).unwrap().text().unwrap();
    let images = get_images(&body);
    let json = serde_json::to_string(&images).expect("Could not serialise images to JSON");
    std::fs::write(output_file, json).expect(&format!("Failed to write to {}", output_file));
}

fn print_help() {
    println!("Usage: pxscrape [username] [output file]");
}

#[derive(Serialize, Debug)]
struct Image {
    small_url: String,
    medium_url: String,
    large_url: String,
    title: String,
    location: String,
    date: String,
}

fn get_images(src: &String) -> Vec<Image> {
    for line in src.lines() {
        if line.starts_with("App.bootstrap = {") {
            if line.len() == 0 {
                panic!("App.bootstrap is empty");
            }
            return parse_app_bootstrap(line.split_at(16).1);
        }
    }
    panic!("Could not find App.bootstrap");
}

fn parse_app_bootstrap(app_bootstrap: &str) -> Vec<Image> {
    let json_value: serde_json::Value = serde_json::from_str(app_bootstrap).unwrap();
    let mut images: Vec<Image> = Vec::new();
    for photo in json_value["userdata"]["photos"].as_array().unwrap() {
        let (small_url, medium_url, large_url) = parse_image_urls(photo);
        images.push(Image {
            small_url: small_url,
            medium_url: medium_url,
            large_url: large_url,
            title: photo["name"].as_str().unwrap().to_string(),
            location: photo["description"].as_str().unwrap_or("").to_string(),
            date: photo["created_at"].as_str().unwrap().to_string(),
        });
    }
    images
}

fn parse_image_urls(image_obj: &serde_json::Value) -> (String, String, String) {
    let (mut small_url, mut medium_url, mut large_url) = (None, None, None);
    for image in image_obj["images"].as_array().unwrap() {
        match image["size"].as_u64().unwrap() {
            35 => large_url = Some(image["url"].as_str().unwrap().to_string()),
            34 => medium_url = Some(image["url"].as_str().unwrap().to_string()),
            4 => small_url = Some(image["url"].as_str().unwrap().to_string()),
            _ => {}
        }
    }
    if !small_url.is_some() || !medium_url.is_some() || !large_url.is_some() {
        panic!("Could not find correct image sizes in App.bootstrap");
    }
    (small_url.unwrap(), medium_url.unwrap(), large_url.unwrap())
}
