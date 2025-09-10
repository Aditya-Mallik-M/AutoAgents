use clap::Parser;
use monitor::{CurrencyMonitor, MonitoringConfig};
use std::sync::Arc;

mod advanced_tools;
mod agent;
mod api;
mod error_test;
mod interactive;
mod monitor;

use autoagents::{
    core::error::Error,
    llm::{
        backends::{anthropic::Anthropic, ollama::Ollama, openai::OpenAI},
        builder::LLMBuilder,
        LLMProvider,
    },
};

// Command line arguments
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// LLM provider to use (openai, anthropic, ollama)
    #[arg(
        short = 'l',
        long = "llm",
        default_value = "openai",
        help = "LLM provider: openai, anthropic, or ollama"
    )]
    llm: String,

    /// Model name to use
    #[arg(
        short,
        long,
        help = "Model name (e.g., gpt-4o-mini, claude-3-sonnet, llama2)"
    )]
    model: Option<String>,

    /// Interactive mode (default) or single query mode
    #[arg(short, long, help = "Run in interactive chat mode")]
    interactive: bool,

    /// Single query for non-interactive mode
    #[arg(short, long, help = "Single query to process")]
    query: Option<String>,

    /// Monitor mode - continuously watch currency rates and make trading suggestions
    #[arg(long, help = "Run in autonomous monitoring mode")]
    monitor: bool,

    /// Initial investment amount for monitoring mode
    #[arg(long, help = "Initial investment amount (required for monitor mode)")]
    initial_amount: Option<f64>,

    /// Initial currency for monitoring mode
    #[arg(
        long,
        help = "Initial currency code (e.g., USD, EUR) for monitoring mode"
    )]
    initial_currency: Option<String>,

    /// Monitoring interval in seconds
    #[arg(
        long,
        default_value = "60",
        help = "Monitoring interval in seconds (default: 60)"
    )]
    interval: u64,

    /// Run error handling tests
    #[arg(
        long,
        help = "Run error handling tests to verify user-friendly messages"
    )]
    test_errors: bool,
}

fn create_llm(provider: &str, model: Option<String>) -> Result<Arc<dyn LLMProvider>, Error> {
    match provider.to_lowercase().as_str() {
        "openai" => {
            let api_key = std::env::var("OPENAI_API_KEY").map_err(|_| {
                Error::CustomError("OPENAI_API_KEY environment variable not set".to_string())
            })?;
            let model = model.unwrap_or_else(|| "gpt-4o-mini".to_string());
            let llm = LLMBuilder::<OpenAI>::new()
                .api_key(&api_key)
                .model(&model)
                .build()
                .map_err(|e| Error::CustomError(format!("Failed to create OpenAI LLM: {}", e)))?;
            Ok(llm)
        }
        "anthropic" => {
            let api_key = std::env::var("ANTHROPIC_API_KEY").map_err(|_| {
                Error::CustomError("ANTHROPIC_API_KEY environment variable not set".to_string())
            })?;
            let model = model.unwrap_or_else(|| "claude-3-sonnet-20240229".to_string());
            let llm = LLMBuilder::<Anthropic>::new()
                .api_key(&api_key)
                .model(&model)
                .build()
                .map_err(|e| {
                    Error::CustomError(format!("Failed to create Anthropic LLM: {}", e))
                })?;
            Ok(llm)
        }
        "ollama" => {
            let model = model.unwrap_or_else(|| "llama2".to_string());
            let llm = LLMBuilder::<Ollama>::new()
                .model(&model)
                .build()
                .map_err(|e| Error::CustomError(format!("Failed to create Ollama LLM: {}", e)))?;
            Ok(llm)
        }
        _ => Err(Error::CustomError(format!(
            "Unsupported LLM provider: {}",
            provider
        ))),
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    // Parse command line arguments
    let args = Args::parse();

    // Handle different modes
    if args.test_errors {
        // Run error handling tests (skip API key validation for tests)
        println!("🧪 Running Error Handling Tests for Currency Exchange Agent");
        println!("===========================================================\n");

        match error_test::ErrorHandlingTest::run_all_tests().await {
            Ok(()) => {
                println!("\n🎉 All error handling tests completed successfully!");
                println!("✨ The currency exchange agent provides user-friendly error messages.");
            }
            Err(e) => {
                eprintln!("\n❌ Error handling tests failed: {}", e);
                std::process::exit(1);
            }
        }
        return Ok(());
    }

    // Validate that ALPHA_VANTAGE_API_KEY is set (for normal operations)
    if std::env::var("ALPHA_VANTAGE_API_KEY").is_err() {
        eprintln!("❌ Error: ALPHA_VANTAGE_API_KEY environment variable not set.");
        eprintln!("📋 To fix this:");
        eprintln!("   1. Get a free API key from: https://www.alphavantage.co/support/#api-key");
        eprintln!("   2. Set the environment variable:");
        eprintln!("      export ALPHA_VANTAGE_API_KEY=\"your-api-key-here\"");
        eprintln!("   3. Run the command again");
        eprintln!();
        eprintln!("💡 The Alpha Vantage API provides professional-grade forex data and technical indicators.");
        std::process::exit(1);
    }

    // Create LLM provider
    let llm = create_llm(&args.llm, args.model)?;

    // Handle different modes
    if args.monitor {
        // Monitoring mode - autonomous currency monitoring
        let initial_amount = args.initial_amount.ok_or_else(|| {
            Error::CustomError("--initial-amount is required for monitor mode".to_string())
        })?;
        let initial_currency = args.initial_currency.ok_or_else(|| {
            Error::CustomError("--initial-currency is required for monitor mode".to_string())
        })?;

        run_monitoring_mode(llm, initial_amount, initial_currency, args.interval).await?;
    } else if let Some(query) = args.query {
        // Single query mode
        interactive::run_single_query(llm, query).await?;
    } else {
        // Interactive mode (default)
        interactive::run_interactive_session(llm).await?;
    }

    Ok(())
}

async fn run_monitoring_mode(
    llm: Arc<dyn LLMProvider>,
    initial_amount: f64,
    initial_currency: String,
    interval_seconds: u64,
) -> Result<(), Error> {
    println!("🌍 Advanced Currency Trading Monitor 💱📈");
    println!("==========================================");

    // Create monitoring configuration
    let mut config = MonitoringConfig::default();
    config.monitoring_interval_seconds = interval_seconds;

    // Create and start the currency monitor
    let mut monitor = CurrencyMonitor::new(initial_amount, initial_currency, llm, Some(config))?;

    // Set up signal handler for graceful shutdown
    println!("💡 Press Ctrl+C to stop monitoring and view final portfolio summary");

    // Handle Ctrl+C gracefully
    let monitor_handle = tokio::spawn(async move { monitor.start_monitoring().await });

    // Wait for Ctrl+C
    match tokio::signal::ctrl_c().await {
        Ok(()) => {
            println!("\n🛑 Shutdown signal received. Stopping monitor...");
        }
        Err(err) => {
            eprintln!("Unable to listen for shutdown signal: {}", err);
        }
    }

    // The monitor will stop when the handle is dropped
    monitor_handle.abort();

    println!("✅ Currency monitor stopped successfully.");
    Ok(())
}
