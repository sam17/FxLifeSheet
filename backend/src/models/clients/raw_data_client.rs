use reqwest::{Client, Error, Response};
use serde::Deserialize;
use std::sync::Arc;
use crate::models::collector::raw_data::PublishableDataObj;

pub struct RawDataClient {
    base_url: String,
    client: Arc<Client>,
}

impl RawDataClient {
    // Initialize a new RawDataClient
    pub fn new(base_url: &str) -> Self {
        RawDataClient {
            base_url: base_url.to_string(),
            client: Arc::new(Client::new()),
        }
    }

    // Helper function for making requests
    async fn make_request(&self, path: &str) -> Result<Response, Error> {
        self.client.get(&format!("{}{}", self.base_url, path)).send().await
    }

    // Get all raw data
    pub async fn get_all_raw_data(&self) -> Result<Vec<PublishableDataObj>, Error> {
        let response = self.make_request("/data").await?;
        let raw_data: Vec<PublishableDataObj> = response.json().await?;
        Ok(raw_data)
    }

    // Get raw data by key
    pub async fn get_raw_data_by_key(&self, key: &str) -> Result<PublishableDataObj, Error> {
        let response = self.make_request(&format!("/data/{}", key)).await?;
        let raw_data: PublishableDataObj = response.json().await?;
        Ok(raw_data)
    }

    // Post raw data
    pub async fn post_raw_data(&self, data: &PublishableDataObj) -> Result<PublishableDataObj, Error> {
        let response = self.client.post(&format!("{}{}", self.base_url, "/data"))
            .json(data)
            .send()
            .await?;

        let posted_data: PublishableDataObj = response.json().await?;
        Ok(posted_data)
    }
}
