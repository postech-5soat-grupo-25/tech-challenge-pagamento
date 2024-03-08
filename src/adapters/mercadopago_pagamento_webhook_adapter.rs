use crate::base::domain_error::DomainError;
use crate::entities::pagamento::Pagamento;
use crate::traits::pagamento_webhook_adapter::PagamentoWebhookAdapter;
use reqwest::Error as ReqwestError;
use std::env;

use serde_json::Value;

impl From<ReqwestError> for DomainError {
    fn from(error: ReqwestError) -> Self {
        DomainError::Invalid(format!("Reqwest error: {}", error))
    }
}

#[derive(Clone)]
pub struct MercadoPagoPagamentoWebhookAdapter {}

impl MercadoPagoPagamentoWebhookAdapter {
    pub fn new() -> Self {
        MercadoPagoPagamentoWebhookAdapter {}
    }
}

#[async_trait]
impl PagamentoWebhookAdapter for MercadoPagoPagamentoWebhookAdapter {
    fn processa_webhook(&self, data: Value, mut pagamento: Pagamento) -> Pagamento {
        // TODO update this to new key values
        if let Some(obj) = data.as_object() {
            if let Some(action) = obj.get("action") {
                if let Some(action_str) = action.as_str() {
                    if action_str == "payment.approved" {
                        pagamento.set_estado(String::from("aprovado"));
                    }
                }
            }
            if let Some(id) = obj.get("id") {
                // Check if the action attribute is a string and if it equals "payment.approved"
                if let Some(id_str) = id.as_str() {
                    pagamento.set_referencia(id_str.to_string());
                }
            }
        }
        pagamento
    }

    async fn set_webhook_pagamento(
        &self,
        mut pagamento: Pagamento,
    ) -> Result<Pagamento, DomainError> {
        let post_url = match env::var("MOCK_PAGAMENTOS_URL") {
            Ok(val) => val,
            Err(_) => {
                eprintln!("MOCK_PAGAMENTOS_URL environment variable not set");
                return Err(DomainError::Invalid(
                    "MOCK_PAGAMENTOS_URL environment variable not set".to_string(),
                ));
            }
        };

        let API_HOST = match env::var("API_HOST") {
            Ok(val) => val,
            Err(_) => {
                eprintln!("API_HOST environment variable not set");
                return Err(DomainError::Invalid(
                    "API_HOST environment variable not set".to_string(),
                ));
            }
        };

        let webhook_url = format!(
            "https://{}/{}/pagamento",
            API_HOST,
            pagamento.id_pedido().clone()
        );

        let client = reqwest::Client::new();
        //TODO set value properly
        let body = serde_json::json!({
            "webhook_url": webhook_url,
            "value": 100,
        });

        let response = client.post(post_url).json(&body).send().await;

        match response {
            Ok(resp) => {
                if resp.status().is_success() {
                    match resp.json::<Value>().await {
                        Ok(json_body) => {
                            if let Some(payment_code) = json_body["payment_code"].as_str() {
                                pagamento.set_referencia(payment_code.to_string());
                                println!("webhook set successful");
                                Ok(pagamento)
                            } else {
                                eprintln!("Payment code not found in the response");
                                Err(DomainError::Invalid("Internal Server Error".to_string()))?
                            }
                        }
                        Err(e) => {
                            println!("Error parsing JSON body: {:?}", e);
                            Err(DomainError::Invalid("Internal Server Error".to_string()))?
                        }
                    }
                } else {
                    println!("webhook set failed with status code");
                    Err(DomainError::Invalid("Internal Server Error".to_string()))
                }
            }
            Err(e) => {
                // Handle errors here
                println!("An error occurred: {:?}", e);
                Err(DomainError::Invalid("Internal Server Error".to_string()))
            }
        }
    }
}
