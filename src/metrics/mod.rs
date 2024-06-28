pub mod base;
pub mod events;
pub mod service_checks;

use serde::Serialize;
use base::agent::aliveness;
use base::cpu::load;
use tokio::task;

#[derive(Debug, Serialize)]
pub enum Metric {
    Count { name: String, value: u64 },
    Rate { name: String, value: f64 },
    Gauge { name: String, value: f64 },
    Set { name: String, values: Vec<String> },
    Histogram { name: String, values: Vec<f64> },
    Distribution { name: String, values: Vec<f64> },
}

#[derive(Debug, Serialize)]
pub enum Event {
    Info { name: String, message: String },
    Warning { name: String, message: String },
    Error { name: String, message: String },
}

#[derive(Debug, Serialize)]
pub enum ServiceCheck {
    Ok { name: String, message: String },
    Warning { name: String, message: String },
    Critical { name: String, message: String },
}

pub async fn collect_metrics() -> Vec<Metric> {
    let mut metrics = vec![];

    let aliveness_metric = task::spawn_blocking(|| aliveness::collect()).await.unwrap();
    metrics.push(aliveness_metric);

    let cpu_load_metric = task::spawn_blocking(|| load::collect()).await.unwrap();
    metrics.push(cpu_load_metric);

    metrics
}
