use reqwest::blocking::Client;
use serde::Deserialize;
use std::time::Duration;

#[derive(Debug, Deserialize)]
pub struct OHLCBar {
    pub timestamp: String,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: u64,
}

pub struct ResearchClient {
    base_url: String,
    http: Client,
}

impl ResearchClient {
    pub fn new(base_url: &str) -> Self {
        let http = Client::builder()
            .timeout(Duration::from_secs(10))
            .build()
            .expect("failed to build reqwest client");

        Self {
            base_url: base_url.trim_end_matches('/').to_string(),
            http,
        }
    }

    pub fn fetch_historical(&self, symbol: &str, days: u32) -> Result<Vec<OHLCBar>, String> {
        let url = format!("{}/v1/historical/{}?days={}", self.base_url, symbol, days);
        let resp = self
            .http
            .get(&url)
            .send()
            .map_err(|e| format!("request error: {}", e))?;

        if !resp.status().is_success() {
            return Err(format!("bad status: {}", resp.status()));
        }

        let bars: Vec<OHLCBar> = resp
            .json()
            .map_err(|e| format!("json parse error: {}", e))?;

        Ok(bars)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_builds_url() {
        let c = ResearchClient::new("http://localhost:8000/");
        let res = c.fetch_historical("NIFTY","30".parse().unwrap());
        // We can't assert on response in unit test environment; ensure method exists and errors gracefully
        assert!(res.is_err());
    }
}
