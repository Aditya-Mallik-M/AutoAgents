use anyhow::Result;
use colored::*;
use reqwest::Client;
use serde_json::Value;

#[tokio::main]
async fn main() -> Result<()> {
    // Get API key from environment variable
    let api_key =
        std::env::var("EXCHANGE_API_KEY").expect("EXCHANGE_API_KEY environment variable not set");

    println!("{}", "Currency Exchange Rate Fetcher".green().bold());
    println!("Fetching latest exchange rates for EUR...\n");

    // Create HTTP client
    let client = Client::new();

    // Build the API URL with query parameters
    let url = format!(
        "https://api.exchangeratesapi.io/v1/latest?access_key={}&base=EUR&symbols=INR,USD,GBP,JPY",
        api_key
    );

    // Make the API request
    let response = client.get(&url).send().await?;

    // Check if the request was successful
    if response.status().is_success() {
        let data: Value = response.json().await?;

        // Check if the API returned an error
        if let Some(error) = data.get("error") {
            println!("{}", format!("API Error: {}", error).red());
            return Ok(());
        }

        // Extract and display the exchange rates
        println!("{}", "Latest Exchange Rates".cyan().bold());
        println!(
            "{}",
            format!("Base Currency: {}", data["base"].as_str().unwrap_or("EUR")).yellow()
        );
        println!(
            "{}",
            format!("Date: {}", data["date"].as_str().unwrap_or("Unknown")).yellow()
        );

        if let Some(rates) = data.get("rates").and_then(|r| r.as_object()) {
            println!("\n{}", "Exchange Rates:".cyan());
            for (currency, rate) in rates {
                println!("{}: {:.4}", currency, rate.as_f64().unwrap_or(0.0));
            }
        } else {
            println!("{}", "No rates data found in the response".red());
        }

        // Note about the API
        println!(
            "\n{}",
            "Note: Using EUR as base currency (supported by free tier).".yellow()
        );
    } else {
        println!(
            "{}",
            format!("API request failed with status: {}", response.status()).red()
        );
    }

    Ok(())
}
