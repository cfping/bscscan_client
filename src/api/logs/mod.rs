use reqwest::Error;

use crate::types::{logs::LogEntry, ApiResponse};

pub async fn fetch_logs(
    client: &reqwest::Client,
    base_url: &str,
    api_key: &str,
    from_block: u64,
    to_block: u64,
    address: &str,
    topic0: &str,
) -> Result<ApiResponse<Vec<LogEntry>>, Error> {
    let url = format!(
        "{}/api?module=logs&action=getLogs&fromBlock={}&toBlock={}&address={}&topic0={}&apikey={}",
        base_url, from_block, to_block, address, topic0, api_key
    );

    let response = client.get(&url).send().await?;
    let log_response: ApiResponse<Vec<LogEntry>> = response.json().await?;

    Ok(log_response)
}
