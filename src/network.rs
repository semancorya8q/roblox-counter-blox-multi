use reqwest::blocking::Client;
use serde_json::Value;

pub struct Network {
    client: Client,
}

impl Network {
    pub fn new() -> Self {
        Network {
            client: Client::new(),
        }
    }

    pub fn fetch_data(&self, url: &str) -> Result<Value, reqwest::Error> {
        let response = self.client.get(url).send()?;
        let json: Value = response.json()?;
        Ok(json)
    }
}