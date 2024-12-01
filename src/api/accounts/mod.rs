use reqwest::Error;
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct Transaction {
    pub blockNumber: String,
    pub timeStamp: String,
    pub hash: String,
    pub nonce: String,
    pub blockHash: String,
    pub transactionIndex: String,
    pub from: String,
    pub to: String,
    pub value: String,
    pub gas: String,
    pub gasPrice: String,
    pub isError: String,
    pub txreceipt_status: String,
    pub input: String,
    pub contractAddress: String,
    pub cumulativeGasUsed: String,
    pub gasUsed: String,
    pub confirmations: String,
    pub methodId: String,
    pub functionName: String,
}

#[derive(Deserialize, Debug)]
pub struct ApiResponse {
    pub status: String,
    pub message: String,
    pub result: Vec<Transaction>,
}

#[derive(Deserialize, Debug)]
pub struct BalanceResponse {
    pub status: String,
    pub message: String,
    pub result: String,
}

#[derive(Deserialize, Debug)]
pub struct BalanceMultiResponse {
    pub status: String,
    pub message: String,
    pub result: Vec<Balance>,
}

#[derive(Deserialize, Debug)]
pub struct Balance {
    pub account: String,
    pub balance: String,
}

pub async fn get_bnb_balance(client:&reqwest::Client, base_url: &str,api_key: &str, address: &str) -> Result<String, Error> {
    let url = format!(
        "{}/api?module=account&action=balance&address={}&tag=latest&apikey={}",base_url,
        address, api_key
    );

    let response = client.get(&url).send().await?;
    let api_response: BalanceResponse = response.json().await?;

    Ok(api_response.result)
}

pub async fn get_bnb_balance_multiple(client:&reqwest::Client, base_url: &str,api_key: &str, addresses: &[&str]) -> Result<Vec<Balance>, Error> {
    let addresses_str = addresses.join(",");
    let url = format!(
        "{}/api?module=account&action=balancemulti&address={}&tag=latest&apikey={}",base_url,
        addresses_str, api_key
    );

    let response = client.get(&url).send().await?;
    let api_response: BalanceMultiResponse = response.json().await?;

    Ok(api_response.result)
}

pub async fn get_normal_transactions(client:&reqwest::Client, base_url: &str,api_key: &str, address: &str, start_block: &str, end_block: &str) -> Result<Vec<Transaction>, Error> {
    let url = format!(
        "{}/api?module=account&action=txlist&address={}&startblock={}&endblock={}&sort=asc&apikey={}",base_url,
        address, start_block, end_block, api_key
    );

    let response = client.get(&url).send().await?;
    let api_response: ApiResponse = response.json().await?;

    Ok(api_response.result)
}

pub async fn get_internal_transactions(client:&reqwest::Client, base_url: &str,api_key: &str, address: &str, start_block: &str, end_block: &str) -> Result<Vec<Transaction>, Error> {
    let url = format!(
        "{}/api?module=account&action=txlistinternal&address={}&startblock={}&endblock={}&sort=asc&apikey={}",base_url,
        address, start_block, end_block, api_key
    );

    let response = client.get(&url).send().await?;
    let api_response: ApiResponse = response.json().await?;

    Ok(api_response.result)
}

pub async fn get_token_transfers(client:&reqwest::Client, base_url: &str,api_key: &str, address: &str, start_block: &str, end_block: &str) -> Result<Vec<Transaction>, Error> {
    let url = format!(
        "{}/api?module=account&action=tokentx&address={}&startblock={}&endblock={}&sort=asc&apikey={}",base_url,
        address, start_block, end_block, api_key
    );

    let response = client.get(&url).send().await?;
    let api_response: ApiResponse = response.json().await?;

    Ok(api_response.result)
}