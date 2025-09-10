use crate::agent::{CurrencyAgentOutput, CurrencyExchangeAgent};
use autoagents::core::agent::memory::SlidingWindowMemory;
use autoagents::core::agent::prebuilt::executor::ReActAgentOutput;
use autoagents::core::agent::task::Task;
use autoagents::core::agent::{AgentBuilder, RunnableAgent};
use autoagents::core::environment::Environment;
use autoagents::core::error::Error;
use autoagents::core::protocol::{Event, TaskResult};
use autoagents::core::runtime::SingleThreadedRuntime;
use autoagents::llm::LLMProvider;
use colored::*;
use serde_json::Value;
use std::io::{self, Write};
use std::sync::Arc;
use tokio_stream::StreamExt;

pub async fn run_interactive_session(llm: Arc<dyn LLMProvider>) -> Result<(), Error> {
    // Create runtime and environment
    let runtime = SingleThreadedRuntime::new(None);
    let agent = CurrencyExchangeAgent {};

    // Build the agent with memory
    let memory = Box::new(SlidingWindowMemory::new(20));
    let agent_handle = AgentBuilder::new(agent)
        .with_llm(llm)
        .runtime(runtime.clone())
        .with_memory(memory)
        .build()
        .await?;

    // Create environment and register runtime
    let mut environment = Environment::new(None);
    let _ = environment.register_runtime(runtime.clone()).await;
    let receiver = environment.take_event_receiver(None).await?;

    // Handle events in background
    handle_events(receiver);

    // Welcome message
    println!("{}", "\n\n\n");
    println!(
        "{}",
        "🌍 Welcome to the Intelligent Currency Exchange Agent! 💱"
            .green()
            .bold()
    );
    println!(
        "{}",
        "Powered by AutoAgents with natural language processing capabilities.".cyan()
    );
    println!(
        "{}",
        "Ask me anything about currency exchange rates, conversions, or trends!".yellow()
    );
    println!("{}", "Examples:".dimmed());
    println!("{}", "  • 'What's the current USD to EUR rate?'".dimmed());
    println!("{}", "  • 'Convert 100 dollars to euros'".dimmed());
    println!(
        "{}",
        "  • 'How did GBP perform against USD last month?'".dimmed()
    );
    println!(
        "{}",
        "  • 'Show me historical rates for JPY on January 1st, 2024'".dimmed()
    );
    println!(
        "{}",
        "\nType 'exit' or 'quit' to end the conversation.\n".dimmed()
    );

    // Interactive loop
    loop {
        // Get user input
        let input = match get_user_input("💱 Ask me anything about currencies > ") {
            Ok(input) => input,
            Err(e) => {
                println!("{}", format!("❌ Input error: {}", e).red());
                continue;
            }
        };

        // Check for exit commands
        if matches!(input.to_lowercase().as_str(), "exit" | "quit" | "bye" | "q") {
            println!(
                "{}",
                "👋 Thank you for using the Currency Exchange Agent! Goodbye!".green()
            );
            break;
        }

        // Process the query
        println!("{}", "🤔 Thinking...".cyan());

        let task = Task::new(&input);
        match agent_handle.agent.clone().run(task).await {
            Ok(result) => {
                if let TaskResult::Value(value) = result {
                    if let Ok(react_output) = serde_json::from_value::<ReActAgentOutput>(value) {
                        if let Ok(currency_output) =
                            serde_json::from_str::<CurrencyAgentOutput>(&react_output.response)
                        {
                            println!("\n{}", "💡 Analysis:".green().bold());
                            println!("{}", currency_output.analysis);

                            if let Some(data) = currency_output.data {
                                println!("\n{}", "📊 Raw Data:".blue().bold());
                                if let Ok(pretty_data) = serde_json::from_str::<Value>(&data) {
                                    println!(
                                        "{}",
                                        serde_json::to_string_pretty(&pretty_data).unwrap_or(data)
                                    );
                                } else {
                                    println!("{}", data);
                                }
                            }

                            if let Some(recommendations) = currency_output.recommendations {
                                println!("\n{}", "🎯 Recommendations:".yellow().bold());
                                println!("{}", recommendations);
                            }

                            if let Some(error) = currency_output.error {
                                println!("\n{}", "❌ Error:".red().bold());
                                println!("{}", error);
                            }
                        } else {
                            // Fallback to raw response if parsing fails
                            println!("\n{}", "💬 Response:".green().bold());
                            println!("{}", react_output.response);
                        }
                    } else {
                        println!("{}", "❌ Failed to parse agent response".red());
                    }
                } else {
                    println!("{}", "❌ No response from agent".red());
                }
            }
            Err(e) => {
                println!("{}", format!("❌ Error: {}", e).red());
                println!(
                    "{}",
                    "💡 Try rephrasing your question or check your API keys.".yellow()
                );
            }
        }

        println!(); // Add spacing between queries
    }

    Ok(())
}

pub async fn run_single_query(llm: Arc<dyn LLMProvider>, query: String) -> Result<(), Error> {
    // Create runtime and environment
    let runtime = SingleThreadedRuntime::new(None);
    let agent = CurrencyExchangeAgent {};

    // Build the agent
    let agent_handle = AgentBuilder::new(agent)
        .with_llm(llm)
        .runtime(runtime.clone())
        .build()
        .await?;

    // Create environment and register runtime
    let mut environment = Environment::new(None);
    let _ = environment.register_runtime(runtime.clone()).await;

    println!("{}", "🤔 Processing your query...".cyan());

    let task = Task::new(&query);
    match agent_handle.agent.clone().run(task).await {
        Ok(result) => {
            if let TaskResult::Value(value) = result {
                if let Ok(react_output) = serde_json::from_value::<ReActAgentOutput>(value) {
                    if let Ok(currency_output) =
                        serde_json::from_str::<CurrencyAgentOutput>(&react_output.response)
                    {
                        println!("\n{}", "💡 Analysis:".green().bold());
                        println!("{}", currency_output.analysis);

                        if let Some(data) = currency_output.data {
                            println!("\n{}", "📊 Data:".blue().bold());
                            println!("{}", data);
                        }

                        if let Some(recommendations) = currency_output.recommendations {
                            println!("\n{}", "🎯 Recommendations:".yellow().bold());
                            println!("{}", recommendations);
                        }

                        if let Some(error) = currency_output.error {
                            println!("\n{}", "❌ Error:".red().bold());
                            println!("{}", error);
                        }
                    } else {
                        // Fallback to raw response if parsing fails
                        println!("\n{}", "💬 Response:".green().bold());
                        println!("{}", react_output.response);
                    }
                } else {
                    println!("{}", "❌ Failed to parse agent response".red());
                }
            } else {
                println!("{}", "❌ No response from agent".red());
            }
        }
        Err(e) => {
            println!("{}", format!("❌ Error: {}", e).red());
        }
    }

    Ok(())
}

fn get_user_input(prompt: &str) -> Result<String, Box<dyn std::error::Error>> {
    print!("{}", prompt);
    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_string())
}

fn handle_events(mut receiver: tokio_stream::wrappers::ReceiverStream<Event>) {
    tokio::spawn(async move {
        while let Some(event) = receiver.next().await {
            match event {
                Event::ToolCallFailed {
                    tool_name, error, ..
                } => {
                    println!(
                        "{}",
                        format!("🔧 Tool Call Failed: {} - Error: {}", tool_name, error).red()
                    );
                }
                Event::TaskComplete { result, .. } => match result {
                    TaskResult::Value(_) => {
                        println!("{}", "✅ Task completed successfully".green());
                    }
                    TaskResult::Failure(e) => {
                        println!("{}", format!("❌ Task failed: {}", e).red());
                    }
                    TaskResult::Aborted => {
                        println!("{}", "⚠️ Task was aborted".yellow());
                    }
                },
                _ => {} // Ignore other events
            }
        }
    });
}
