use crate::api::FinancialDataClient;
use autoagents::core::tool::{ToolCallError, ToolInputT, ToolRuntime, ToolT};
use autoagents_derive::tool;
use serde::{Deserialize, Serialize};
use serde_json::Value;

// Advanced Trading Tool Input Types
#[derive(Serialize, Deserialize, Debug)]
pub struct TechnicalAnalysisArgs {
    pub from_currency: String,
    pub to_currency: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval: Option<String>, // "1min", "daily"
}

impl ToolInputT for TechnicalAnalysisArgs {
    fn io_schema() -> &'static str {
        r#"{"type":"object","properties":{"from_currency":{"type":"string","description":"Base currency code (e.g., USD, EUR)"},"to_currency":{"type":"string","description":"Target currency code (e.g., EUR, GBP, JPY)"},"interval":{"type":"string","description":"Time interval for analysis: '1min' for intraday or 'daily' for daily analysis. Default is 'daily'."}},"required":["from_currency","to_currency"],"additionalProperties":false}"#
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TradingSignalArgs {
    pub from_currency: String,
    pub to_currency: String,
}

impl ToolInputT for TradingSignalArgs {
    fn io_schema() -> &'static str {
        r#"{"type":"object","properties":{"from_currency":{"type":"string","description":"Base currency code (e.g., USD, EUR)"},"to_currency":{"type":"string","description":"Target currency code (e.g., EUR, GBP, JPY)"}},"required":["from_currency","to_currency"],"additionalProperties":false}"#
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ForexQuoteArgs {
    pub from_currency: String,
    pub to_currency: String,
}

impl ToolInputT for ForexQuoteArgs {
    fn io_schema() -> &'static str {
        r#"{"type":"object","properties":{"from_currency":{"type":"string","description":"Base currency code (e.g., USD, EUR)"},"to_currency":{"type":"string","description":"Target currency code (e.g., EUR, GBP, JPY)"}},"required":["from_currency","to_currency"],"additionalProperties":false}"#
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MarketAnalysisArgs {
    pub currency_pairs: String, // Comma-separated pairs like "USD/EUR,GBP/USD,USD/JPY"
}

impl ToolInputT for MarketAnalysisArgs {
    fn io_schema() -> &'static str {
        r#"{"type":"object","properties":{"currency_pairs":{"type":"string","description":"Comma-separated currency pairs to analyze (e.g., 'USD/EUR,GBP/USD,USD/JPY'). Use format FROM/TO for each pair."}},"required":["currency_pairs"],"additionalProperties":false}"#
    }
}

// Advanced Trading Tools
#[tool(
    name = "GetForexQuote",
    description = "Get real-time forex quote with bid/ask spread and professional trading data. Use this for live market data and trading decisions.",
    input = ForexQuoteArgs,
)]
pub struct GetForexQuoteTool {}

impl ToolRuntime for GetForexQuoteTool {
    fn execute(&self, args: Value) -> Result<Value, ToolCallError> {
        let args: ForexQuoteArgs = serde_json::from_value(args)
            .map_err(|e| ToolCallError::RuntimeError(format!("Invalid arguments: {}", e).into()))?;

        tokio::task::block_in_place(|| {
            tokio::runtime::Handle::current().block_on(async {
let client = FinancialDataClient::get_instance()?;
let quote = client.get_forex_quote(&args.from_currency, &args.to_currency).await?;
let response = serde_json::json!({
    "success": true,
    "quote": quote,
    "analysis": format!("Live {} quote: Bid: {:.5}, Ask: {:.5}, Spread: {:.5} pips", 
        quote.symbol, quote.bid, quote.ask, (quote.ask - quote.bid) * 10000.0),
    "recommendations": format!("Current market conditions for {}. Spread indicates market liquidity.", quote.symbol)
});

Ok(response)
            })
        })
    }
}

#[tool(
    name = "GetTechnicalAnalysis", 
    description = "Perform comprehensive technical analysis with RSI, MACD, Bollinger Bands, and moving averages. Essential for trading decisions.",
    input = TechnicalAnalysisArgs,
)]
pub struct GetTechnicalAnalysisTool {}

impl ToolRuntime for GetTechnicalAnalysisTool {
    fn execute(&self, args: Value) -> Result<Value, ToolCallError> {
        let args: TechnicalAnalysisArgs = serde_json::from_value(args)
            .map_err(|e| ToolCallError::RuntimeError(format!("Invalid arguments: {}", e).into()))?;

        tokio::task::block_in_place(|| {
            tokio::runtime::Handle::current().block_on(async {
let client = FinancialDataClient::get_instance()?;
let interval = args.interval.as_deref().unwrap_or("daily");
// Get OHLC data
let ohlc_data = client.get_forex_ohlc(&args.from_currency, &args.to_currency, interval).await?;
if ohlc_data.len() < 50 {
    return Ok(serde_json::json!({
        "success": false,
        "error": "Insufficient historical data for technical analysis. Need at least 50 data points.",
        "data_points": ohlc_data.len()
    }));
}

// Calculate technical indicators
let indicators = client.calculate_technical_indicators(&ohlc_data)?;
// Generate analysis
let mut analysis_parts = Vec::new();
// RSI Analysis
if indicators.rsi < 30.0 {
    analysis_parts.push(format!("🔴 RSI ({:.1}) indicates OVERSOLD condition - potential BUY opportunity", indicators.rsi));
} else if indicators.rsi > 70.0 {
    analysis_parts.push(format!("🔴 RSI ({:.1}) indicates OVERBOUGHT condition - potential SELL opportunity", indicators.rsi));
} else {
    analysis_parts.push(format!("🟡 RSI ({:.1}) in neutral zone", indicators.rsi));
}

// MACD Analysis
if indicators.macd.macd > indicators.macd.signal {
    analysis_parts.push("🟢 MACD shows bullish momentum".to_string());
} else {
    analysis_parts.push("🔴 MACD shows bearish momentum".to_string());
}

// Moving Average Analysis
if indicators.moving_averages.ema_12 > indicators.moving_averages.ema_26 {
    analysis_parts.push("🟢 Short-term trend is BULLISH (EMA12 > EMA26)".to_string());
} else {
    analysis_parts.push("🔴 Short-term trend is BEARISH (EMA12 < EMA26)".to_string());
}

let response = serde_json::json!({
    "success": true,
    "pair": format!("{}/{}", args.from_currency, args.to_currency),
    "interval": interval,
    "indicators": indicators,
    "analysis": analysis_parts.join("; "),
    "data_points": ohlc_data.len(),
    "recommendations": "Use these indicators in combination for trading decisions. RSI for entry/exit timing, MACD for momentum confirmation, and moving averages for trend direction."
});

Ok(response)
            })
        })
    }
}

#[tool(
    name = "GenerateTradingSignal",
    description = "Generate intelligent BUY/SELL/HOLD signals with confidence scores, entry prices, stop-loss and take-profit levels based on technical analysis.",
    input = TradingSignalArgs,
)]
pub struct GenerateTradingSignalTool {}

impl ToolRuntime for GenerateTradingSignalTool {
    fn execute(&self, args: Value) -> Result<Value, ToolCallError> {
        let args: TradingSignalArgs = serde_json::from_value(args)
            .map_err(|e| ToolCallError::RuntimeError(format!("Invalid arguments: {}", e).into()))?;

        tokio::task::block_in_place(|| {
            tokio::runtime::Handle::current().block_on(async {
let client = FinancialDataClient::get_instance()?;
// Get current quote
let quote = client.get_forex_quote(&args.from_currency, &args.to_currency).await?;
// Get OHLC data for technical analysis
let ohlc_data = client.get_forex_ohlc(&args.from_currency, &args.to_currency, "daily").await?;
if ohlc_data.len() < 50 {
    return Ok(serde_json::json!({
        "success": false,
        "error": "Insufficient data for signal generation",
        "recommendation": "Wait for more historical data to accumulate"
    }));
}

// Calculate indicators and generate signal
let indicators = client.calculate_technical_indicators(&ohlc_data)?;
let signal = client.generate_trading_signal(&quote, &indicators);
// Format signal strength and confidence as percentages
let strength_pct = (signal.strength * 100.0).round() as u32;
let confidence_pct = (signal.confidence * 100.0).round() as u32;
// Generate emoji based on signal type
let signal_emoji = match signal.signal_type {
    crate::api::SignalType::StrongBuy => "🚀",
    crate::api::SignalType::Buy => "📈", 
    crate::api::SignalType::Hold => "⏸️",
    crate::api::SignalType::Sell => "📉",
    crate::api::SignalType::StrongSell => "💥",
};

let response = serde_json::json!({
    "success": true,
    "pair": format!("{}/{}", args.from_currency, args.to_currency),
    "signal": {
        "type": format!("{:?}", signal.signal_type),
        "emoji": signal_emoji,
        "strength": format!("{}%", strength_pct),
        "confidence": format!("{}%", confidence_pct),
        "entry_price": signal.entry_price,
        "stop_loss": signal.stop_loss,
        "take_profit": signal.take_profit,
        "reasoning": signal.reasoning,
        "timestamp": signal.timestamp
    },
    "current_quote": {
        "bid": quote.bid,
        "ask": quote.ask,
        "spread_pips": (quote.ask - quote.bid) * 10000.0
    },
    "analysis": format!("{} {} signal with {}% confidence. {}", 
        signal_emoji,
        format!("{:?}", signal.signal_type).to_uppercase(), 
        confidence_pct,
        signal.reasoning
    ),
    "recommendations": match signal.signal_type {
        crate::api::SignalType::StrongBuy | crate::api::SignalType::Buy =>
            "Consider LONG position. Monitor stop-loss levels closely.",
        crate::api::SignalType::StrongSell | crate::api::SignalType::Sell =>
            "Consider SHORT position. Watch for trend reversal signals.",
        crate::api::SignalType::Hold =>
            "Wait for clearer market direction. Monitor key support/resistance levels."
    }
});

Ok(response)
            })
        })
    }
}

#[tool(
    name = "AnalyzeMarketOverview",
    description = "Comprehensive market analysis across multiple currency pairs with correlations, trends, and trading opportunities.",
    input = MarketAnalysisArgs,
)]
pub struct AnalyzeMarketOverviewTool {}

impl ToolRuntime for AnalyzeMarketOverviewTool {
    fn execute(&self, args: Value) -> Result<Value, ToolCallError> {
        let args: MarketAnalysisArgs = serde_json::from_value(args)
            .map_err(|e| ToolCallError::RuntimeError(format!("Invalid arguments: {}", e).into()))?;

        tokio::task::block_in_place(|| {
            tokio::runtime::Handle::current().block_on(async {
let client = FinancialDataClient::get_instance()?;
let mut market_data = Vec::new();
let mut signals = Vec::new();
// Parse currency pairs
for pair in args.currency_pairs.split(',') {
    let pair = pair.trim();
    if let Some((from, to)) = pair.split_once('/') {
        let from = from.trim();
        let to = to.trim();
        // Get quote and generate signal for each pair
        match client.get_forex_quote(from, to).await {
            Ok(quote) => {
// Try to get technical analysis
match client.get_forex_ohlc(from, to, "daily").await {
    Ok(ohlc_data) if ohlc_data.len() >= 50 => {
        if let Ok(indicators) = client.calculate_technical_indicators(&ohlc_data) {
            let signal = client.generate_trading_signal(&quote, &indicators);
            market_data.push(serde_json::json!({
"pair": pair,
"price": quote.price,
"bid": quote.bid,
"ask": quote.ask,
"spread_pips": (quote.ask - quote.bid) * 10000.0,
"rsi": indicators.rsi,
"signal": format!("{:?}", signal.signal_type),
"confidence": (signal.confidence * 100.0).round()
            }));
            signals.push(serde_json::json!({
"pair": pair,
"signal": format!("{:?}", signal.signal_type),
"strength": (signal.strength * 100.0).round(),
"confidence": (signal.confidence * 100.0).round(),
"reasoning": signal.reasoning
            }));
        }
    },
    _ => {
        // Just add basic quote data
        market_data.push(serde_json::json!({
            "pair": pair,
            "price": quote.price,
            "bid": quote.bid,
            "ask": quote.ask,
            "spread_pips": (quote.ask - quote.bid) * 10000.0,
            "note": "Insufficient data for technical analysis"
        }));
    }
}
            },
            Err(_) => {
market_data.push(serde_json::json!({
    "pair": pair,
    "error": "Failed to fetch data"
}));
            }
        }
    }
}

// Generate market summary
let buy_signals = signals.iter().filter(|s|
    s["signal"].as_str().unwrap_or("").contains("Buy")).count();
let sell_signals = signals.iter().filter(|s|
    s["signal"].as_str().unwrap_or("").contains("Sell")).count();
let hold_signals = signals.iter().filter(|s|
    s["signal"].as_str().unwrap_or("") == "Hold").count();

let market_sentiment = if buy_signals > sell_signals {
    "🟢 BULLISH - More buy opportunities detected"
} else if sell_signals > buy_signals {
    "🔴 BEARISH - More sell opportunities detected"  
} else {
    "🟡 NEUTRAL - Mixed signals across pairs"
};

let response = serde_json::json!({
    "success": true,
    "market_overview": {
        "pairs_analyzed": market_data.len(),
        "market_sentiment": market_sentiment,
        "signal_distribution": {
            "buy_signals": buy_signals,
            "sell_signals": sell_signals,
            "hold_signals": hold_signals
        }
    },
    "market_data": market_data,
    "trading_signals": signals,
    "analysis": format!("Market analysis complete for {} pairs. {} buy signals, {} sell signals, {} hold signals detected.", 
        market_data.len(), buy_signals, sell_signals, hold_signals),
    "recommendations": "Focus on pairs with high-confidence signals. Diversify across different currency regions to manage risk."
});

Ok(response)
            })
        })
    }
}
