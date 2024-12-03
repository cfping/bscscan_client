use reqwest::Error;

use crate::types::{
    contracts::{Contract, ContractCreation},
    ApiResponse,
};

pub async fn get_contract_abi(
    client: &reqwest::Client,
    base_url: &str,
    api_key: &str,
    address: &str,
) -> Result<String, Error> {
    let url = format!(
        "{}/api?module=contract&action=getabi&address={}&apikey={}",
        base_url, address, api_key
    );

    let response = client.get(&url).send().await?;
    let api_response: ApiResponse<String> = response.json().await?;

    Ok(api_response.result)
}

pub async fn get_contract_source_code(
    client: &reqwest::Client,
    base_url: &str,
    api_key: &str,
    address: &str,
) -> Result<Vec<Contract>, Error> {
    let url = format!(
        "{}/api?module=contract&action=getsourcecode&address={}&apikey={}",
        base_url, address, api_key
    );

    let response = client.get(&url).send().await?;
    let api_response: ApiResponse<Vec<Contract>> = response.json().await?;

    Ok(api_response.result)
}

pub async fn get_contract_creation(
    client: &reqwest::Client,
    base_url: &str,
    api_key: &str,
    addresses: &[&str],
) -> Result<Vec<ContractCreation>, Error> {
    let addresses_str = addresses.join(",");
    let url = format!(
        "{}/api?module=contract&action=getcontractcreation&contractaddresses={}&apikey={}",
        base_url, addresses_str, api_key
    );

    let response = client.get(&url).send().await?;
    let api_response: ApiResponse<Vec<ContractCreation>> = response.json().await?;

    Ok(api_response.result)
}
