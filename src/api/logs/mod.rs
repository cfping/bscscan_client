use reqwest::Error;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct LogResponse {
    status: String,
    message: String,
    result: Vec<LogEntry>,
}

#[derive(Deserialize,Serialize)]
struct LogEntry {
    address: String,
    topics: Vec<String>,
    data: String,
    blockNumber: String,
    blockHash: String,
    timeStamp: String,
    gasPrice: String,
    gasUsed: String,
    logIndex: String,
    transactionHash: String,
    transactionIndex: String,
}

pub async fn fetch_logs(client:&reqwest::Client, base_url: &str,
    api_key: &str,
    from_block: u64,
    to_block: u64,
    address: &str,
    topic0: &str,
) -> Result<LogResponse, Error> {
    let url = format!(
        "{}/api?module=logs&action=getLogs&fromBlock={}&toBlock={}&address={}&topic0={}&apikey={}",base_url,
        from_block, to_block, address, topic0, api_key
    );

    let response = client.get(&url).send().await?;
    let log_response: LogResponse = response.json().await?;

    Ok(log_response)
}