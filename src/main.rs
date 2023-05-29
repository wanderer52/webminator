use regex::Regex;
use std::env;

fn get_thread_url() -> Result<String, String> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        return Err(String::from(
            "Please provide exactly one command line argument.",
        ));
    }

    let re = Regex::new(r"https://boards\.(4chan|4channel)\.org/[[:alnum:]]+/thread/\d+");
}

fn main() {}
