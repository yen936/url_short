mod url_shortener;
mod storage;
mod models;

use clap::{Parser, Subcommand};
use models::UrlMapping;
use url_shortener::generate_short_url;
use storage::{save_mapping, load_mappings, find_short_url_by_original};

#[derive(Parser)]
#[command(name = "URL Shortener")]
#[command(about = "A simple CLI tool for URL shortening")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Shorten { url: String },
    Lookup { short_url: String },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
            Commands::Shorten { url } => {
            // Check if the URL already has a short version
            match find_short_url_by_original(url) {
                Ok(Some(existing_short_url)) => {
                    println!("Short URL already exists: {}", existing_short_url);
                }
                Ok(None) => {
                    // Generate a new short URL if it doesn't exist
                    let short_url = generate_short_url(&url);
                    let mapping = UrlMapping {
                        short_url: short_url.clone(),
                        original_url: url.clone(),
                    };
                    if let Err(e) = save_mapping(&mapping) {
                        eprintln!("Failed to save mapping: {}", e);
                    } else {
                        println!("Saved, New Short URL is: {}", short_url);
                    }
                }
                Err(e) => eprintln!("Failed to check existing mappings: {}", e),
            }
        }
        Commands::Lookup { short_url } => {
            let mappings = load_mappings().expect("Failed to load mappings");
            match mappings.iter().find(|m| m.short_url == *short_url) {
                Some(mapping) => println!("Original URL: {}", mapping.original_url),
                None => println!("No URL found for {}", short_url),
            }
        }
    }
}