use crate::metrics::{Metric, Event, ServiceCheck};
use crate::custom_metrics::CustomMetrics;
use openssl::rsa::{Rsa, Padding};
use reqwest::Client;
use serde::Serialize;
use std::error::Error;

pub async fn bootstrap(api_url: &str, api_key: &str, public_key: &Rsa<openssl::pkey::Public>) -> Result<(), Box<dyn Error>> {
    let client = Client::new();
    let pub_key_pem = public_key.public_key_to_pem()?;
    let response = client.post(format!("{}/bootstrap", api_url))
        .header("X-API-KEY", api_key)
        .timeout(std::time::Duration::from_secs(5))
        .body(pub_key_pem)
        .send()
        .await?;

    if response.status().is_success() {
        Ok(())
    } else {
        Err(Box::from(response.text().await?))
    }
}

pub async fn send_metrics(api_url: &str, api_key: &str, metrics: &[Metric], public_key: &Rsa<openssl::pkey::Public>, private_key: &Rsa<openssl::pkey::Private>) -> Result<(), Box<dyn Error>> {
    let client = Client::new();
    let metrics_json = serde_json::to_string(metrics)?;
    let signed_metrics = sign_data(&metrics_json, private_key)?;

    let response = client.post(api_url)
        .header("X-API-KEY", api_key)
        .header("X-SIGNED-METRICS", signed_metrics)
        .body(metrics_json)
        .send()
        .await?;

    if response.status().is_success() {
        Ok(())
    } else {
        Err(Box::from(response.text().await?))
    }
}

pub async fn send_custom_metrics(api_url: &str, api_key: &str, custom_metrics: &CustomMetrics, public_key: &Rsa<openssl::pkey::Public>, private_key: &Rsa<openssl::pkey::Private>) -> Result<(), Box<dyn Error>> {
    let client = Client::new();
    let custom_metrics_json = serde_json::to_string(custom_metrics)?;
    let signed_metrics = sign_data(&custom_metrics_json, private_key)?;

    let response = client.post(&format!("{}/custom", api_url))
        .header("X-API-KEY", api_key)
        .header("X-SIGNED-METRICS", signed_metrics)
        .body(custom_metrics_json)
        .send()
        .await?;

    if response.status().is_success() {
        Ok(())
    } else {
        Err(Box::from(response.text().await?))
    }
}

pub async fn send_event(api_url: &str, api_key: &str, event: &Event, public_key: &Rsa<openssl::pkey::Public>, private_key: &Rsa<openssl::pkey::Private>) -> Result<(), Box<dyn Error>> {
    let client = Client::new();
    let event_json = serde_json::to_string(event)?;
    let signed_event = sign_data(&event_json, private_key)?;

    let response = client.post(&format!("{}/event", api_url))
        .header("X-API-KEY", api_key)
        .header("X-SIGNED-EVENT", signed_event)
        .body(event_json)
        .send()
        .await?;

    if response.status().is_success() {
        Ok(())
    } else {
        Err(Box::from(response.text().await?))
    }
}

pub async fn send_service_check(api_url: &str, api_key: &str, service_check: &ServiceCheck, public_key: &Rsa<openssl::pkey::Public>, private_key: &Rsa<openssl::pkey::Private>) -> Result<(), Box<dyn Error>> {
    let client = Client::new();
    let service_check_json = serde_json::to_string(service_check)?;
    let signed_service_check = sign_data(&service_check_json, private_key)?;

    let response = client.post(&format!("{}/service_check", api_url))
        .header("X-API-KEY", api_key)
        .header("X-SIGNED-SERVICE-CHECK", signed_service_check)
        .body(service_check_json)
        .send()
        .await?;

    if response.status().is_success() {
        Ok(())
    } else {
        Err(Box::from(response.text().await?))
    }
}

fn sign_data(data: &str, private_key: &Rsa<openssl::pkey::Private>) -> Result<String, Box<dyn Error>> {
    let keypair = openssl::pkey::PKey::from_rsa(private_key.clone()).unwrap();
    let mut signer = openssl::sign::Signer::new(openssl::hash::MessageDigest::sha256(), &keypair).unwrap();
    signer.update(data.as_bytes()).unwrap();
    let signature = signer.sign_to_vec().unwrap();
    Ok(base64::encode(signature))
}
