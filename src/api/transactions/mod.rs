use reqwest::Error;
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct ContractExecutionStatus {
    pub isError: String,
    pub errDescription: String,
}

#[derive(Deserialize, Debug)]
pub struct ContractExecutionStatusResponse {
    pub status: String,
    pub message: String,
    pub result: ContractExecutionStatus,
}

#[derive(Deserialize, Debug, Clone)]
pub struct TransactionReceiptStatus {
    pub status: String,
}

#[derive(Deserialize, Debug)]
pub struct TransactionReceiptStatusResponse {
    pub status: String,
    pub message: String,
    pub result: TransactionReceiptStatus,
}

pub async fn get_contract_execution_status(client:&reqwest::Client, base_url: &str,api_key: &str, txhash: &str) -> Result<ContractExecutionStatus, Error> {
    let url = format!(
        "{}/api?module=transaction&action=getstatus&txhash={}&apikey={}",base_url,
        txhash, api_key
    );

    let response = client.get(&url).send().await?;
    let api_response: ContractExecutionStatusResponse = response.json().await?;

    Ok(api_response.result)
}

pub async fn get_transaction_receipt_status(client:&reqwest::Client, base_url: &str,api_key: &str, txhash: &str) -> Result<TransactionReceiptStatus, Error> {
    let url = format!(
        "{}/api?module=transaction&action=gettxreceiptstatus&txhash={}&apikey={}",base_url,
        txhash, api_key
    );

    let response = client.get(&url).send().await?;
    let api_response: TransactionReceiptStatusResponse = response.json().await?;

    Ok(api_response.result)
}