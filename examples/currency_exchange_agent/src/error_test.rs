use crate::api::FinancialDataClient;
use autoagents::core::tool::ToolCallError;

/// Test module to verify user-friendly error handling
/// This demonstrates the improved error messages without requiring actual API calls
pub struct ErrorHandlingTest;

impl ErrorHandlingTest {
    /// Test missing API key error handling
    pub fn test_missing_api_key() -> Result<(), ToolCallError> {
        println!("🧪 Testing missing API key error handling...");

        // Temporarily unset the API key to test error handling
        let original_key = std::env::var("ALPHA_VANTAGE_API_KEY").ok();
        std::env::remove_var("ALPHA_VANTAGE_API_KEY");

        let result = FinancialDataClient::get_instance();

        // Restore original key if it existed
        if let Some(key) = original_key {
            std::env::set_var("ALPHA_VANTAGE_API_KEY", key);
        }

        match result {
            Err(ToolCallError::RuntimeError(msg)) => {
                println!("✅ Correctly caught missing API key error:");
                println!("   {}", msg);
                assert!(msg.to_string().contains("ALPHA_VANTAGE_API_KEY"));
                assert!(msg.to_string().contains("alphavantage.co"));
            }
            _ => {
                println!("❌ Expected missing API key error but got different result");
                return Err(ToolCallError::RuntimeError("Test failed".into()));
            }
        }

        Ok(())
    }

    /// Test input validation error messages
    pub async fn test_input_validation() -> Result<(), ToolCallError> {
        println!("🧪 Testing input validation error handling...");

        // Create a client with a dummy API key for testing
        let client = FinancialDataClient::new("test_key".to_string());

        // Test invalid currency codes in forex quote
        println!("  Testing invalid currency code validation...");
        match client.get_forex_quote("INVALID", "EUR").await {
            Err(ToolCallError::RuntimeError(msg)) => {
                println!("  ✅ Correctly caught invalid currency error:");
                println!("     {}", msg);
                assert!(msg.to_string().contains("💱") || msg.to_string().contains("🔍"));
            }
            _ => {
                println!("  ❌ Expected invalid currency error");
                return Err(ToolCallError::RuntimeError(
                    "Invalid currency test failed".into(),
                ));
            }
        }

        Ok(())
    }

    /// Test API response format validation
    pub fn test_response_format_validation() {
        println!("🧪 Testing API response format validation...");

        use serde_json::json;

        // Test missing main structure
        let invalid_response1 = json!({
            "some_other_field": "value"
        });

        match FinancialDataClient::validate_forex_response_format(
            &invalid_response1,
            "test operation",
        ) {
            Err(ToolCallError::RuntimeError(msg)) => {
                println!("  ✅ Correctly caught missing main structure:");
                println!("     {}", msg);
                assert!(msg.to_string().contains("🔍"));
                assert!(msg.to_string().contains("Realtime Currency Exchange Rate"));
            }
            _ => panic!("Expected missing main structure error"),
        }

        // Test missing required fields
        let invalid_response2 = json!({
            "Realtime Currency Exchange Rate": {
                "1. From_Currency Code": "USD",
                // Missing other required fields
            }
        });

        match FinancialDataClient::validate_forex_response_format(
            &invalid_response2,
            "test operation",
        ) {
            Err(ToolCallError::RuntimeError(msg)) => {
                println!("  ✅ Correctly caught missing required fields:");
                println!("     {}", msg);
                assert!(msg.to_string().contains("🔍"));
                assert!(msg.to_string().contains("Missing required fields"));
            }
            _ => panic!("Expected missing required fields error"),
        }

        // Test invalid numeric values
        let invalid_response3 = json!({
            "Realtime Currency Exchange Rate": {
                "1. From_Currency Code": "USD",
                "2. From_Currency Name": "United States Dollar",
                "3. To_Currency Code": "EUR",
                "4. To_Currency Name": "Euro",
                "5. Exchange Rate": "invalid_number",
                "6. Last Refreshed": "2024-01-15 10:00:00",
                "7. Time Zone": "UTC",
                "8. Bid Price": "1.0850",
                "9. Ask Price": "1.0860"
            }
        });

        match FinancialDataClient::validate_forex_response_format(
            &invalid_response3,
            "test operation",
        ) {
            Err(ToolCallError::RuntimeError(msg)) => {
                println!("  ✅ Correctly caught invalid exchange rate format:");
                println!("     {}", msg);
                assert!(msg.to_string().contains("💱"));
                assert!(msg.to_string().contains("Invalid exchange rate format"));
            }
            _ => panic!("Expected invalid exchange rate format error"),
        }

        // Test valid response
        let valid_response = json!({
            "Realtime Currency Exchange Rate": {
                "1. From_Currency Code": "USD",
                "2. From_Currency Name": "United States Dollar",
                "3. To_Currency Code": "EUR",
                "4. To_Currency Name": "Euro",
                "5. Exchange Rate": "0.9234",
                "6. Last Refreshed": "2024-01-15 10:00:00",
                "7. Time Zone": "UTC",
                "8. Bid Price": "0.9230",
                "9. Ask Price": "0.9238"
            }
        });

        match FinancialDataClient::validate_forex_response_format(&valid_response, "test operation")
        {
            Ok(()) => {
                println!("  ✅ Valid response format correctly accepted");
            }
            Err(e) => panic!("Valid response should not produce error: {}", e),
        }

        println!("  ✅ All response format validation tests passed");
    }

    /// Test HTTP status code error formatting
    pub fn test_http_error_formatting() {
        println!("🧪 Testing HTTP error message formatting...");

        use reqwest::StatusCode;

        let test_cases = vec![
            (StatusCode::UNAUTHORIZED, "🔑", "Authentication failed"),
            (StatusCode::FORBIDDEN, "🚫", "Access denied"),
            (StatusCode::TOO_MANY_REQUESTS, "⏰", "Rate limit exceeded"),
            (StatusCode::INTERNAL_SERVER_ERROR, "🔧", "server error"),
            (
                StatusCode::SERVICE_UNAVAILABLE,
                "🚧",
                "temporarily unavailable",
            ),
            (StatusCode::BAD_REQUEST, "📝", "Invalid request parameters"),
        ];

        for (status, expected_emoji, expected_text) in test_cases {
            let error_msg = FinancialDataClient::format_http_error(status, "test operation");
            println!("  {} -> {}", status, error_msg);

            assert!(
                error_msg.contains(expected_emoji),
                "Expected emoji '{}' in error message for {}",
                expected_emoji,
                status
            );
            assert!(
                error_msg
                    .to_lowercase()
                    .contains(&expected_text.to_lowercase()),
                "Expected text '{}' in error message for {}",
                expected_text,
                status
            );
        }

        println!("  ✅ All HTTP error messages formatted correctly");
    }

    /// Run all error handling tests
    pub async fn run_all_tests() -> Result<(), ToolCallError> {
        println!("🚀 Running Currency Exchange Agent Error Handling Tests\n");

        // Test missing API key
        Self::test_missing_api_key()?;
        println!();

        // Test input validation
        Self::test_input_validation().await?;
        println!();

        // Test response format validation
        Self::test_response_format_validation();
        println!();

        // Test HTTP error formatting
        Self::test_http_error_formatting();
        println!();

        println!("🎉 All error handling tests passed successfully!");
        println!("✨ The currency exchange agent now provides user-friendly error messages");

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_error_handling() {
        ErrorHandlingTest::run_all_tests()
            .await
            .expect("Error handling tests should pass");
    }
}
