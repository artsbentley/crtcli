use reqwest::Client;
use reqwest::Error as ReqwestError;
use serde_json::json;
use std::io;

use crate::args::{Environment, InjectDSH, TenantConfig};

use thiserror::Error; // Add this line to use the `thiserror` crate for custom errors.
use tracing::{error, info};

#[derive(Debug, Error)]
pub enum ApiError {
    #[error("Token retrieval failed: {0}")]
    TokenRetrievalFailed(String),
    #[error("HTTP request failed: {0}")]
    HttpRequestFailed(#[from] reqwest::Error),
}

pub struct DshApi {
    pub environment: Environment,
    pub tenant: String,
    pub api_key: String,
    pub bearer_token: Option<String>,
    pub bearer_endpoint: String,
    pub api_client_id: String,
}

impl DshApi {
    // pub fn new( environment: Environment, tenant: String, api_key: String,) -> Self {
    pub fn new(tenant_config: &TenantConfig) -> Self {
        let environment = tenant_config.environment.clone();
        let tenant = tenant_config.name.clone();
        let bearer_endpoint = tenant_config.environment.bearer_endpoint();
        let api_client_id = tenant_config.environment.api_client_id();
        let api_key = match &tenant_config.inject_dsh {
            InjectDSH::True(api_key) => api_key.clone(),
            // TODO: this case must be handled with an erro instead of empty string
            InjectDSH::False => String::new(), // Or handle this case as per your requirements
        };

        DshApi {
            environment,
            tenant,
            api_key,
            bearer_token: None,
            bearer_endpoint,
            api_client_id,
        }
    }

    pub async fn initialize_bearer_token(&mut self) -> Result<(), ApiError> {
        let token = self.retrieve_token().await?.clone();
        self.bearer_token = Some(token);
        Ok(())
    }

    pub async fn retrieve_token(&self) -> Result<String, ApiError> {
        let params = [
            ("grant_type", "client_credentials"),
            (
                "client_id",
                &format!("robot:{}:{}", &self.api_client_id, &self.tenant),
            ),
            ("client_secret", &self.api_key),
        ];

        let client = reqwest::Client::new();
        let response = client
            .post(&self.bearer_endpoint)
            .header("Content-Type", "application/x-www-form-urlencoded")
            .form(&params)
            .send()
            .await?;

        if !response.status().is_success() {
            // return Err(io::Error::new(io::ErrorKind::Other, "token retrieval failed"));
            eprintln!("token retrieval failed");
        }

        let json_response: serde_json::Value = response.json().await?;
        let token = json_response
            .get("access_token")
            .and_then(|t| t.as_str())
            .ok_or(ApiError::TokenRetrievalFailed("failed".to_string()))?;

        Ok(token.to_string())
    }

    // TODO: make the function call the bearer token init func
    pub async fn send_secret(&self, name: &str, value: &str) -> Result<(), ApiError> {
        // TODO: retrieve the url from environment enum
        let secret_url = "https://api.poc.kpn-dsh.com/resources/v0/allocation/training/secret"; // Replace with your actual URL

        let secret_data = json!({
            "name": name,
            "value": value,
        });

        let client = Client::new();
        let response = client
            .post(secret_url)
            .header("accept", "*/*")
            .header(
                "Authorization",
                format!(
                    "Bearer {}",
                    self.bearer_token.as_ref().unwrap_or(&"".to_string())
                ),
            )
            .header("Content-Type", "application/json")
            .json(&secret_data)
            .send()
            .await?;

        if response.status().is_success() {
            info!("Secret sent successfully!");
        } else {
            error!(
                "Error: Request failed with status code {:?}",
                response.status()
            );
        }

        Ok(())
    }

    // TODO: create struct for the naming convention of DSH file names, so it can easily be passed
    // here as a struct instead of having to use all the individual parameters
    pub async fn create_dsh_cert(
        &self,
        brokerprefix: &str,
        certname: &str,
        keyname: &str,
    ) -> Result<(), ApiError> {
        let url = format!("https://api.poc.kpn-dsh.com/resources/v0/allocation/training/certificate/{brokerprefix}-kafka-proxy-certificate/configuration");

        let certificate_data = json!({
            "certChainSecret": certname,
            "keySecret": keyname,
        });

        let client = Client::new();
        let response = client
            .put(&url)
            .header("Content-Type", "application/json")
            .header(
                "Authorization",
                format!(
                    "Bearer {}",
                    self.bearer_token.as_ref().unwrap_or(&"".to_string())
                ),
            )
            .json(&certificate_data)
            .send()
            .await?;

        if response.status().is_success() {
            println!("Certificate configuration sent successfully!");
        } else {
            eprintln!(
                "Error: Request failed with status code {:?}",
                response.status()
            );
        }

        Ok(())
    }
}
