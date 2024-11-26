mod web;
mod html_extractors;

use std::env;
use web::get_website_content;
use html_extractors::extract_links;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let enough_parameters_given = args.len() != 2;
    if enough_parameters_given {
        println!("Usage: {} <url>", args[0]);
        std::process::exit(1);
    }

    let url = &args[1];
    let website_request = get_website_content(url);
    
    let error = website_request.error;
    if let Some(error) = error {
        eprintln!("Error fetching website: {}", error);
        std::process::exit(1);
    }
    
    let content = website_request.content.expect("No content received from website");

    println!("Found the following links:");
    let links = extract_links(&content);
    
    if links.is_empty() {
        println!("No links found on the page");
        std::process::exit(0);
    }

    for link in links {
        println!("  {}", link);
    }
}
