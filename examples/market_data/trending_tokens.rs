/// Trending Tokens Analysis Example
/// 
/// This example demonstrates how to fetch and analyze trending tokens
/// across different timeframes and identify trading opportunities.

use axiomtrade_rs::api::market_data::MarketDataClient;
use axiomtrade_rs::auth::AuthClient;
use axiomtrade_rs::models::market::TimePeriod;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load credentials and authenticate
    dotenvy::dotenv().ok();
    let mut market_client = authenticate().await?;

    println!("Trending Tokens Analysis");
    println!("Analyzing market trends across multiple timeframes\n");

    // Get trending tokens for different timeframes
    let timeframes = vec![
        ("1H", TimePeriod::OneHour),
        ("24H", TimePeriod::TwentyFourHours),
        ("7D", TimePeriod::SevenDays),
        ("30D", TimePeriod::ThirtyDays)
    ];
    
    for (timeframe_str, timeframe) in timeframes {
        println!("=== {} TRENDING TOKENS ===", timeframe_str);
        
        match market_client.get_trending_tokens(timeframe).await {
            Ok(trending) => {
                println!("Found {} trending tokens", trending.len());
                
                // Show top 10 with detailed analysis
                for (i, token) in trending.iter().take(10).enumerate() {
                    println!("{}. {} ({})", 
                        i + 1, 
                        token.symbol,
                        token.name
                    );
                    
                    println!("   Price: ${:.8}", token.price_usd);
                    println!("   Change 24h: {:.2}%", token.price_change_24h);
                    println!("   Volume: ${:.2}", token.volume_24h);
                    println!("   Market Cap: ${:.2}", token.market_cap);
                    println!("   Holders: {}", token.holders);
                    
                    // Risk assessment
                    assess_token_risk(token);
                    println!();
                }
                
                // Market analysis
                analyze_market_trends(&trending, timeframe_str);
            }
            Err(e) => {
                println!("Failed to fetch {} trending tokens: {}", timeframe_str, e);
            }
        }
        
        println!();
    }

    // Cross-timeframe analysis
    println!("=== CROSS-TIMEFRAME ANALYSIS ===");
    
    // Get data for multiple timeframes
    let short_term = market_client.get_trending_tokens(TimePeriod::OneHour).await
        .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;
    let long_term = market_client.get_trending_tokens(TimePeriod::TwentyFourHours).await
        .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;
    
    // Find tokens trending in both timeframes
    let consistent_trends = find_consistent_trends(&short_term, &long_term);
    
    if !consistent_trends.is_empty() {
        println!("Tokens trending consistently across timeframes:");
        for token in consistent_trends {
            println!("  {} - Strong momentum detected", 
                token.symbol);
        }
    } else {
        println!("No tokens found trending consistently across timeframes");
    }

    // Market opportunity analysis
    println!("\n=== OPPORTUNITY ANALYSIS ===");
    analyze_opportunities(&short_term).await;

    println!("Trending tokens analysis completed");
    Ok(())
}

fn assess_token_risk(token: &axiomtrade_rs::models::market::TrendingToken) {
    let mut risk_factors = Vec::new();
    
    // Volume/Market Cap ratio
    let vol_to_mcap = token.volume_24h / token.market_cap;
    if vol_to_mcap > 1.0 {
        risk_factors.push("High volume/mcap ratio");
    }
    
    // Extreme price movements
    if token.price_change_24h.abs() > 100.0 {
        risk_factors.push("Extreme price volatility");
    }
    
    // Very small market cap
    if token.market_cap < 100000.0 {
        risk_factors.push("Very small market cap");
    }
    
    // Low holder count
    if token.holders < 50 {
        risk_factors.push("Low holder count");
    }
    
    if !risk_factors.is_empty() {
        println!("   ⚠️  Risk factors: {}", risk_factors.join(", "));
    } else {
        println!("   ✅ Low risk profile");
    }
}

fn analyze_market_trends(tokens: &[axiomtrade_rs::models::market::TrendingToken], timeframe: &str) {
    let total_tokens = tokens.len();
    let gainers = tokens.iter().filter(|t| t.price_change_24h > 0.0).count();
    let losers = total_tokens - gainers;
    
    let avg_change: f64 = tokens.iter()
        .map(|t| t.price_change_24h)
        .sum::<f64>() / total_tokens as f64;
    
    let total_volume: f64 = tokens.iter()
        .map(|t| t.volume_24h)
        .sum();
    
    println!("Market Summary for {}:", timeframe);
    println!("  Gainers: {} ({:.1}%)", gainers, (gainers as f64 / total_tokens as f64) * 100.0);
    println!("  Losers: {} ({:.1}%)", losers, (losers as f64 / total_tokens as f64) * 100.0);
    println!("  Average change: {:.2}%", avg_change);
    println!("  Total volume: ${:.2}", total_volume);
    
    // Market sentiment
    if avg_change > 5.0 {
        println!("  📈 Market sentiment: Very Bullish");
    } else if avg_change > 0.0 {
        println!("  📊 Market sentiment: Bullish");
    } else if avg_change > -5.0 {
        println!("  📉 Market sentiment: Bearish");
    } else {
        println!("  📛 Market sentiment: Very Bearish");
    }
}

fn find_consistent_trends<'a>(
    short_term: &'a [axiomtrade_rs::models::market::TrendingToken],
    long_term: &'a [axiomtrade_rs::models::market::TrendingToken]
) -> Vec<&'a axiomtrade_rs::models::market::TrendingToken> {
    let mut consistent = Vec::new();
    
    for st_token in short_term.iter().take(20) {
        if let Some(lt_token) = long_term.iter()
            .find(|lt| lt.symbol == st_token.symbol) {
            
            // Check if trending in same direction
            let st_positive = st_token.price_change_24h > 0.0;
            let lt_positive = lt_token.price_change_24h > 0.0;
            
            if st_positive == lt_positive && 
               st_token.price_change_24h.abs() > 10.0 &&
               lt_token.price_change_24h.abs() > 10.0 {
                consistent.push(st_token);
            }
        }
    }
    
    consistent
}

async fn analyze_opportunities(tokens: &[axiomtrade_rs::models::market::TrendingToken]) {
    // Find tokens with good risk/reward profiles
    let opportunities: Vec<_> = tokens.iter()
        .filter(|token| {
            // Reasonable market cap
            let reasonable_mcap = token.market_cap > 1000000.0 && token.market_cap < 100000000.0;
            
            // Moderate positive momentum
            let good_momentum = token.price_change_24h > 5.0 && 
                               token.price_change_24h < 50.0;
            
            // High volume
            let high_volume = token.volume_24h > 100000.0;
            
            // Good holder count
            let good_holders = token.holders > 100;
            
            reasonable_mcap && good_momentum && high_volume && good_holders
        })
        .collect();
    
    if opportunities.is_empty() {
        println!("No clear opportunities identified in current market conditions");
        return;
    }
    
    println!("Potential opportunities identified:");
    for (i, token) in opportunities.iter().enumerate() {
        println!("{}. {} - {:.2}% gain, ${:.2} volume", 
            i + 1,
            token.symbol,
            token.price_change_24h,
            token.volume_24h
        );
        
        // Simple scoring
        let holders_score = (token.holders as f64 / 500.0).min(5.0);
        let momentum_score = (token.price_change_24h / 10.0).min(5.0);
        let volume_score = (token.volume_24h / 500000.0).min(5.0);
        
        let total_score = (holders_score + momentum_score + volume_score) / 3.0;
        
        println!("   Opportunity score: {:.1}/5.0", total_score);
        
        if total_score > 3.5 {
            println!("   🎯 High potential opportunity");
        } else if total_score > 2.5 {
            println!("   📊 Moderate opportunity");
        }
    }
    
    println!("\nRemember to:");
    println!("- Conduct thorough research before investing");
    println!("- Never invest more than you can afford to lose");
    println!("- Consider market conditions and your risk tolerance");
    println!("- Use proper position sizing and stop losses");
}

async fn authenticate() -> Result<MarketDataClient, Box<dyn std::error::Error>> {
    let email = env::var("AXIOM_EMAIL")
        .expect("AXIOM_EMAIL must be set in .env file");
    let password = env::var("AXIOM_PASSWORD")
        .expect("AXIOM_PASSWORD must be set in .env file");

    // First authenticate with AuthClient
    let mut auth_client = AuthClient::new()
        .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;
    auth_client.login(&email, &password, None).await
        .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;
    
    // Then create MarketDataClient
    let market_client = MarketDataClient::new()
        .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;
    
    Ok(market_client)
}