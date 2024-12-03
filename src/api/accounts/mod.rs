use reqwest::Error;

use crate::types::{
    accounts::{Balance, Transaction},
    ApiResponse,
};

pub async fn get_bnb_balance(
    client: &reqwest::Client,
    base_url: &str,
    api_key: &str,
    address: &str,
) -> Result<String, Error> {
    let url = format!(
        "{}/api?module=account&action=balance&address={}&tag=latest&apikey={}",
        base_url, address, api_key
    );

    let response = client.get(&url).send().await?;
    let api_response: ApiResponse<String> = response.json().await?;

    Ok(api_response.result)
}

pub async fn get_bnb_balance_multiple(
    client: &reqwest::Client,
    base_url: &str,
    api_key: &str,
    addresses: &[&str],
) -> Result<Vec<Balance>, Error> {
    let addresses_str = addresses.join(",");
    let url = format!(
        "{}/api?module=account&action=balancemulti&address={}&tag=latest&apikey={}",
        base_url, addresses_str, api_key
    );

    let response = client.get(&url).send().await?;
    let api_response: ApiResponse<Vec<Balance>> = response.json().await?;

    Ok(api_response.result)
}

pub async fn get_normal_transactions(
    client: &reqwest::Client,
    base_url: &str,
    api_key: &str,
    address: &str,
    start_block: &str,
    end_block: &str,
) -> Result<Vec<Transaction>, Error> {
    let url = format!(
        "{}/api?module=account&action=txlist&address={}&startblock={}&endblock={}&sort=asc&apikey={}",base_url,
        address, start_block, end_block, api_key
    );

    let response = client.get(&url).send().await?;
    let api_response: ApiResponse<Vec<Transaction>> = response.json().await?;

    Ok(api_response.result)
}

pub async fn get_internal_transactions(
    client: &reqwest::Client,
    base_url: &str,
    api_key: &str,
    address: &str,
    start_block: &str,
    end_block: &str,
) -> Result<Vec<Transaction>, Error> {
    let url = format!(
        "{}/api?module=account&action=txlistinternal&address={}&startblock={}&endblock={}&sort=asc&apikey={}",base_url,
        address, start_block, end_block, api_key
    );

    let response = client.get(&url).send().await?;
    let api_response: ApiResponse<Vec<Transaction>> = response.json().await?;

    Ok(api_response.result)
}

pub async fn get_token_transfers(
    client: &reqwest::Client,
    base_url: &str,
    api_key: &str,
    address: &str,
    start_block: &str,
    end_block: &str,
) -> Result<Vec<Transaction>, Error> {
    let url = format!(
        "{}/api?module=account&action=tokentx&address={}&startblock={}&endblock={}&sort=asc&apikey={}",base_url,
        address, start_block, end_block, api_key
    );

    let response = client.get(&url).send().await?;
    let api_response: ApiResponse<Vec<Transaction>> = response.json().await?;

    Ok(api_response.result)
}
