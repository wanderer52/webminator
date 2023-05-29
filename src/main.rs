use regex::Regex;
use reqwest::blocking::Client;
use std::env;

fn get_thread_url() -> Result<String, String> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        return Err(String::from(
            "Please provide exactly one command line argument.",
        ));
    }

    let re = Regex::new(r"https://boards\.(4chan|4channel)\.org/[[:alnum:]]+/thread/\d+")
        .expect("Failed to compile regular expression.");
    let url = &args[1];
    if !re.is_match(url) {
        return Err(String::from("Invalid 4Chan url."));
    }

    Ok(url.to_string())
}

fn get_thread_source(url: &String) -> Result<String, String> {
    let client = Client::new();

    let response = client
        .get(url)
        .send()
        .map_err(|err| format!("Failed to send HTTP request: {}", err))?;

    if response.status().is_success() {
        response
            .text()
            .map_err(|err| format!("Failed to retrieve response body: {}", err))
    } else {
        Err(format!(
            "HTTP request failed with status code: {}",
            response.status()
        ))
    }
}

fn main() {
    match get_thread_url() {
        Ok(link) => {
            let url = link;
        }
        Err(err) => println!("Error: {err}"),
    };
}
