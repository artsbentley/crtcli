use reqwest::{Error,Client};
use serde_json::json;

use crate::args::{Environment, InjectDSH};


pub fn process_tls(input_str: &str) -> String {
    input_str.replace("\n", "\\n")
}

pub struct DshApi {
    environment: Environment,
    tenant: String,
    api_key: String,
    bearer_token: String,
    temp_access_token_endpoint: String,
    api_client_id: String,
}

impl DshApi {
    pub fn new(
        environment: Environment,
        tenant: &str,
        //NOTE: is this enum really needed?
        api_key: InjectDSH,
    ) -> Self {
        DshApi {

            environment,
            tenant: tenant.to_string(),
            api_key: if let InjectDSH::True(api_key) = api_key {
                api_key
            },
            bearer_token: self::retrieve_token().unwrap,






            temp_access_token_endpoint: temp_access_token_endpoint.to_string(),
            api_client_id: api_client_id.to_string(),
            tenant: tenant.to_string(),
            api_client_secret: api_client_secret.to_string(),
        }
    }

    pub async fn retrieve_token(&self) -> Result<(), Error> {
        // Define the request parameters
        let params = [
            ("grant_type", "client_credentials"),
            (
                "client_id",
                &format!("robot:{}:{}", &self.api_client_id, &self.tenant),
            ),
            ("client_secret", &self.api_client_secret),
        ];

        // Send a POST request to the token endpoint
        let client = reqwest::Client::new();
        let response = client
            .post(&self.temp_access_token_endpoint)
            .header("Content-Type", "application/x-www-form-urlencoded")
            .form(&params)
            .send()
            .await?; // Use .await here to await the response asynchronously

        if response.status().is_success() {
            // Parse the JSON response to get the access token
            let json_response: serde_json::Value = response.json().await?;
            if let Some(access_token) = json_response.get("access_token") {
                if let Some(token) = access_token.as_str() {
                    println!("Bearer token: {}", token);
                }
            } else {
                println!("Invalid response from the server.");
            }
        } else {
            println!(
                "Error: Request failed with status code {:?}",
                response.status()
            );
        }

        Ok(())
    }

    pub async fn send_secret(&self, name: &str, value: &str) -> Result<(), Error> {
            // Define the URL for sending the secret
            let secret_url = "https://api.{}/resources/v0/allocation/{}/secret";

            // Create a JSON object for the secret data
            let secret_data = json!({
                "name": name,
                "value": value,
            });

            // Send a POST request to send the secret
            let client = Client::new();
            let response = client
                .post(secret_url)
                .header("accept", "*/*")
                .header("Authorization", "ENTERBEARERHERER")
                .header("Content-Type", "application/json")
                .json(&secret_data)
                .send()
                .await?;

            if response.status().is_success() {
                // You can handle the successful response here if needed
                println!("Secret sent successfully!");
            } else {
                println!(
                    "Error: Request failed with status code {:?}",
                    response.status()
                );
            }

            Ok(())
        }
}
