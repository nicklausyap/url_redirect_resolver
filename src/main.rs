// This code allow you to analyze or resolve the final URL after following redirects
// Date created: Wednesday, 7 February, 2024
use std::env;
use std::io;
use reqwest::Client;

async fn get_final_url(url: &str) -> Result<String, reqwest::Error> {
    let client = Client::new();
    let response = client.get(url).send().await?;

    // Retrieve the final URL after following redirects
    let final_url = response.url().to_string();

    Ok(final_url)
}

#[tokio::main]
async fn main() {
    let mut url: String;

    // Fetch command line arguments
    let args: Vec<String> = env::args().collect();

    // Check if a command-line argument is provided
    if args.len() > 1 {
        url = args[1].clone();
    } else {
        loop {
            println!("Enter a URL:");
            
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Failed to read line");
            url = input.trim().to_string();

            if url.is_empty() {
                println!("URL cannot be empty. Please try again.");
            } else {
                break;
            }
        }
    }

    match get_final_url(&url).await {
        Ok(final_url) => println!("Final URL: {}", final_url),
        Err(e) => eprintln!("Error: {}", e),
    }
}
