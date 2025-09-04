use anyhow::Result;
use chrono::Local;
use colored::*;
use reqwest::Client;
use serde_json::Value;
use std::io::{self, Write};

fn get_user_input(prompt: &str) -> Result<String> {
    print!("{}", prompt);
    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_string())
}

fn get_today_date() -> String {
    Local::now().format("%Y-%m-%d").to_string()
}

fn validate_date_format(date: &str) -> bool {
    // Check if date matches YYYY-MM-DD format
    if date.len() != 10 {
        return false;
    }

    let parts: Vec<&str> = date.split('-').collect();
    if parts.len() != 3 {
        return false;
    }

    // Check if year, month, day are valid numbers
    if let (Ok(year), Ok(month), Ok(day)) = (
        parts[0].parse::<u32>(),
        parts[1].parse::<u32>(),
        parts[2].parse::<u32>(),
    ) {
        year >= 1999 && year <= 2026 && month >= 1 && month <= 12 && day >= 1 && day <= 31
    } else {
        false
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    // Get API key from environment variable
    let api_key =
        std::env::var("EXCHANGE_API_KEY").expect("EXCHANGE_API_KEY environment variable not set");

    println!(
        "{}",
        "Interactive Currency Exchange Rate Fetcher".green().bold()
    );
    println!("Get historical exchange rates for any date ranging from 1999 to today!\n");

    // Get today's date as default
    let today = get_today_date();

    // Get date input from user
    let date = loop {
        let prompt = format!(
            "Enter date (YYYY-MM-DD format) or press Enter for today [{}]: ",
            today
        );
        let input = get_user_input(&prompt)?;

        // If input is empty, use today's date
        let date_to_validate = if input.is_empty() {
            today.clone()
        } else {
            input
        };

        if validate_date_format(&date_to_validate) {
            break date_to_validate;
        } else {
            println!(
                "{}",
                "Invalid date format! Please use YYYY-MM-DD format (e.g., 2024-01-15)".red()
            );
        }
    };

    println!(
        "\n{}",
        format!("Fetching exchange rates for {}...", date).cyan()
    );

    // Create HTTP client
    let client = Client::new();

    // Build the API URL for historical data with corrected format
    let url = format!(
        "https://api.exchangeratesapi.io/v1/{}?access_key={}&base=EUR&symbols=USD,GBP,EUR,INR,JPY",
        date, api_key
    );

    // Make the API request
    let response = client.get(&url).send().await?;

    // Check if the request was successful
    if response.status().is_success() {
        let data: Value = response.json().await?;

        // Check if the API returned an error
        if let Some(error) = data.get("error") {
            println!("{}", format!("API Error: {}", error).red());
            if let Some(error_info) = data.get("error").and_then(|e| e.get("info")) {
                println!("{}", format!("Details: {}", error_info).red());
            }
            return Ok(());
        }

        // Extract and display the exchange rates
        println!("\n{}", "Historical Exchange Rates".cyan().bold());
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
                println!("1 EUR = {:.4} {}", rate.as_f64().unwrap_or(0.0), currency);
            }
        } else {
            println!("{}", "No rates data found in the response".red());
        }

        // Note about the API
        println!(
            "\n{}",
            "Note: Historical data shows exchange rates from EUR to other currencies.".yellow()
        );
    } else {
        println!(
            "{}",
            format!("API request failed with status: {}", response.status()).red()
        );

        // Try to get error details from response body
        if let Ok(error_text) = response.text().await {
            println!("{}", format!("Error details: {}", error_text).red());
        }
    }

    Ok(())
}
