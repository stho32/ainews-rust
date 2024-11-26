mod web;
mod html_extractors;
use std::env;
use web::get_website_content;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() != 2 {
        println!("Usage: {} <url>", args[0]);
        std::process::exit(1);
    }

    let url = &args[1];
    let (content, error) = get_website_content(url);

    match (content, error) {
        (Some(content), None) => println!("Content:\n{}", content),
        (None, Some(error)) => {
            eprintln!("Error: {}", error);
            std::process::exit(1);
        }
        _ => {
            eprintln!("Unexpected state: both content and error are None");
            std::process::exit(1);
        }
    }
}
