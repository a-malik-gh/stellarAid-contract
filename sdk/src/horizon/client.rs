use reqwest::Client;
use serde::Deserialize;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum HorizonError {
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),
    #[error("Horizon API error: {0}")]
    Api(String),
}

#[derive(Debug, Deserialize)]
pub struct AccountResponse {
    pub id: String,
    pub sequence: String,
    pub balances: Vec<Balance>,
}

#[derive(Debug, Deserialize)]
pub struct Balance {
    pub balance: String,
    pub asset_type: String,
    #[serde(default)]
    pub asset_code: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct TransactionPage {
    pub _embedded: TransactionEmbedded,
}

#[derive(Debug, Deserialize)]
pub struct TransactionEmbedded {
    pub records: Vec<TransactionRecord>,
}

#[derive(Debug, Deserialize)]
pub struct TransactionRecord {
    pub hash: String,
    pub created_at: String,
    pub successful: bool,
}

#[derive(Debug, Deserialize)]
pub struct PaymentPage {
    pub _embedded: PaymentEmbedded,
}

#[derive(Debug, Deserialize)]
pub struct PaymentEmbedded {
    pub records: Vec<PaymentRecord>,
}

#[derive(Debug, Deserialize)]
pub struct PaymentRecord {
    pub id: String,
    #[serde(rename = "type")]
    pub payment_type: String,
    #[serde(default)]
    pub amount: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct TransactionDetail {
    pub hash: String,
    pub created_at: String,
    pub successful: bool,
    pub envelope_xdr: String,
}

pub struct HorizonClient {
    client: Client,
    base_url: String,
}

impl HorizonClient {
    pub fn new(base_url: impl Into<String>) -> Self {
        Self {
            client: Client::new(),
            base_url: base_url.into(),
        }
    }

    pub async fn get_account(&self, address: &str) -> Result<AccountResponse, HorizonError> {
        let url = format!("{}/accounts/{}", self.base_url, address);
        let resp = self.client.get(&url).send().await?;
        if !resp.status().is_success() {
            return Err(HorizonError::Api(resp.text().await.unwrap_or_default()));
        }
        Ok(resp.json().await?)
    }

    pub async fn get_transactions(
        &self,
        address: &str,
        cursor: Option<&str>,
    ) -> Result<TransactionPage, HorizonError> {
        let mut url = format!("{}/accounts/{}/transactions?order=desc&limit=50", self.base_url, address);
        if let Some(c) = cursor {
            url.push_str(&format!("&cursor={}", c));
        }
        let resp = self.client.get(&url).send().await?;
        if !resp.status().is_success() {
            return Err(HorizonError::Api(resp.text().await.unwrap_or_default()));
        }
        Ok(resp.json().await?)
    }

    pub async fn get_payments(
        &self,
        address: &str,
        cursor: Option<&str>,
    ) -> Result<PaymentPage, HorizonError> {
        let mut url = format!("{}/accounts/{}/payments?order=desc&limit=50", self.base_url, address);
        if let Some(c) = cursor {
            url.push_str(&format!("&cursor={}", c));
        }
        let resp = self.client.get(&url).send().await?;
        if !resp.status().is_success() {
            return Err(HorizonError::Api(resp.text().await.unwrap_or_default()));
        }
        Ok(resp.json().await?)
    }

    pub async fn get_transaction(&self, hash: &str) -> Result<TransactionDetail, HorizonError> {
        let url = format!("{}/transactions/{}", self.base_url, hash);
        let resp = self.client.get(&url).send().await?;
        if !resp.status().is_success() {
            return Err(HorizonError::Api(resp.text().await.unwrap_or_default()));
        }
        Ok(resp.json().await?)
    }
}
