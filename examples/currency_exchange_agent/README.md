# 🌍 Professional AI Forex Trading Agent 💱📈🤖

An intelligent, **professional-grade AI-powered forex trading platform** built with AutoAgents framework and Alpha Vantage API. Features advanced technical analysis, real-time trading signals, comprehensive market analysis, and AI-driven trading recommendations. Get institutional-quality forex data, technical indicators, and autonomous trading insights powered by advanced LLM technology.

## ✨ Features

### 📊 Professional Trading Features
- 🎯 **Professional Forex Quotes**: Real-time bid/ask spreads with pip calculations
- 📈 **Technical Analysis**: RSI, MACD, Bollinger Bands, Moving Averages (EMA/SMA)
- 🤖 **AI Trading Signals**: Intelligent BUY/SELL/HOLD recommendations with confidence scores
- 🎯 **Entry/Exit Points**: Automated stop-loss and take-profit level calculations
- 🌍 **Market Overview**: Multi-pair analysis with correlation insights
- ⚡ **Real-time Data**: Powered by Alpha Vantage professional financial data
- 📊 **OHLC Data**: Complete candlestick data for technical analysis

### 🔄 Autonomous Monitoring System
- ⏰ **Real-time Monitoring**: Continuous monitoring of currency rates (configurable intervals)
- 🧠 **AI Market Analysis**: Intelligent analysis of rate changes and market consequences
- 💼 **Portfolio Management**: Track holdings, profit/loss, and transaction history
- 🎯 **Automatic Trading Suggestions**: AI-generated buy/sell recommendations with reasoning
- 📊 **Mathematical Calculations**: Precise profit/loss calculations and portfolio valuation
- ⚠️ **Risk Management**: Built-in stop-loss and take-profit mechanisms
- 📈 **Performance Tracking**: Real-time portfolio performance and transaction history
- 🚨 **Market Alerts**: Notifications for significant rate changes and trading opportunities

### 🛠️ Technical Excellence
- 🎨 **Beautiful CLI Interface**: Colorful, emoji-rich interactive chat experience
- 🛠️ **AutoAgents Framework**: Built with modern agent architecture using tools and executors
- 🛡️ **Enhanced Error Handling**: User-friendly error messages with emojis, input validation, and actionable guidance
- 🧪 **Built-in Testing**: Comprehensive error handling test suite with `--test-errors` flag
- ⚡ **High Performance**: Async Rust implementation with efficient API calls
- 🔍 **Alpha Vantage Integration**: Professional-grade API error detection and rate limit handling

## 🚀 Prerequisites

### API Keys Required

1. **Alpha Vantage API** (required for advanced trading features):
```bash
export ALPHA_VANTAGE_API_KEY="your-alpha-vantage-api-key"
```
Get your free API key at [Alpha Vantage](https://www.alphavantage.co/support/#api-key) - provides professional-grade financial data with real-time forex quotes, technical indicators, and OHLC data

2. **LLM Provider API Key** (choose one):
```bash
# For OpenAI
export OPENAI_API_KEY="your-openai-api-key"

# For Anthropic
export ANTHROPIC_API_KEY="your-anthropic-api-key"

# For Ollama (no API key needed, but requires local installation)
# Install Ollama from https://ollama.ai/
```

## 📦 Installation & Usage

### Running the Agent

#### Interactive Chat Mode (Default)
```bash
# With OpenAI (recommended)
cargo run -p currency-exchange-agent -- --llm openai --model gpt-4o-mini

# With Anthropic
cargo run -p currency-exchange-agent -- --llm anthropic --model claude-3-sonnet-20240229

# With Ollama (local)
cargo run -p currency-exchange-agent -- --llm ollama --model llama2

# Explicitly enable interactive mode (optional, as it's default)
cargo run -p currency-exchange-agent -- --llm openai --model gpt-4o-mini --interactive
```

#### Single Query Mode
```bash
# Process a single query and exit
cargo run -p currency-exchange-agent -- --llm openai --model gpt-4o-mini --query "What's the current USD to EUR exchange rate?"
```

#### Autonomous Monitoring Mode
```bash
# Start autonomous currency monitoring with $1000 USD initial investment
cargo run -p currency-exchange-agent -- --llm openai --model gpt-4o-mini --monitor --initial-amount 1000 --initial-currency USD

# Monitor with custom interval (every 30 seconds)
cargo run -p currency-exchange-agent -- --llm openai --model gpt-4o-mini --monitor --initial-amount 500 --initial-currency EUR --interval 30

# Monitor with different LLM provider
cargo run -p currency-exchange-agent -- --llm anthropic --model claude-3-sonnet-20240229 --monitor --initial-amount 2000 --initial-currency GBP
```

#### Error Handling Testing Mode
```bash
# Test the improved error handling system (no API keys required for testing)
cargo run -p currency-exchange-agent -- --test-errors

# This will validate:
# ✅ User-friendly error messages with emojis
# ✅ Input validation (negative amounts, invalid dates)
# ✅ HTTP status code error formatting
# ✅ Alpha Vantage specific error detection
# ✅ Missing API key handling
```

### 🗣️ Natural Language Examples

Simply ask questions in plain English! The agent understands various ways to express currency-related queries:

#### Professional Trading Queries
```
💱 Ask me anything about currencies > Give me a live forex quote for EUR/USD
💱 Ask me anything about currencies > Analyze EUR/USD with technical indicators
💱 Ask me anything about currencies > Should I buy or sell GBP/USD right now?
💱 Ask me anything about currencies > What's the RSI for USD/JPY?
💱 Ask me anything about currencies > Generate trading signals for major currency pairs
💱 Ask me anything about currencies > Give me a market overview for USD/EUR, GBP/USD, USD/JPY
💱 Ask me anything about currencies > What are the MACD signals for EUR/GBP?
```

#### Professional Trading Analysis
```
💱 Ask me anything about currencies > Perform technical analysis on AUD/USD
💱 Ask me anything about currencies > What are the Bollinger Bands telling us about EUR/CHF?
💱 Ask me anything about currencies > Give me entry and exit points for USD/CAD
💱 Ask me anything about currencies > Analyze market sentiment across major forex pairs
```

## 🎯 Sample Interactions

### Advanced Trading Signal Generation
```
💱 Ask me anything about currencies > Should I buy or sell EUR/USD right now?

🤔 Thinking...

💡 Analysis:
I'll analyze EUR/USD using technical indicators and generate an intelligent trading signal with entry/exit points.

📊 Data:
{
  "success": true,
  "pair": "EUR/USD",
  "signal": {
    "type": "Buy",
    "emoji": "📈",
    "strength": "75%",
    "confidence": "82%",
    "entry_price": 1.0856,
    "stop_loss": 1.0798,
    "take_profit": 1.0945,
    "reasoning": "RSI (28.5) indicates oversold conditions. MACD shows bullish crossover. EMA12 > EMA26 confirms upward momentum.",
    "timestamp": "2024-01-15T14:30:00Z"
  },
  "current_quote": {
    "bid": 1.0854,
    "ask": 1.0858,
    "spread_pips": 0.4
  }
}

🎯 Recommendations:
📈 BUY signal with 82% confidence. Consider LONG position. Monitor stop-loss levels closely.
Current market shows oversold RSI with bullish MACD crossover - strong technical setup for upward movement.
```

### Professional Technical Analysis
```
💱 Ask me anything about currencies > Analyze GBP/USD with technical indicators

🤔 Thinking...

💡 Analysis:
I'll perform comprehensive technical analysis on GBP/USD including RSI, MACD, Bollinger Bands, and moving averages.

📊 Data:
{
  "success": true,
  "pair": "GBP/USD",
  "interval": "daily",
  "indicators": {
    "rsi": 67.3,
    "macd": {
      "macd": 0.0023,
      "signal": 0.0019,
      "histogram": 0.0004
    },
    "bollinger_bands": {
      "upper": 1.2745,
      "middle": 1.2678,
      "lower": 1.2611
    },
    "moving_averages": {
      "sma_20": 1.2665,
      "ema_12": 1.2689,
      "ema_26": 1.2654
    }
  },
  "analysis": "🟡 RSI (67.3) in neutral zone; 🟢 MACD shows bullish momentum; 🟢 Short-term trend is BULLISH (EMA12 > EMA26)",
  "data_points": 126
}

🎯 Recommendations:
Use these indicators in combination for trading decisions. RSI for entry/exit timing, MACD for momentum confirmation, and moving averages for trend direction.
```

### Natural Language Query - Currency Conversion
```
💱 Ask me anything about currencies > Convert 500 dollars to British pounds

🤔 Thinking...

💡 Analysis:
I'll convert 500 USD to GBP using the current exchange rate from our currency conversion API.

📊 Data:
{
  "success": true,
  "query": {
    "from": "USD",
    "to": "GBP", 
    "amount": 500
  },
  "result": 394.55
}

🎯 Recommendations:
500 USD converts to 394.55 GBP at the current exchange rate of 1 USD = 0.7891 GBP. Exchange rates fluctuate throughout the day, so this conversion reflects the most recent available rate.
```

### Natural Language Query - Historical Analysis
```
💱 Ask me anything about currencies > How did EUR perform against USD between January 1st and January 31st, 2024?

🤔 Thinking...

💡 Analysis:
I'll analyze the EUR/USD fluctuation between January 1st and January 31st, 2024 to show you the performance and changes during that period.

📊 Data:
{
  "success": true,
  "fluctuation": true,
  "start_date": "2024-01-01",
  "end_date": "2024-01-31", 
  "base": "USD",
  "rates": {
    "EUR": {
      "start_rate": 0.9156,
      "end_rate": 0.9234,
      "change": 0.0078,
      "change_pct": 0.85
    }
  }
}

🎯 Recommendations:
EUR strengthened against USD during January 2024, gaining 0.85% (+0.0078 points). The EUR went from 0.9156 to 0.9234 per USD, indicating a positive trend for the Euro during this period.
```

## 🏗️ Architecture

The agent is built with the **AutoAgents Framework** and includes:

### Core Technologies
- **AutoAgents**: Modern agent framework with LLM integration
- **Rust**: High-performance, memory-safe systems programming  
- **ReActExecutor**: Reasoning and Acting pattern for agent decision-making
- **Tool System**: Modular tools for different currency operations
- **LLM Integration**: OpenAI, Anthropic, or Ollama for natural language understanding

### Agent Components
- **CurrencyExchangeAgent**: Main agent with ReActExecutor and professional trading capabilities
- **Professional Trading Tools**: 4 Alpha Vantage-powered trading tools
  - `GetForexQuote`: Real-time forex quotes with bid/ask spreads and professional trading data
  - `GetTechnicalAnalysis`: Comprehensive technical analysis with RSI, MACD, Bollinger Bands, Moving Averages
  - `GenerateTradingSignal`: AI-powered BUY/SELL/HOLD signals with confidence scores, entry/exit points
  - `AnalyzeMarketOverview`: Multi-pair market analysis with correlations and trading opportunities
- **Financial Data Client**: Alpha Vantage integration for professional-grade data
- **Technical Analysis Engine**: Advanced indicator calculations and signal generation
- **Event System**: Real-time event handling and logging
- **Memory System**: Sliding window memory for conversation context

### Dependencies
- **tokio**: Async runtime for handling HTTP requests
- **reqwest**: HTTP client for API communication
- **serde**: JSON serialization/deserialization with chrono support
- **clap**: Command line argument parsing
- **colored**: Beautiful terminal output
- **tokio-stream**: Streaming support for real-time responses
- **ta**: Technical analysis indicators (RSI, MACD, Bollinger Bands)
- **statrs**: Statistical functions for advanced calculations
- **uuid**: Unique identifier generation for trading signals
- **chrono**: Date and time handling for market data

## 🔧 Technical Details

### AutoAgents Tools
Each tool is defined with the `#[tool]` macro and implements:
- **Input validation**: Structured input types with JSON schemas
- **Error handling**: Comprehensive error handling with user-friendly messages
- **Async execution**: Non-blocking API calls with proper error propagation
- **Structured output**: JSON responses with analysis and recommendations

### Alpha Vantage API Endpoints
- **FX_DAILY** - Daily OHLC forex data for technical analysis
- **FX_INTRADAY** - Intraday forex data with 1min, 5min intervals
- **CURRENCY_EXCHANGE_RATE** - Real-time forex quotes with bid/ask spreads
- **Technical Indicators** - Built-in RSI, MACD, Bollinger Bands calculations
- **170+ world currencies** supported with professional-grade data

### LLM Integration
- **Natural Language Understanding**: Interprets user queries and selects appropriate tools
- **Context Awareness**: Maintains conversation context for follow-up questions
- **Intelligent Routing**: Automatically determines which currency operations to perform
- **Response Generation**: Provides analysis, insights, and recommendations

### Data Sources
- **Alpha Vantage API**: Professional financial data provider with institutional-grade forex data
- **170+ world currencies** supported with real-time quotes and technical analysis
- **Historical OHLC data** for comprehensive technical indicator calculations
- **Real-time bid/ask spreads** for professional trading applications
- **Technical Indicators**: Built-in RSI, MACD, Bollinger Bands, and moving averages
- **Professional Trading Signals**: AI-powered BUY/SELL/HOLD recommendations

## 📊 API Information

### Alpha Vantage API
This agent uses **Alpha Vantage API exclusively** which provides:
- **Free Tier**: 25 requests/day for forex data and technical indicators
- **Paid Plans**: Higher limits, real-time data, premium endpoints
- **Professional Data**: Institutional-grade forex quotes with bid/ask spreads
- **Technical Indicators**: Built-in RSI, MACD, Bollinger Bands, and more
- **OHLC Data**: Complete candlestick data for technical analysis
- **SSL Security**: All requests use HTTPS encryption

### Supported Features
- Real-time forex quotes with bid/ask spreads
- Historical OHLC data for technical analysis
- Advanced technical indicators (RSI, MACD, Bollinger Bands, Moving Averages)
- AI-powered trading signal generation
- Multi-currency portfolio monitoring
- Professional-grade financial data

## 🚀 Getting Started

1. **Set up API keys**:
   ```bash
   export ALPHA_VANTAGE_API_KEY="your_alpha_vantage_api_key"
   export OPENAI_API_KEY="your_openai_api_key"  # or other LLM provider
   ```

2. **Run the agent**:
   ```bash
   cargo run -p currency-exchange-agent -- --llm openai --model gpt-4o-mini
   ```

3. **Start chatting**:
   ```
   💱 Ask me anything about currencies > What's the current USD to EUR rate?
   ```

4. **Exit when done**:
   ```
   💱 Ask me anything about currencies > exit
   ```

## 🛡️ Enhanced Error Handling

The currency exchange agent features comprehensive, user-friendly error handling that transforms cryptic API errors into clear, actionable guidance.

### 🧪 Testing Error Handling

Run the built-in error handling test suite:
```bash
cargo run -p currency-exchange-agent -- --test-errors
```

This validates:
- ✅ **Missing API Key Detection**: Clear guidance on obtaining and setting API keys
- ✅ **Input Validation**: Helpful messages for invalid amounts, dates, and formats
- ✅ **HTTP Error Translation**: User-friendly explanations for status codes
- ✅ **Alpha Vantage Specific Errors**: Rate limit detection and API-specific guidance
- ✅ **Network Error Handling**: Clear messages for connection and timeout issues

### 🎯 Error Message Features

**Emoji-Enhanced Messages**: Visual indicators for different error types
```
🔑 Authentication failed. Please check your Alpha Vantage API key is correct and active.
⏰ Rate limit exceeded. Alpha Vantage allows 25 requests per day on free tier.
📅 Invalid date format 'invalid-date'. Please use YYYY-MM-DD format (e.g., 2024-01-15).
💰 Invalid amount: -100. Amount must be greater than zero.
```

**Actionable Guidance**: Specific steps to resolve issues
- Direct links to API key registration
- Format examples for dates and currencies
- Rate limit explanations with upgrade suggestions
- Network troubleshooting tips

**Graceful Degradation**: Partial success with warnings
- Continues processing successful currency pairs
- Logs warnings for failed operations
- Provides comprehensive results when possible

## 🔍 Troubleshooting

### Common Issues

1. **API Key Errors**: Ensure your `ALPHA_VANTAGE_API_KEY` is set and valid
2. **LLM Provider Issues**: Check that your LLM API key is correctly configured
3. **Network Errors**: Verify internet connection and API endpoint availability
4. **Rate Limits**: Free tier has 25 requests/day limit for Alpha Vantage API

### Error Testing

If you encounter issues, run the error handling tests to verify the system is working correctly:
```bash
cargo run -p currency-exchange-agent -- --test-errors
```

This will help identify if the problem is with your setup or the application itself.

### Debug Mode

For verbose output, you can modify the event handling in the code to show more detailed logs.

## ⚠️ Disclaimer

This tool is for informational purposes only. Currency data is provided for educational and informational purposes. Always consult with financial professionals for investment decisions. The AutoAgents framework and this agent are not responsible for any financial losses.

## 📄 License

This project is part of the AutoAgents framework and follows the same licensing terms.

## 🤝 Contributing

Feel free to contribute by:
- **Adding new tools**: Implement additional currency analysis features
- **Improving LLM integration**: Enhance natural language understanding
- **Adding more data sources**: Integrate additional financial APIs
- **Enhancing user experience**: Improve the chat interface and output formatting
- **Adding tests**: Implement comprehensive testing for tools and agent behavior

---

**Built with ❤️ using [AutoAgents Framework](https://github.com/windsurf-ai/AutoAgents)**
