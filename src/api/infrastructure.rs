use crate::errors::Result;
use crate::models::infrastructure::*;
use std::time::Duration;
use tokio::time::timeout;

/// Infrastructure monitoring and health check client
/// Monitors MEV protection services and RPC endpoints
pub struct InfrastructureClient {
    client: reqwest::Client,
    timeout_duration: Duration,
}

impl InfrastructureClient {
    /// Create a new infrastructure monitoring client
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
            timeout_duration: Duration::from_secs(5),
        }
    }

    /// Check Axiom Trade lighthouse service health
    /// 
    /// # Returns
    /// 
    /// ServiceHealth - Health status of the lighthouse service
    pub async fn check_lighthouse_health(&self) -> Result<ServiceHealth> {
        let url = "https://api8.axiom.trade/lighthouse";
        self.check_service_health("Lighthouse", url).await
    }

    /// Check all 0slot MEV protection services
    /// 
    /// # Returns
    /// 
    /// Vec<ServiceHealth> - Health status of all 0slot services
    pub async fn check_0slot_health(&self) -> Result<Vec<ServiceHealth>> {
        let endpoints = vec![
            ("0slot-LA", "https://la1.0slot.trade/health"),
            ("0slot-NY", "https://ny3.0slot.trade/health"),
            ("0slot-DE", "https://de1.0slot.trade/health"),
            ("0slot-AMS", "https://ams1.0slot.trade/health"),
            ("0slot-JP", "https://jp1.0slot.trade/health"),
        ];

        let mut results = Vec::new();
        for (name, url) in endpoints {
            let health = self.check_service_health(name, url).await?;
            results.push(health);
        }
        Ok(results)
    }

    /// Check all Nozomi temporal network nodes
    /// 
    /// # Returns
    /// 
    /// Vec<ServiceHealth> - Health status of all Nozomi nodes
    pub async fn check_nozomi_health(&self) -> Result<Vec<ServiceHealth>> {
        let endpoints = vec![
            ("Nozomi-LAX", "https://lax1.secure.nozomi.temporal.xyz/ping"),
            ("Nozomi-EWR", "https://ewr1.secure.nozomi.temporal.xyz/ping"),
            ("Nozomi-AMS", "https://ams1.secure.nozomi.temporal.xyz/ping"),
            ("Nozomi-FRA", "https://fra2.secure.nozomi.temporal.xyz/ping"),
            ("Nozomi-ASH", "https://ash1.secure.nozomi.temporal.xyz/ping"),
            ("Nozomi-SGP", "https://sgp1.secure.nozomi.temporal.xyz/ping"),
            ("Nozomi-TYO", "https://tyo1.secure.nozomi.temporal.xyz/ping"),
            ("Nozomi-PIT", "https://pit1.secure.nozomi.temporal.xyz/ping"),
            ("Nozomi-Main", "https://nozomi.temporal.xyz/ping"),
        ];

        let mut results = Vec::new();
        for (name, url) in endpoints {
            let health = self.check_service_health(name, url).await?;
            results.push(health);
        }
        Ok(results)
    }

    /// Check external MEV protection service
    /// 
    /// # Returns
    /// 
    /// ServiceHealth - Health status of the external MEV service
    pub async fn check_external_mev_health(&self) -> Result<ServiceHealth> {
        let url = "https://tx.axiomext.net/ping";
        self.check_service_health("External-MEV", url).await
    }

    /// Check all Jito block engine endpoints
    /// 
    /// # Returns
    /// 
    /// Vec<ServiceHealth> - Health status of all Jito endpoints
    pub async fn check_jito_health(&self) -> Result<Vec<ServiceHealth>> {
        let endpoints = vec![
            ("Jito-SLC", "https://slc.mainnet.block-engine.jito.wtf/api/v1/getTipAccounts"),
            ("Jito-London", "https://london.mainnet.block-engine.jito.wtf/api/v1/getTipAccounts"),
            ("Jito-Frankfurt", "https://frankfurt.mainnet.block-engine.jito.wtf/api/v1/getTipAccounts"),
            ("Jito-NY", "https://ny.mainnet.block-engine.jito.wtf/api/v1/getTipAccounts"),
            ("Jito-Tokyo", "https://tokyo.mainnet.block-engine.jito.wtf/api/v1/getTipAccounts"),
        ];

        let mut results = Vec::new();
        for (name, url) in endpoints {
            let health = self.check_jito_endpoint(name, url).await?;
            results.push(health);
        }
        Ok(results)
    }

    /// Check Astralane gateway health
    /// 
    /// # Returns
    /// 
    /// Vec<ServiceHealth> - Health status of Astralane gateways
    pub async fn check_astralane_health(&self) -> Result<Vec<ServiceHealth>> {
        let api_key = "AxiomozyNSTbBlP88VY35BvSdDVS3du1be8Q1VMmconPgpWFVWnpmfnpUrhRj97F";
        let endpoints = vec![
            ("Astralane-FRA", format!("https://axiom-fra.gateway.astralane.io/gethealth?api-key={}", api_key)),
            ("Astralane-CA", format!("https://axiom-ca.gateway.astralane.io/gethealth?api-key={}", api_key)),
        ];

        let mut results = Vec::new();
        for (name, url) in endpoints {
            let health = self.check_service_health(name, &url).await?;
            results.push(health);
        }
        Ok(results)
    }

    /// Check Arbitrum RPC health
    /// 
    /// # Returns
    /// 
    /// ServiceHealth - Health status of Arbitrum RPC
    pub async fn check_arbitrum_rpc_health(&self) -> Result<ServiceHealth> {
        let url = "https://arb1.arbitrum.io/rpc";
        let payload = serde_json::json!({
            "jsonrpc": "2.0",
            "method": "eth_blockNumber",
            "params": [],
            "id": 1
        });

        let start_time = std::time::Instant::now();
        
        let response_result = timeout(
            self.timeout_duration,
            self.client.post(url).json(&payload).send()
        ).await;

        let response_time = start_time.elapsed();

        match response_result {
            Ok(Ok(response)) => {
                let status = if response.status().is_success() {
                    HealthStatus::Healthy
                } else {
                    HealthStatus::Unhealthy
                };

                Ok(ServiceHealth {
                    service_name: "Arbitrum-RPC".to_string(),
                    status,
                    response_time_ms: response_time.as_millis() as u64,
                    last_checked: chrono::Utc::now(),
                    error_message: None,
                    additional_info: Some(serde_json::json!({
                        "endpoint": url,
                        "method": "eth_blockNumber"
                    })),
                })
            }
            Ok(Err(e)) => Ok(ServiceHealth {
                service_name: "Arbitrum-RPC".to_string(),
                status: HealthStatus::Unhealthy,
                response_time_ms: response_time.as_millis() as u64,
                last_checked: chrono::Utc::now(),
                error_message: Some(e.to_string()),
                additional_info: None,
            }),
            Err(_) => Ok(ServiceHealth {
                service_name: "Arbitrum-RPC".to_string(),
                status: HealthStatus::Timeout,
                response_time_ms: self.timeout_duration.as_millis() as u64,
                last_checked: chrono::Utc::now(),
                error_message: Some("Request timeout".to_string()),
                additional_info: None,
            }),
        }
    }

    /// Get comprehensive infrastructure status
    /// 
    /// # Returns
    /// 
    /// InfrastructureStatus - Complete status of all monitored services
    pub async fn get_comprehensive_status(&self) -> Result<InfrastructureStatus> {
        let lighthouse = self.check_lighthouse_health().await?;
        let zero_slot = self.check_0slot_health().await?;
        let nozomi = self.check_nozomi_health().await?;
        let external_mev = self.check_external_mev_health().await?;
        let jito = self.check_jito_health().await?;
        let astralane = self.check_astralane_health().await?;
        let arbitrum = self.check_arbitrum_rpc_health().await?;

        let all_services = vec![vec![lighthouse, external_mev, arbitrum], zero_slot, nozomi, jito, astralane].concat();
        
        let healthy_count = all_services.iter().filter(|s| s.status == HealthStatus::Healthy).count();
        let total_count = all_services.len();
        
        let overall_status = if healthy_count == total_count {
            OverallStatus::AllHealthy
        } else if healthy_count > total_count / 2 {
            OverallStatus::MostlyHealthy
        } else if healthy_count > 0 {
            OverallStatus::PartiallyHealthy
        } else {
            OverallStatus::Unhealthy
        };

        Ok(InfrastructureStatus {
            overall_status,
            total_services: total_count,
            healthy_services: healthy_count,
            services: all_services,
            last_updated: chrono::Utc::now(),
        })
    }

    /// Internal helper to check service health
    async fn check_service_health(&self, name: &str, url: &str) -> Result<ServiceHealth> {
        let start_time = std::time::Instant::now();
        
        let response_result = timeout(
            self.timeout_duration,
            self.client.get(url).send()
        ).await;

        let response_time = start_time.elapsed();

        match response_result {
            Ok(Ok(response)) => {
                let status = if response.status().is_success() {
                    HealthStatus::Healthy
                } else {
                    HealthStatus::Unhealthy
                };

                Ok(ServiceHealth {
                    service_name: name.to_string(),
                    status,
                    response_time_ms: response_time.as_millis() as u64,
                    last_checked: chrono::Utc::now(),
                    error_message: None,
                    additional_info: Some(serde_json::json!({
                        "endpoint": url,
                        "status_code": response.status().as_u16()
                    })),
                })
            }
            Ok(Err(e)) => Ok(ServiceHealth {
                service_name: name.to_string(),
                status: HealthStatus::Unhealthy,
                response_time_ms: response_time.as_millis() as u64,
                last_checked: chrono::Utc::now(),
                error_message: Some(e.to_string()),
                additional_info: None,
            }),
            Err(_) => Ok(ServiceHealth {
                service_name: name.to_string(),
                status: HealthStatus::Timeout,
                response_time_ms: self.timeout_duration.as_millis() as u64,
                last_checked: chrono::Utc::now(),
                error_message: Some("Request timeout".to_string()),
                additional_info: None,
            }),
        }
    }

    /// Internal helper to check Jito endpoints (POST requests)
    async fn check_jito_endpoint(&self, name: &str, url: &str) -> Result<ServiceHealth> {
        let start_time = std::time::Instant::now();
        
        let response_result = timeout(
            self.timeout_duration,
            self.client.post(url).json(&serde_json::json!({})).send()
        ).await;

        let response_time = start_time.elapsed();

        match response_result {
            Ok(Ok(response)) => {
                let status = if response.status().is_success() {
                    HealthStatus::Healthy
                } else {
                    HealthStatus::Unhealthy
                };

                Ok(ServiceHealth {
                    service_name: name.to_string(),
                    status,
                    response_time_ms: response_time.as_millis() as u64,
                    last_checked: chrono::Utc::now(),
                    error_message: None,
                    additional_info: Some(serde_json::json!({
                        "endpoint": url,
                        "method": "getTipAccounts"
                    })),
                })
            }
            Ok(Err(e)) => Ok(ServiceHealth {
                service_name: name.to_string(),
                status: HealthStatus::Unhealthy,
                response_time_ms: response_time.as_millis() as u64,
                last_checked: chrono::Utc::now(),
                error_message: Some(e.to_string()),
                additional_info: None,
            }),
            Err(_) => Ok(ServiceHealth {
                service_name: name.to_string(),
                status: HealthStatus::Timeout,
                response_time_ms: self.timeout_duration.as_millis() as u64,
                last_checked: chrono::Utc::now(),
                error_message: Some("Request timeout".to_string()),
                additional_info: None,
            }),
        }
    }
}