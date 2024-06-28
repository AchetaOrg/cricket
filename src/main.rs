mod config;
mod metrics;
mod network;
mod security;
mod custom_metrics;

use clap::Parser;
use config::CricketConfig;
use log::{info, error};
use std::sync::{Arc, Mutex};
use std::collections::VecDeque;

#[derive(Parser)]
#[command(name = "Cricket")]
#[command(about = "A lightweight server monitoring agent", long_about = None)]
struct Args {
    /// API URL
    #[arg(long)]
    api_url: Option<String>,

    /// API Key for initial bootstrap
    #[arg(long)]
    api_key: Option<String>,

    /// Interval in seconds between metric collections
    #[arg(long)]
    interval: Option<u64>,

    /// Submission interval in seconds for sending metrics
    #[arg(long)]
    submission_interval: Option<u64>,

    /// Path to the key pair
    #[arg(long)]
    key_path: Option<String>,
}

#[tokio::main]
async fn main() {
    env_logger::init();

    let args = Args::parse();
    let mut config = CricketConfig::new().expect("Failed to load config");

    // Override config values with command-line arguments if provided
    if let Some(api_url) = args.api_url {
        config.api_url = api_url;
    }
    if let Some(api_key) = args.api_key {
        config.api_key = api_key;
    }
    if let Some(interval) = args.interval {
        config.interval = interval;
    }
    if let Some(submission_interval) = args.submission_interval {
        config.submission_interval = submission_interval;
    }
    if let Some(key_path) = args.key_path {
        config.key_path = key_path;
    }

    let api_key = config.api_key.clone();

    info!("Cricket agent started");

    // Bootstrap and generate/load key pair
    let (public_key, private_key) = security::initialize_keys(&config).expect("Failed to initialize keys");
    if let Err(e) = network::bootstrap(&config.api_url, &api_key, &public_key).await {
        error!("Failed to bootstrap: {}", e);
        return;
    }

    let config = Arc::new(config);
    let metrics_queue = Arc::new(Mutex::new(VecDeque::new()));

    log::debug!("Starting metrics collection and submission tasks");
    // Metric collection task
    let collection = {
        let config = config.clone();
        let metrics_queue = metrics_queue.clone();

        tokio::spawn(async move {
            loop {
                log::debug!("Running metrics collection");
                let metrics = metrics::collect_metrics().await;
                {
                    let mut queue = metrics_queue.lock().unwrap();
                    queue.push_back(metrics);

                    // Retain only the last three collections (5s * 3 = 15s)
                    if queue.len() > 3 {
                        queue.pop_front();
                    }
                }

                tokio::time::sleep(std::time::Duration::from_secs(config.interval)).await;
            }
        })
    };

    // Metric submission task
    let submission = {
        let config = config.clone();
        let metrics_queue = metrics_queue.clone();

        tokio::spawn(async move {
            loop {
                log::debug!("Running metrics submission");
                let mut metrics = Vec::new();
                {
                    let mut queue = metrics_queue.lock().unwrap();
                    while let Some(batch) = queue.pop_front() {
                        metrics.extend(batch);
                    }
                }

                if !metrics.is_empty() {
                    if let Err(e) = network::send_metrics(&config.api_url, &config.api_key, &metrics, &public_key, &private_key).await {
                        error!("Failed to send metrics: {}", e);
                    }
                }

                tokio::time::sleep(std::time::Duration::from_secs(config.submission_interval)).await;
            }
        })
    };
    tokio::join!(collection, submission);
}
