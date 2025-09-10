use autoagents::core::tool::ToolCallError;
use chrono::{DateTime, Utc};
use reqwest::{Client, StatusCode};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use ta::indicators::*;
use ta::Next;

// Advanced Financial Data Types
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ForexQuote {
    pub symbol: String,
    pub bid: f64,
    pub ask: f64,
    pub price: f64,
    pub timestamp: DateTime<Utc>,
    pub change: f64,
    pub change_percent: f64,
    pub volume: Option<f64>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct OHLCData {
    pub timestamp: DateTime<Utc>,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: f64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TechnicalIndicators {
    pub rsi: f64,
    pub macd: MACDData,
    pub bollinger_bands: BollingerBands,
    pub moving_averages: MovingAverages,
    pub stochastic: StochasticData,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MACDData {
    pub macd: f64,
    pub signal: f64,
    pub histogram: f64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BollingerBands {
    pub upper: f64,
    pub middle: f64,
    pub lower: f64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MovingAverages {
    pub sma_20: f64,
    pub sma_50: f64,
    pub ema_12: f64,
    pub ema_26: f64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct StochasticData {
    pub k: f64,
    pub d: f64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TradingSignal {
    pub signal_type: SignalType,
    pub strength: f64,   // 0.0 to 1.0
    pub confidence: f64, // 0.0 to 1.0
    pub entry_price: f64,
    pub stop_loss: Option<f64>,
    pub take_profit: Option<f64>,
    pub reasoning: String,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum SignalType {
    Buy,
    Sell,
    Hold,
    StrongBuy,
    StrongSell,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MarketSentiment {
    pub sentiment_score: f64, // -1.0 to 1.0
    pub news_sentiment: f64,
    pub social_sentiment: f64,
    pub fear_greed_index: f64,
    pub volatility_index: f64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct EconomicEvent {
    pub title: String,
    pub country: String,
    pub currency: String,
    pub impact: EventImpact,
    pub actual: Option<f64>,
    pub forecast: Option<f64>,
    pub previous: Option<f64>,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum EventImpact {
    Low,
    Medium,
    High,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PortfolioAnalysis {
    pub total_value: f64,
    pub daily_pnl: f64,
    pub total_pnl: f64,
    pub sharpe_ratio: f64,
    pub max_drawdown: f64,
    pub var_95: f64, // Value at Risk 95%
    pub positions: Vec<Position>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Position {
    pub symbol: String,
    pub quantity: f64,
    pub entry_price: f64,
    pub current_price: f64,
    pub pnl: f64,
    pub pnl_percent: f64,
    pub weight: f64,
}

// Alpha Vantage API Response Types
#[derive(Debug, Deserialize)]
struct AlphaVantageForex {
    #[serde(rename = "Realtime Currency Exchange Rate")]
    pub realtime_currency_exchange_rate: AlphaVantageRate,
}

#[derive(Debug, Deserialize)]
struct AlphaVantageRate {
    #[serde(rename = "1. From_Currency Code")]
    pub from_currency: String,
    #[serde(rename = "2. From_Currency Name")]
    pub from_name: String,
    #[serde(rename = "3. To_Currency Code")]
    pub to_currency: String,
    #[serde(rename = "4. To_Currency Name")]
    pub to_name: String,
    #[serde(rename = "5. Exchange Rate")]
    pub exchange_rate: String,
    #[serde(rename = "6. Last Refreshed")]
    pub last_refreshed: String,
    #[serde(rename = "7. Time Zone")]
    pub time_zone: String,
    #[serde(rename = "8. Bid Price")]
    pub bid_price: String,
    #[serde(rename = "9. Ask Price")]
    pub ask_price: String,
}

#[derive(Debug, Deserialize)]
struct AlphaVantageTimeSeries {
    #[serde(rename = "Time Series FX (Daily)")]
    pub time_series: Option<HashMap<String, AlphaVantageOHLC>>,
    #[serde(rename = "Time Series FX (1min)")]
    pub time_series_1min: Option<HashMap<String, AlphaVantageOHLC>>,
}

#[derive(Debug, Deserialize)]
struct AlphaVantageOHLC {
    #[serde(rename = "1. open")]
    pub open: String,
    #[serde(rename = "2. high")]
    pub high: String,
    #[serde(rename = "3. low")]
    pub low: String,
    #[serde(rename = "4. close")]
    pub close: String,
}

// Advanced Financial Data Client
pub struct FinancialDataClient {
    client: Client,
    alpha_vantage_key: String,
}

impl FinancialDataClient {
    pub fn new(alpha_vantage_key: String) -> Self {
        Self {
            client: Client::new(),
            alpha_vantage_key,
        }
    }

    pub fn get_instance() -> Result<Self, ToolCallError> {
        let alpha_vantage_key = std::env::var("ALPHA_VANTAGE_API_KEY").map_err(|_| {
            ToolCallError::RuntimeError(
                "ALPHA_VANTAGE_API_KEY environment variable not set. Get your free key at https://www.alphavantage.co/"
                    .to_string()
                    .into(),
            )
        })?;
        Ok(Self::new(alpha_vantage_key))
    }

    // Get real-time forex quote with bid/ask spread
    pub async fn get_forex_quote(&self, from: &str, to: &str) -> Result<ForexQuote, ToolCallError> {
        let context = format!("forex quote {}/{}", from, to);
        let url = format!(
            "https://www.alphavantage.co/query?function=CURRENCY_EXCHANGE_RATE&from_currency={}&to_currency={}&apikey={}",
            from, to, self.alpha_vantage_key
        );

        let response = self.client.get(&url).send().await.map_err(|e| {
            ToolCallError::RuntimeError(Self::format_network_error(&e, &context).into())
        })?;

        if !response.status().is_success() {
            return Err(ToolCallError::RuntimeError(
                Self::format_http_error(response.status(), &context).into(),
            ));
        }

        // First get the raw JSON to check for Alpha Vantage errors
        let json_data: serde_json::Value = response.json().await.map_err(|e| {
            ToolCallError::RuntimeError(Self::format_network_error(&e, &context).into())
        })?;

        // Check for Alpha Vantage specific errors
        Self::check_alpha_vantage_error(&json_data, &context)?;

        // Enhanced response format validation
        Self::validate_forex_response_format(&json_data, &context)?;

        // Parse the validated JSON data
        let data: AlphaVantageForex = serde_json::from_value(json_data.clone()).map_err(|e| {
            ToolCallError::RuntimeError(
                format!(
                    "📊 Unable to parse {} response: {}. The API response format may have changed.",
                    context, e
                )
                .into(),
            )
        })?;

        let rate_data = data.realtime_currency_exchange_rate;
        let price: f64 = rate_data
            .exchange_rate
            .parse()
            .map_err(|_| ToolCallError::RuntimeError(format!("💱 Invalid exchange rate format for {}. Please verify the currency codes are correct.", context).into()))?;
        let bid: f64 = rate_data.bid_price.parse().map_err(|_| {
            ToolCallError::RuntimeError(
                format!(
                    "💰 Invalid bid price format for {}. The API response may be corrupted.",
                    context
                )
                .into(),
            )
        })?;
        let ask: f64 = rate_data.ask_price.parse().map_err(|_| {
            ToolCallError::RuntimeError(
                format!(
                    "💰 Invalid ask price format for {}. The API response may be corrupted.",
                    context
                )
                .into(),
            )
        })?;

        Ok(ForexQuote {
            symbol: format!("{}/{}", from, to),
            bid,
            ask,
            price,
            timestamp: Utc::now(),
            change: 0.0, // Would need historical data to calculate
            change_percent: 0.0,
            volume: None,
        })
    }

    // Get historical OHLC data for technical analysis
    pub async fn get_forex_ohlc(
        &self,
        from: &str,
        to: &str,
        interval: &str,
    ) -> Result<Vec<OHLCData>, ToolCallError> {
        let context = format!("OHLC data {}/{} ({})", from, to, interval);
        let function = match interval {
            "1min" => "FX_INTRADAY",
            "daily" => "FX_DAILY",
            _ => "FX_DAILY",
        };

        let mut url = format!(
            "https://www.alphavantage.co/query?function={}&from_symbol={}&to_symbol={}&apikey={}",
            function, from, to, self.alpha_vantage_key
        );

        if interval == "1min" {
            url.push_str("&interval=1min");
        }

        let response = self.client.get(&url).send().await.map_err(|e| {
            ToolCallError::RuntimeError(Self::format_network_error(&e, &context).into())
        })?;

        if !response.status().is_success() {
            return Err(ToolCallError::RuntimeError(
                Self::format_http_error(response.status(), &context).into(),
            ));
        }

        let json_data: serde_json::Value = response.json().await.map_err(|e| {
            ToolCallError::RuntimeError(Self::format_network_error(&e, &context).into())
        })?;

        // Check for Alpha Vantage specific errors
        Self::check_alpha_vantage_error(&json_data, &context)?;

        let time_series = if interval == "1min" {
            json_data.get("Time Series FX (1min)")
        } else {
            json_data.get("Time Series FX (Daily)")
        };

        let mut ohlc_data = Vec::new();

        if let Some(series) = time_series {
            if let Some(obj) = series.as_object() {
                if obj.is_empty() {
                    return Err(ToolCallError::RuntimeError(
                        format!("📅 No {} data available. The currency pair may not be supported or data may not exist for the requested time period.", context).into()
                    ));
                }

                for (timestamp_str, values) in obj {
                    if let Ok(timestamp) = chrono::DateTime::parse_from_str(
                        &format!("{} 00:00:00 +0000", timestamp_str),
                        "%Y-%m-%d %H:%M:%S %z",
                    ) {
                        let open: f64 = values["1. open"]
                            .as_str()
                            .ok_or_else(|| {
                                ToolCallError::RuntimeError(
                                    format!(
                                        "📊 Missing open price in {} data for {}.",
                                        context, timestamp_str
                                    )
                                    .into(),
                                )
                            })?
                            .parse()
                            .map_err(|_| {
                                ToolCallError::RuntimeError(
                                    format!(
                                        "📊 Invalid open price format in {} data for {}.",
                                        context, timestamp_str
                                    )
                                    .into(),
                                )
                            })?;
                        let high: f64 = values["2. high"]
                            .as_str()
                            .ok_or_else(|| {
                                ToolCallError::RuntimeError(
                                    format!(
                                        "📊 Missing high price in {} data for {}.",
                                        context, timestamp_str
                                    )
                                    .into(),
                                )
                            })?
                            .parse()
                            .map_err(|_| {
                                ToolCallError::RuntimeError(
                                    format!(
                                        "📊 Invalid high price format in {} data for {}.",
                                        context, timestamp_str
                                    )
                                    .into(),
                                )
                            })?;
                        let low: f64 = values["3. low"]
                            .as_str()
                            .ok_or_else(|| {
                                ToolCallError::RuntimeError(
                                    format!(
                                        "📊 Missing low price in {} data for {}.",
                                        context, timestamp_str
                                    )
                                    .into(),
                                )
                            })?
                            .parse()
                            .map_err(|_| {
                                ToolCallError::RuntimeError(
                                    format!(
                                        "📊 Invalid low price format in {} data for {}.",
                                        context, timestamp_str
                                    )
                                    .into(),
                                )
                            })?;
                        let close: f64 = values["4. close"]
                            .as_str()
                            .ok_or_else(|| {
                                ToolCallError::RuntimeError(
                                    format!(
                                        "📊 Missing close price in {} data for {}.",
                                        context, timestamp_str
                                    )
                                    .into(),
                                )
                            })?
                            .parse()
                            .map_err(|_| {
                                ToolCallError::RuntimeError(
                                    format!(
                                        "📊 Invalid close price format in {} data for {}.",
                                        context, timestamp_str
                                    )
                                    .into(),
                                )
                            })?;

                        ohlc_data.push(OHLCData {
                            timestamp: timestamp.with_timezone(&Utc),
                            open,
                            high,
                            low,
                            close,
                            volume: 0.0, // Forex doesn't have traditional volume
                        });
                    } else {
                        return Err(ToolCallError::RuntimeError(
                            format!("📅 Invalid timestamp format '{}' in {} data. Expected YYYY-MM-DD format.", timestamp_str, context).into()
                        ));
                    }
                }
            } else {
                return Err(ToolCallError::RuntimeError(
                    format!("📊 Invalid {} data structure received from Alpha Vantage. The API response format may have changed.", context).into()
                ));
            }
        } else {
            return Err(ToolCallError::RuntimeError(
                format!("📊 No time series data found in {} response. Please verify the currency codes and try again.", context).into()
            ));
        }

        if ohlc_data.is_empty() {
            return Err(ToolCallError::RuntimeError(
                format!("📊 No valid {} data points found. The currency pair may not be supported or data may be unavailable.", context).into()
            ));
        }

        // Sort by timestamp (oldest first)
        ohlc_data.sort_by(|a, b| a.timestamp.cmp(&b.timestamp));
        Ok(ohlc_data)
    }

    // Calculate technical indicators
    pub fn calculate_technical_indicators(
        &self,
        ohlc_data: &[OHLCData],
    ) -> Result<TechnicalIndicators, ToolCallError> {
        if ohlc_data.len() < 50 {
            return Err(ToolCallError::RuntimeError(
                "Insufficient data for technical analysis (need at least 50 periods)".into(),
            ));
        }

        let closes: Vec<f64> = ohlc_data.iter().map(|d| d.close).collect();
        let _highs: Vec<f64> = ohlc_data.iter().map(|d| d.high).collect();
        let _lows: Vec<f64> = ohlc_data.iter().map(|d| d.low).collect();

        // RSI (14 period)
        let mut rsi_indicator = RelativeStrengthIndex::new(14).unwrap();
        let mut rsi_value = 50.0;
        for &close in &closes {
            rsi_value = rsi_indicator.next(close);
        }

        // Moving Averages
        let sma_20 = closes.iter().rev().take(20).sum::<f64>() / 20.0;
        let sma_50 = closes.iter().rev().take(50).sum::<f64>() / 50.0;

        // EMA calculation
        let mut ema_12 = closes[0];
        let mut ema_26 = closes[0];
        let alpha_12 = 2.0 / (12.0 + 1.0);
        let alpha_26 = 2.0 / (26.0 + 1.0);

        for &close in &closes[1..] {
            ema_12 = alpha_12 * close + (1.0 - alpha_12) * ema_12;
            ema_26 = alpha_26 * close + (1.0 - alpha_26) * ema_26;
        }

        // MACD
        let macd_line = ema_12 - ema_26;
        let signal_line = macd_line; // Simplified - should be EMA of MACD
        let histogram = macd_line - signal_line;

        // Bollinger Bands (20 period, 2 std dev)
        let mean = sma_20;
        let variance = closes
            .iter()
            .rev()
            .take(20)
            .map(|&x| (x - mean).powi(2))
            .sum::<f64>()
            / 20.0;
        let std_dev = variance.sqrt();

        // Stochastic (14 period)
        let recent_data = &ohlc_data[ohlc_data.len().saturating_sub(14)..];
        let highest_high = recent_data.iter().map(|d| d.high).fold(0.0, f64::max);
        let lowest_low = recent_data
            .iter()
            .map(|d| d.low)
            .fold(f64::INFINITY, f64::min);
        let current_close = closes[closes.len() - 1];
        let k_percent = ((current_close - lowest_low) / (highest_high - lowest_low)) * 100.0;

        Ok(TechnicalIndicators {
            rsi: rsi_value,
            macd: MACDData {
                macd: macd_line,
                signal: signal_line,
                histogram,
            },
            bollinger_bands: BollingerBands {
                upper: mean + (2.0 * std_dev),
                middle: mean,
                lower: mean - (2.0 * std_dev),
            },
            moving_averages: MovingAverages {
                sma_20,
                sma_50,
                ema_12,
                ema_26,
            },
            stochastic: StochasticData {
                k: k_percent,
                d: k_percent, // Simplified - should be SMA of %K
            },
        })
    }

    // Generate trading signals based on technical analysis
    pub fn generate_trading_signal(
        &self,
        quote: &ForexQuote,
        indicators: &TechnicalIndicators,
    ) -> TradingSignal {
        let mut signal_strength: f64 = 0.0;
        let mut reasoning_parts = Vec::new();

        // RSI Analysis
        if indicators.rsi < 30.0 {
            signal_strength += 0.3;
            reasoning_parts.push("RSI indicates oversold condition (bullish)");
        } else if indicators.rsi > 70.0 {
            signal_strength -= 0.3;
            reasoning_parts.push("RSI indicates overbought condition (bearish)");
        }

        // MACD Analysis
        if indicators.macd.macd > indicators.macd.signal && indicators.macd.histogram > 0.0 {
            signal_strength += 0.25;
            reasoning_parts.push("MACD bullish crossover");
        } else if indicators.macd.macd < indicators.macd.signal && indicators.macd.histogram < 0.0 {
            signal_strength -= 0.25;
            reasoning_parts.push("MACD bearish crossover");
        }

        // Moving Average Analysis
        if indicators.moving_averages.ema_12 > indicators.moving_averages.ema_26 {
            signal_strength += 0.2;
            reasoning_parts.push("Short-term EMA above long-term EMA (bullish trend)");
        } else {
            signal_strength -= 0.2;
            reasoning_parts.push("Short-term EMA below long-term EMA (bearish trend)");
        }

        // Bollinger Bands Analysis
        if quote.price <= indicators.bollinger_bands.lower {
            signal_strength += 0.15;
            reasoning_parts.push("Price at lower Bollinger Band (potential bounce)");
        } else if quote.price >= indicators.bollinger_bands.upper {
            signal_strength -= 0.15;
            reasoning_parts.push("Price at upper Bollinger Band (potential reversal)");
        }

        // Determine signal type and confidence
        let (signal_type, confidence): (SignalType, f64) = if signal_strength > 0.6 {
            (SignalType::StrongBuy, 0.8 + (signal_strength - 0.6) * 0.5)
        } else if signal_strength > 0.3 {
            (SignalType::Buy, 0.6 + (signal_strength - 0.3) * 0.67)
        } else if signal_strength < -0.6 {
            (
                SignalType::StrongSell,
                0.8 + (signal_strength.abs() - 0.6) * 0.5,
            )
        } else if signal_strength < -0.3 {
            (SignalType::Sell, 0.6 + (signal_strength.abs() - 0.3) * 0.67)
        } else {
            (SignalType::Hold, 0.5)
        };

        // Calculate stop loss and take profit levels
        let atr_estimate = (quote.ask - quote.bid) * 10.0; // Simplified ATR
        let (stop_loss, take_profit) = match signal_type {
            SignalType::Buy | SignalType::StrongBuy => (
                Some(quote.price - 2.0 * atr_estimate),
                Some(quote.price + 3.0 * atr_estimate),
            ),
            SignalType::Sell | SignalType::StrongSell => (
                Some(quote.price + 2.0 * atr_estimate),
                Some(quote.price - 3.0 * atr_estimate),
            ),
            SignalType::Hold => (None, None),
        };

        TradingSignal {
            signal_type,
            strength: signal_strength.abs().min(1.0),
            confidence: confidence.min(1.0),
            entry_price: quote.price,
            stop_loss,
            take_profit,
            reasoning: reasoning_parts.join("; "),
            timestamp: Utc::now(),
        }
    }
}

// Error handling helper functions for user-friendly messages
impl FinancialDataClient {
    /// Convert HTTP status codes to user-friendly error messages
    pub fn format_http_error(status: StatusCode, context: &str) -> String {
        match status {
            StatusCode::UNAUTHORIZED => {
                "🔑 Authentication failed. Please check your Alpha Vantage API key is correct and active.".to_string()
            }
            StatusCode::FORBIDDEN => {
                "🚫 Access denied. Your API key may not have permission for this operation.".to_string()
            }
            StatusCode::TOO_MANY_REQUESTS => {
                "⏰ Rate limit exceeded. Alpha Vantage allows 25 requests per day on free tier. Please wait or upgrade your plan.".to_string()
            }
            StatusCode::INTERNAL_SERVER_ERROR => {
                "🔧 Alpha Vantage server error. Please try again in a few minutes.".to_string()
            }
            StatusCode::SERVICE_UNAVAILABLE => {
                "🚧 Alpha Vantage service temporarily unavailable. Please try again later.".to_string()
            }
            StatusCode::BAD_REQUEST => {
                format!("📝 Invalid request parameters for {}. Please check currency codes and other parameters.", context)
            }
            _ => {
                format!("🌐 API request failed for {} (Status: {}). Please check your internet connection and try again.", context, status)
            }
        }
    }

    /// Format network errors with user-friendly messages
    pub fn format_network_error(error: &reqwest::Error, context: &str) -> String {
        if error.is_timeout() {
            format!(
                "⏱️ Request timeout for {}. Please check your internet connection and try again.",
                context
            )
        } else if error.is_connect() {
            format!("🌐 Cannot connect to Alpha Vantage API for {}. Please check your internet connection.", context)
        } else if error.is_decode() {
            format!(
                "📊 Unable to process response data for {}. The API may be experiencing issues.",
                context
            )
        } else {
            format!("🔌 Network error while fetching {}: {}. Please check your connection and try again.", context, error)
        }
    }

    /// Check if Alpha Vantage response contains error messages
    pub fn check_alpha_vantage_error(
        json_data: &serde_json::Value,
        context: &str,
    ) -> Result<(), ToolCallError> {
        // Check for common Alpha Vantage error patterns
        if let Some(error_message) = json_data.get("Error Message").and_then(|v| v.as_str()) {
            let friendly_message = if error_message.contains("Invalid API call") {
                format!("❌ Invalid request for {}: {}. Please check your currency codes (e.g., USD, EUR, GBP).", context, error_message)
            } else if error_message.contains("API key") {
                "🔑 API key issue: Please verify your Alpha Vantage API key is correct and active."
                    .to_string()
            } else {
                format!(
                    "⚠️ Alpha Vantage API error for {}: {}",
                    context, error_message
                )
            };
            return Err(ToolCallError::RuntimeError(friendly_message.into()));
        }

        // Check for rate limit messages
        if let Some(note) = json_data.get("Note").and_then(|v| v.as_str()) {
            if note.contains("API call frequency") {
                return Err(ToolCallError::RuntimeError(
                    "⏰ API rate limit reached. Free tier allows 25 requests per day. Please wait or upgrade your Alpha Vantage plan for higher limits.".into()
                ));
            }
        }

        // Check for information messages that might indicate issues
        if let Some(info) = json_data.get("Information").and_then(|v| v.as_str()) {
            if info.contains("API call frequency") {
                return Err(ToolCallError::RuntimeError(
                    "⏰ API rate limit reached. Free tier allows 25 requests per day. Please wait or upgrade your Alpha Vantage plan for higher limits.".into()
                ));
            }
        }

        Ok(())
    }

    /// Validate forex response format and provide detailed error information
    pub fn validate_forex_response_format(
        json_data: &serde_json::Value,
        context: &str,
    ) -> Result<(), ToolCallError> {
        // Check if the main forex response structure exists
        if let Some(forex_data) = json_data.get("Realtime Currency Exchange Rate") {
            // Validate required fields in the forex response
            let required_fields = [
                ("1. From_Currency Code", "from currency code"),
                ("2. From_Currency Name", "from currency name"),
                ("3. To_Currency Code", "to currency code"),
                ("4. To_Currency Name", "to currency name"),
                ("5. Exchange Rate", "exchange rate"),
                ("6. Last Refreshed", "last refreshed timestamp"),
                ("7. Time Zone", "time zone"),
                ("8. Bid Price", "bid price"),
                ("9. Ask Price", "ask price"),
            ];

            let mut missing_fields = Vec::new();
            let mut invalid_fields = Vec::new();

            for (field_key, field_name) in required_fields.iter() {
                match forex_data.get(field_key) {
                    Some(value) => {
                        if value.is_null()
                            || (value.is_string() && value.as_str().unwrap_or("").is_empty())
                        {
                            invalid_fields.push(*field_name);
                        }
                    }
                    None => {
                        missing_fields.push(*field_name);
                    }
                }
            }

            // Report specific issues found
            if !missing_fields.is_empty() {
                return Err(ToolCallError::RuntimeError(
                    format!("🔍 Missing required fields in {} response: {}. The Alpha Vantage API response format may have changed. Please try again later or contact support.", 
                        context, missing_fields.join(", ")).into()
                ));
            }

            if !invalid_fields.is_empty() {
                return Err(ToolCallError::RuntimeError(
                    format!("📊 Invalid or empty fields in {} response: {}. The API may be experiencing issues or the response format has changed.", 
                        context, invalid_fields.join(", ")).into()
                ));
            }

            // Validate that exchange rate, bid, and ask are parseable numbers
            if let Some(exchange_rate) = forex_data.get("5. Exchange Rate").and_then(|v| v.as_str())
            {
                if exchange_rate.parse::<f64>().is_err() {
                    return Err(ToolCallError::RuntimeError(
                        format!("💱 Invalid exchange rate format '{}' in {} response. Expected a numeric value.", exchange_rate, context).into()
                    ));
                }
            }

            if let Some(bid_price) = forex_data.get("8. Bid Price").and_then(|v| v.as_str()) {
                if bid_price.parse::<f64>().is_err() {
                    return Err(ToolCallError::RuntimeError(
                        format!("💰 Invalid bid price format '{}' in {} response. Expected a numeric value.", bid_price, context).into()
                    ));
                }
            }

            if let Some(ask_price) = forex_data.get("9. Ask Price").and_then(|v| v.as_str()) {
                if ask_price.parse::<f64>().is_err() {
                    return Err(ToolCallError::RuntimeError(
                        format!("💰 Invalid ask price format '{}' in {} response. Expected a numeric value.", ask_price, context).into()
                    ));
                }
            }
        } else {
            // Main forex data structure is missing
            return Err(ToolCallError::RuntimeError(
                format!("🔍 Missing 'Realtime Currency Exchange Rate' section in {} response. The Alpha Vantage API response format may have changed significantly. Please verify your currency codes and try again later.", context).into()
            ));
        }

        Ok(())
    }
}
