use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// Health status of a service
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum HealthStatus {
    Healthy,
    Unhealthy,
    Timeout,
    Unknown,
}

/// Overall infrastructure status
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OverallStatus {
    AllHealthy,
    MostlyHealthy,
    PartiallyHealthy,
    Unhealthy,
}

/// Health information for a single service
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServiceHealth {
    pub service_name: String,
    pub status: HealthStatus,
    pub response_time_ms: u64,
    pub last_checked: DateTime<Utc>,
    pub error_message: Option<String>,
    pub additional_info: Option<serde_json::Value>,
}

/// Complete infrastructure status
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InfrastructureStatus {
    pub overall_status: OverallStatus,
    pub total_services: usize,
    pub healthy_services: usize,
    pub services: Vec<ServiceHealth>,
    pub last_updated: DateTime<Utc>,
}

/// MEV protection service information
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MevProtectionService {
    pub name: String,
    pub endpoint: String,
    pub region: String,
    pub service_type: MevServiceType,
    pub health: ServiceHealth,
}

/// Types of MEV protection services
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MevServiceType {
    ZeroSlot,
    Nozomi,
    Jito,
    External,
    Astralane,
}

/// RPC endpoint information
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RpcEndpoint {
    pub name: String,
    pub url: String,
    pub chain: String,
    pub health: ServiceHealth,
    pub block_height: Option<u64>,
    pub latency_ms: Option<u64>,
}

/// Network latency information
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NetworkLatency {
    pub service_name: String,
    pub region: String,
    pub latency_ms: u64,
    pub packet_loss_percent: f64,
    pub timestamp: DateTime<Utc>,
}

/// Service uptime statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UptimeStats {
    pub service_name: String,
    pub uptime_percentage_24h: f64,
    pub uptime_percentage_7d: f64,
    pub uptime_percentage_30d: f64,
    pub total_downtime_minutes_24h: u64,
    pub incident_count_24h: u32,
    pub last_incident: Option<DateTime<Utc>>,
}

/// Performance metrics for services
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PerformanceMetrics {
    pub service_name: String,
    pub avg_response_time_ms: f64,
    pub p95_response_time_ms: u64,
    pub p99_response_time_ms: u64,
    pub requests_per_second: f64,
    pub error_rate_percent: f64,
    pub time_window_hours: u32,
}

/// Alert threshold configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AlertThreshold {
    pub service_name: String,
    pub metric_type: MetricType,
    pub threshold_value: f64,
    pub comparison: ComparisonOperator,
    pub duration_minutes: u32,
    pub severity: AlertSeverity,
}

/// Types of metrics to monitor
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MetricType {
    ResponseTime,
    ErrorRate,
    UptimePercentage,
    RequestsPerSecond,
    PacketLoss,
}

/// Comparison operators for thresholds
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ComparisonOperator {
    GreaterThan,
    LessThan,
    EqualTo,
    GreaterThanOrEqual,
    LessThanOrEqual,
}

/// Alert severity levels
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AlertSeverity {
    Info,
    Warning,
    Critical,
    Emergency,
}

/// System incident information
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Incident {
    pub id: String,
    pub title: String,
    pub description: String,
    pub affected_services: Vec<String>,
    pub severity: AlertSeverity,
    pub status: IncidentStatus,
    pub started_at: DateTime<Utc>,
    pub resolved_at: Option<DateTime<Utc>>,
    pub impact: ImpactLevel,
    pub updates: Vec<IncidentUpdate>,
}

/// Incident status
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IncidentStatus {
    Investigating,
    Identified,
    Monitoring,
    Resolved,
}

/// Impact level of incidents
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ImpactLevel {
    None,
    Minor,
    Major,
    Critical,
}

/// Incident update information
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IncidentUpdate {
    pub id: String,
    pub message: String,
    pub status: IncidentStatus,
    pub timestamp: DateTime<Utc>,
    pub author: String,
}

/// Geographic region information for services
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServiceRegion {
    pub name: String,
    pub code: String,
    pub continent: String,
    pub country: String,
    pub city: String,
    pub latitude: f64,
    pub longitude: f64,
}

/// Load balancing information
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LoadBalancingInfo {
    pub primary_endpoint: String,
    pub backup_endpoints: Vec<String>,
    pub current_active: String,
    pub failover_threshold_ms: u64,
    pub load_distribution: LoadDistribution,
}

/// Load distribution strategy
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LoadDistribution {
    RoundRobin,
    WeightedRoundRobin,
    LeastConnections,
    LatencyBased,
    Geographic,
}