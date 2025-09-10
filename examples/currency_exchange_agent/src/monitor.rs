use crate::api::FinancialDataClient;
use autoagents::{core::error::Error, llm::LLMProvider};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use tokio::time::{interval, sleep};

// Portfolio and monitoring data structures
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Portfolio {
    pub holdings: HashMap<String, f64>, // currency -> amount
    pub initial_investment: f64,
    pub initial_currency: String,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
    pub total_transactions: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateSnapshot {
    pub timestamp: DateTime<Utc>,
    pub rates: HashMap<String, f64>, // currency_pair -> rate
    pub base_currency: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateChange {
    pub currency_pair: String,
    pub old_rate: f64,
    pub new_rate: f64,
    pub change_percent: f64,
    pub change_absolute: f64,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradingRecommendation {
    pub action: String, // "BUY", "SELL", "HOLD"
    pub from_currency: String,
    pub to_currency: String,
    pub amount: f64,
    pub expected_profit: f64,
    pub confidence: f64,
    pub reasoning: String,
    pub risk_level: String, // "LOW", "MEDIUM", "HIGH"
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionResult {
    pub transaction_id: String,
    pub from_currency: String,
    pub to_currency: String,
    pub amount_from: f64,
    pub amount_to: f64,
    pub rate_used: f64,
    pub profit_loss: f64,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonitoringConfig {
    pub monitoring_interval_seconds: u64,
    pub significant_change_threshold: f64, // percentage
    pub monitored_pairs: Vec<String>,
    pub max_risk_per_trade: f64,    // percentage of portfolio
    pub stop_loss_threshold: f64,   // percentage
    pub take_profit_threshold: f64, // percentage
}

impl Default for MonitoringConfig {
    fn default() -> Self {
        Self {
            monitoring_interval_seconds: 60,   // 1 minute
            significant_change_threshold: 0.5, // 0.5% change triggers analysis
            monitored_pairs: vec![
                "USD/EUR".to_string(),
                "USD/GBP".to_string(),
                "USD/JPY".to_string(),
                "EUR/GBP".to_string(),
                "GBP/JPY".to_string(),
                "USD/CHF".to_string(),
                "USD/CAD".to_string(),
                "AUD/USD".to_string(),
            ],
            max_risk_per_trade: 10.0,   // 10% of portfolio per trade
            stop_loss_threshold: -2.0,  // -2% stop loss
            take_profit_threshold: 3.0, // 3% take profit
        }
    }
}

pub struct CurrencyMonitor {
    pub portfolio: Portfolio,
    pub config: MonitoringConfig,
    pub rate_history: Vec<RateSnapshot>,
    pub llm: Arc<dyn LLMProvider>,
    pub client: FinancialDataClient,
    pub is_running: bool,
}

impl CurrencyMonitor {
    pub fn new(
        initial_amount: f64,
        initial_currency: String,
        llm: Arc<dyn LLMProvider>,
        config: Option<MonitoringConfig>,
    ) -> Result<Self, Error> {
        let mut holdings = HashMap::new();
        holdings.insert(initial_currency.clone(), initial_amount);

        let portfolio = Portfolio {
            holdings,
            initial_investment: initial_amount,
            initial_currency: initial_currency.clone(),
            created_at: Utc::now(),
            last_updated: Utc::now(),
            total_transactions: 0,
        };

        let client = FinancialDataClient::get_instance()
            .map_err(|e| Error::CustomError(format!("Failed to create financial client: {}", e)))?;

        Ok(Self {
            portfolio,
            config: config.unwrap_or_default(),
            rate_history: Vec::new(),
            llm,
            client,
            is_running: false,
        })
    }

    pub async fn start_monitoring(&mut self) -> Result<(), Error> {
        println!("🚀 Starting Currency Exchange Monitor...");
        println!(
            "💰 Initial Portfolio: {:.2} {}",
            self.portfolio.initial_investment, self.portfolio.initial_currency
        );
        println!(
            "⏰ Monitoring interval: {} seconds",
            self.config.monitoring_interval_seconds
        );
        println!(
            "📊 Watching {} currency pairs",
            self.config.monitored_pairs.len()
        );

        self.is_running = true;
        let mut interval = interval(Duration::from_secs(self.config.monitoring_interval_seconds));

        // Take initial snapshot
        self.take_rate_snapshot().await?;

        while self.is_running {
            interval.tick().await;

            match self.monitoring_cycle().await {
                Ok(_) => {}
                Err(e) => {
                    eprintln!("❌ Error in monitoring cycle: {}", e);
                    // Continue monitoring despite errors
                }
            }
        }

        Ok(())
    }

    async fn monitoring_cycle(&mut self) -> Result<(), Error> {
        println!("\n🔍 Checking rates at {}", Utc::now().format("%H:%M:%S"));

        // Take new rate snapshot
        let new_snapshot = self.create_rate_snapshot().await?;

        // Compare with previous snapshot if available
        if let Some(previous_snapshot) = self.rate_history.last() {
            let changes = self.detect_significant_changes(previous_snapshot, &new_snapshot);

            if !changes.is_empty() {
                println!("📈 Detected {} significant rate changes", changes.len());

                // Analyze changes with AI
                let analysis = self.analyze_market_changes(&changes).await?;

                // Generate trading recommendations
                let recommendations = self
                    .generate_trading_recommendations(&changes, &analysis)
                    .await?;

                // Execute recommended trades (if any)
                for recommendation in recommendations {
                    self.execute_recommendation(&recommendation).await?;
                }

                // Update portfolio value
                self.update_portfolio_value().await?;
            } else {
                println!("📊 No significant changes detected");
            }
        }

        // Store the new snapshot
        self.rate_history.push(new_snapshot);

        // Keep only last 100 snapshots to manage memory
        if self.rate_history.len() > 100 {
            self.rate_history.remove(0);
        }

        Ok(())
    }

    async fn take_rate_snapshot(&mut self) -> Result<(), Error> {
        let snapshot = self.create_rate_snapshot().await?;
        self.rate_history.push(snapshot);
        Ok(())
    }

    async fn create_rate_snapshot(&self) -> Result<RateSnapshot, Error> {
        let mut rates = HashMap::new();

        for pair in &self.config.monitored_pairs {
            if let Some((from, to)) = pair.split_once('/') {
                match self.client.get_forex_quote(from, to).await {
                    Ok(quote) => {
                        rates.insert(pair.clone(), quote.price);
                    }
                    Err(e) => {
                        eprintln!("⚠️ Failed to get rate for {}: {}", pair, e);
                    }
                }

                // Small delay to respect API rate limits
                sleep(Duration::from_millis(100)).await;
            }
        }

        Ok(RateSnapshot {
            timestamp: Utc::now(),
            rates,
            base_currency: "USD".to_string(), // Default base
        })
    }

    fn detect_significant_changes(
        &self,
        old: &RateSnapshot,
        new: &RateSnapshot,
    ) -> Vec<RateChange> {
        let mut changes = Vec::new();

        for (pair, new_rate) in &new.rates {
            if let Some(old_rate) = old.rates.get(pair) {
                let change_absolute = new_rate - old_rate;
                let change_percent = (change_absolute / old_rate) * 100.0;

                if change_percent.abs() >= self.config.significant_change_threshold {
                    changes.push(RateChange {
                        currency_pair: pair.clone(),
                        old_rate: *old_rate,
                        new_rate: *new_rate,
                        change_percent,
                        change_absolute,
                        timestamp: new.timestamp,
                    });
                }
            }
        }

        changes
    }

    async fn analyze_market_changes(&self, changes: &[RateChange]) -> Result<String, Error> {
        let changes_summary = changes
            .iter()
            .map(|c| {
                format!(
                    "{}: {:.4} -> {:.4} ({:+.2}%)",
                    c.currency_pair, c.old_rate, c.new_rate, c.change_percent
                )
            })
            .collect::<Vec<_>>()
            .join(", ");

        // For now, provide basic technical analysis without LLM
        // TODO: Integrate with AutoAgents framework for AI analysis
        let mut analysis = String::new();

        let significant_changes = changes
            .iter()
            .filter(|c| c.change_percent.abs() > 1.0)
            .count();
        let bullish_changes = changes.iter().filter(|c| c.change_percent > 0.0).count();
        let bearish_changes = changes.iter().filter(|c| c.change_percent < 0.0).count();

        analysis.push_str(&format!(
            "Market Analysis: {} significant changes detected. ",
            significant_changes
        ));

        if bullish_changes > bearish_changes {
            analysis.push_str("Overall bullish sentiment with more currencies strengthening. ");
        } else if bearish_changes > bullish_changes {
            analysis.push_str("Overall bearish sentiment with more currencies weakening. ");
        } else {
            analysis.push_str("Mixed market sentiment with balanced movements. ");
        }

        analysis.push_str(&format!("Changes: {}", changes_summary));

        Ok(analysis)
    }

    async fn generate_trading_recommendations(
        &self,
        changes: &[RateChange],
        analysis: &str,
    ) -> Result<Vec<TradingRecommendation>, Error> {
        let mut recommendations = Vec::new();

        let portfolio_value = self.calculate_total_portfolio_value().await?;
        let max_trade_amount = portfolio_value * (self.config.max_risk_per_trade / 100.0);

        for change in changes {
            // Simple strategy: buy on significant drops, sell on significant rises
            let (action, confidence, risk_level) = if change.change_percent <= -1.0 {
                ("BUY", 0.7, "MEDIUM")
            } else if change.change_percent >= 1.0 {
                ("SELL", 0.7, "MEDIUM")
            } else {
                ("HOLD", 0.5, "LOW")
            };

            if action != "HOLD" {
                let (from_currency, to_currency) = if action == "BUY" {
                    // Buy the currency that dropped (it might recover)
                    let parts: Vec<&str> = change.currency_pair.split('/').collect();
                    (parts[1].to_string(), parts[0].to_string())
                } else {
                    // Sell the currency that rose (take profit)
                    let parts: Vec<&str> = change.currency_pair.split('/').collect();
                    (parts[0].to_string(), parts[1].to_string())
                };

                // Check if we have the source currency
                if let Some(available_amount) = self.portfolio.holdings.get(&from_currency) {
                    let trade_amount = available_amount.min(max_trade_amount);

                    if trade_amount > 0.01 {
                        // Minimum trade amount
                        let expected_profit = trade_amount * (change.change_percent.abs() / 100.0);

                        recommendations.push(TradingRecommendation {
                            action: action.to_string(),
                            from_currency,
                            to_currency,
                            amount: trade_amount,
                            expected_profit,
                            confidence,
                            reasoning: format!(
                                "Rate change of {:+.2}% detected. AI Analysis: {}",
                                change.change_percent,
                                analysis.chars().take(100).collect::<String>()
                            ),
                            risk_level: risk_level.to_string(),
                            timestamp: Utc::now(),
                        });
                    }
                }
            }
        }

        Ok(recommendations)
    }

    async fn execute_recommendation(
        &mut self,
        recommendation: &TradingRecommendation,
    ) -> Result<(), Error> {
        println!("\n💡 Trading Recommendation:");
        println!(
            "   Action: {} {} -> {}",
            recommendation.action, recommendation.from_currency, recommendation.to_currency
        );
        println!(
            "   Amount: {:.2} {}",
            recommendation.amount, recommendation.from_currency
        );
        println!("   Expected Profit: {:.2}", recommendation.expected_profit);
        println!("   Confidence: {:.0}%", recommendation.confidence * 100.0);
        println!("   Risk: {}", recommendation.risk_level);
        println!("   Reasoning: {}", recommendation.reasoning);

        // Get current rate for the transaction
        match self
            .client
            .get_forex_quote(&recommendation.from_currency, &recommendation.to_currency)
            .await
        {
            Ok(quote) => {
                let amount_to = recommendation.amount * quote.price;

                // Update portfolio holdings
                if let Some(from_balance) = self
                    .portfolio
                    .holdings
                    .get_mut(&recommendation.from_currency)
                {
                    if *from_balance >= recommendation.amount {
                        *from_balance -= recommendation.amount;

                        // Add to target currency
                        let to_balance = self
                            .portfolio
                            .holdings
                            .entry(recommendation.to_currency.clone())
                            .or_insert(0.0);
                        *to_balance += amount_to;

                        self.portfolio.total_transactions += 1;
                        self.portfolio.last_updated = Utc::now();

                        println!("✅ Transaction executed:");
                        println!(
                            "   Converted {:.2} {} to {:.2} {} at rate {:.6}",
                            recommendation.amount,
                            recommendation.from_currency,
                            amount_to,
                            recommendation.to_currency,
                            quote.price
                        );

                        self.print_portfolio_summary().await?;
                    } else {
                        println!("❌ Insufficient balance for transaction");
                    }
                } else {
                    println!(
                        "❌ Currency {} not found in portfolio",
                        recommendation.from_currency
                    );
                }
            }
            Err(e) => {
                println!("❌ Failed to execute trade: {}", e);
            }
        }

        Ok(())
    }

    async fn calculate_total_portfolio_value(&self) -> Result<f64, Error> {
        let mut total_value = 0.0;

        for (currency, amount) in &self.portfolio.holdings {
            if currency == &self.portfolio.initial_currency {
                total_value += amount;
            } else {
                // Convert to initial currency
                match self
                    .client
                    .get_forex_quote(currency, &self.portfolio.initial_currency)
                    .await
                {
                    Ok(quote) => {
                        total_value += amount * quote.price;
                    }
                    Err(_) => {
                        // If conversion fails, use the amount as-is (rough approximation)
                        total_value += amount;
                    }
                }
            }
        }

        Ok(total_value)
    }

    async fn update_portfolio_value(&mut self) -> Result<(), Error> {
        self.portfolio.last_updated = Utc::now();
        Ok(())
    }

    async fn print_portfolio_summary(&self) -> Result<(), Error> {
        println!("\n💼 Portfolio Summary:");
        println!(
            "   Initial Investment: {:.2} {}",
            self.portfolio.initial_investment, self.portfolio.initial_currency
        );

        let total_value = self.calculate_total_portfolio_value().await?;
        let profit_loss = total_value - self.portfolio.initial_investment;
        let profit_loss_percent = (profit_loss / self.portfolio.initial_investment) * 100.0;

        println!(
            "   Current Value: {:.2} {} ({:+.2} {}, {:+.2}%)",
            total_value,
            self.portfolio.initial_currency,
            profit_loss,
            self.portfolio.initial_currency,
            profit_loss_percent
        );

        println!("   Holdings:");
        for (currency, amount) in &self.portfolio.holdings {
            if *amount > 0.01 {
                println!("     {}: {:.2}", currency, amount);
            }
        }

        println!(
            "   Total Transactions: {}",
            self.portfolio.total_transactions
        );

        Ok(())
    }

    pub fn stop_monitoring(&mut self) {
        println!("🛑 Stopping currency monitor...");
        self.is_running = false;
    }
}
