// src/geth_parity_proxy.rs
use reqwest::Client;
use reqwest::Error;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ApiProxyResponse {
    pub jsonrpc: String,
    pub id: u64,
    pub result: serde_json::Value,
}

pub async fn eth_block_number(
    client: &reqwest::Client,
    base_url: &str,
    api_key: &str,
) -> Result<ApiProxyResponse, Error> {
    let url = format!(
        "{}?module=proxy&action=eth_blockNumber&apikey={}",
        base_url, api_key
    );
    let response = client.get(&url).send().await?;
    let api_response: ApiProxyResponse = response.json().await?;
    Ok(api_response)
}

pub async fn eth_get_block_by_number(
    client: &reqwest::Client,
    base_url: &str,
    tag: &str,
    boolean: bool,
    api_key: &str,
) -> Result<ApiProxyResponse, Error> {
    let url = format!(
        "{}?module=proxy&action=eth_getBlockByNumber&tag={}&boolean={}&apikey={}",
        base_url, tag, boolean, api_key
    );
    let response = client.get(&url).send().await?;
    let api_response: ApiProxyResponse = response.json().await?;
    Ok(api_response)
}

pub async fn eth_get_block_transaction_count_by_number(
    client: &reqwest::Client,
    base_url: &str,
    tag: &str,
    api_key: &str,
) -> Result<ApiProxyResponse, Error> {
    let url = format!(
        "{}?module=proxy&action=eth_getBlockTransactionCountByNumber&tag={}&apikey={}",
        base_url, tag, api_key
    );
    let response = client.get(&url).send().await?;
    let api_response: ApiProxyResponse = response.json().await?;
    Ok(api_response)
}

pub async fn eth_get_transaction_by_hash(
    client: &reqwest::Client,
    base_url: &str,
    txhash: &str,
    api_key: &str,
) -> Result<ApiProxyResponse, Error> {
    let url = format!(
        "{}?module=proxy&action=eth_getTransactionByHash&txhash={}&apikey={}",
        base_url, txhash, api_key
    );
    let response = client.get(&url).send().await?;
    let api_response: ApiProxyResponse = response.json().await?;
    Ok(api_response)
}

pub async fn eth_get_transaction_by_block_number_and_index(
    client: &reqwest::Client,
    base_url: &str,
    tag: &str,
    index: &str,
    api_key: &str,
) -> Result<ApiProxyResponse, Error> {
    let url = format!(
        "{}?module=proxy&action=eth_getTransactionByBlockNumberAndIndex&tag={}&index={}&apikey={}",
        base_url, tag, index, api_key
    );
    let response = client.get(&url).send().await?;
    let api_response: ApiProxyResponse = response.json().await?;
    Ok(api_response)
}

pub async fn eth_get_transaction_count(
    client: &reqwest::Client,
    base_url: &str,
    address: &str,
    tag: &str,
    api_key: &str,
) -> Result<ApiProxyResponse, Error> {
    let url = format!(
        "{}?module=proxy&action=eth_getTransactionCount&address={}&tag={}&apikey={}",
        base_url, address, tag, api_key
    );
    let response = client.get(&url).send().await?;
    let api_response: ApiProxyResponse = response.json().await?;
    Ok(api_response)
}

pub async fn eth_send_raw_transaction(
    client: &reqwest::Client,
    base_url: &str,
    hex: &str,
    api_key: &str,
) -> Result<ApiProxyResponse, Error> {
    let url = format!(
        "{}?module=proxy&action=eth_sendRawTransaction&hex={}&apikey={}",
        base_url, hex, api_key
    );
    let response = client.get(&url).send().await?;
    let api_response: ApiProxyResponse = response.json().await?;
    Ok(api_response)
}

pub async fn eth_get_transaction_receipt(
    client: &reqwest::Client,
    base_url: &str,
    txhash: &str,
    api_key: &str,
) -> Result<ApiProxyResponse, Error> {
    let url = format!(
        "{}?module=proxy&action=eth_getTransactionReceipt&txhash={}&apikey={}",
        base_url, txhash, api_key
    );
    let response = client.get(&url).send().await?;
    let api_response: ApiProxyResponse = response.json().await?;
    Ok(api_response)
}

pub async fn eth_call(
    client: &reqwest::Client,
    base_url: &str,
    to: &str,
    data: &str,
    tag: &str,
    api_key: &str,
) -> Result<ApiProxyResponse, Error> {
    let url = format!(
        "{}?module=proxy&action=eth_call&to={}&data={}&tag={}&apikey={}",
        base_url, to, data, tag, api_key
    );
    let response = client.get(&url).send().await?;
    let api_response: ApiProxyResponse = response.json().await?;
    Ok(api_response)
}

pub async fn eth_get_code(
    client: &reqwest::Client,
    base_url: &str,
    address: &str,
    tag: &str,
    api_key: &str,
) -> Result<ApiProxyResponse, Error> {
    let url = format!(
        "{}?module=proxy&action=eth_getCode&address={}&tag={}&apikey={}",
        base_url, address, tag, api_key
    );
    let response = client.get(&url).send().await?;
    let api_response: ApiProxyResponse = response.json().await?;
    Ok(api_response)
}

pub async fn eth_get_storage_at(
    client: &reqwest::Client,
    base_url: &str,
    address: &str,
    position: &str,
    tag: &str,
    api_key: &str,
) -> Result<ApiProxyResponse, Error> {
    let url = format!(
        "{}?module=proxy&action=eth_getStorageAt&address={}&position={}&tag={}&apikey={}",
        base_url, address, position, tag, api_key
    );
    let response = client.get(&url).send().await?;
    let api_response: ApiProxyResponse = response.json().await?;
    Ok(api_response)
}

pub async fn eth_gas_price(
    client: &reqwest::Client,
    base_url: &str,
    api_key: &str,
) -> Result<ApiProxyResponse, Error> {
    let url = format!(
        "{}/?module=proxy&action=eth_gasPrice&apikey={}",
        base_url, api_key
    );
    let response = client.get(&url).send().await?;
    let api_response: ApiProxyResponse = response.json().await?;
    Ok(api_response)
}

pub async fn eth_estimate_gas(
    client: &reqwest::Client,
    base_url: &str,
    data: &str,
    to: &str,
    value: &str,
    gas_price: &str,
    gas: &str,
    api_key: &str,
) -> Result<ApiProxyResponse, Error> {
    let url = format!(
        "{}/?module=proxy&action=eth_estimateGas&data={}&to={}&value={}&gasPrice={}&gas={}&apikey={}",base_url,data, to, value, gas_price, gas, api_key
    );
    let response = client.get(&url).send().await?;
    let api_response: ApiProxyResponse = response.json().await?;
    Ok(api_response)
}
