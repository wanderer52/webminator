use regex::Regex;
use reqwest::blocking::Client;
use std::collections::HashSet;
use std::env;
use std::process;

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

fn build_webm_list(source: &String) -> Result<Vec<String>, String> {
    let regex_str = r#"(?i)\/\/is2\.4chan\.org\/[a-z0-9]+\/(\d+)\.webm"#;
    let regex =
        Regex::new(regex_str).map_err(|err| format!("Regex compilation failed: {}", err))?;

    let mut webm_list: Vec<String> = Vec::new();
    let mut unique_links: HashSet<String> = HashSet::new();

    for capture in regex.captures_iter(source) {
        if let Some(link) = capture.get(0) {
            let mut webm_url = String::from("https:");
            webm_url.push_str(link.as_str());
            if unique_links.insert(webm_url.clone()) {
                webm_list.push(webm_url);
            }
        }
    }

    Ok(webm_list)
}

fn main() {
    let url = get_thread_url().unwrap_or_else(|err| {
        eprintln!("Error: {err}");
        process::exit(0)
    });

    let source = get_thread_source(&url).unwrap_or_else(|err| {
        eprintln!("Error: {err}");
        process::exit(0)
    });
}
