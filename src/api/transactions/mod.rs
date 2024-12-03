use reqwest::Error;

use crate::types::{
    transactions::{ContractExecutionStatus, TransactionReceiptStatus},
    ApiResponse,
};

pub async fn get_contract_execution_status(
    client: &reqwest::Client,
    base_url: &str,
    api_key: &str,
    txhash: &str,
) -> Result<ContractExecutionStatus, Error> {
    let url = format!(
        "{}/api?module=transaction&action=getstatus&txhash={}&apikey={}",
        base_url, txhash, api_key
    );

    let response = client.get(&url).send().await?;
    let api_response: ApiResponse<ContractExecutionStatus> = response.json().await?;

    Ok(api_response.result)
}

pub async fn get_transaction_receipt_status(
    client: &reqwest::Client,
    base_url: &str,
    api_key: &str,
    txhash: &str,
) -> Result<TransactionReceiptStatus, Error> {
    let url = format!(
        "{}/api?module=transaction&action=gettxreceiptstatus&txhash={}&apikey={}",
        base_url, txhash, api_key
    );

    let response = client.get(&url).send().await?;
    let api_response: ApiResponse<TransactionReceiptStatus> = response.json().await?;

    Ok(api_response.result)
}
