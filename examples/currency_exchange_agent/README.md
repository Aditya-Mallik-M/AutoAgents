# Currency Exchange Agent Example

This example demonstrates how to fetch and display the latest currency exchange rates using the exchangeratesapi.io API.

## Prerequisites

You'll need an API key from exchangeratesapi.io:

```bash
export EXCHANGE_API_KEY="your-exchange-api-key"
```

Get your free API key at [exchangeratesapi.io](https://exchangeratesapi.io/)

## Usage

### Running the example

```bash
cargo run -p currency-exchange-agent
```

This will fetch the latest exchange rates for EUR (base currency) against:
- INR (Indian Rupee)
- USD (US Dollar)
- GBP (British Pound)
- JPY (Japanese Yen)

## Features

- Fetches real-time exchange rates from exchangeratesapi.io
- Displays formatted output with colored text
- Handles API errors gracefully
- Uses EUR as base currency (supported by free tier)

## Example Output

```
Currency Exchange Rate Fetcher
Fetching latest exchange rates for EUR...

Latest Exchange Rates
Base Currency: EUR
Date: 2024-01-15

Exchange Rates:
GBP: 0.8642
INR: 91.2345
JPY: 164.7890
USD: 1.0876

Note: Using EUR as base currency (supported by free tier).
```

## API Information

This example uses the free tier of exchangeratesapi.io which:
- Supports EUR as the base currency
- Provides up to 1,000 API requests per month
- Returns real-time exchange rates

For more features like different base currencies, consider upgrading to a paid plan.
