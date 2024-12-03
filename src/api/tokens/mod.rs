use reqwest::Error;

use crate::types::{
    tokens::{AddressNFTBalance, AddressNFTInventory, AddressTokenBalance, TokenHolder, TokenInfo},
    ApiResponse,
};

pub async fn get_bep20_token_circulating_supply(
    client: &reqwest::Client,
    base_url: &str,
    api_key: &str,
    contract_address: &str,
) -> Result<ApiResponse<String>, Error> {
    let url = format!(
        "{}/api?module=stats&action=tokenCsupply&contractaddress={}&apikey={}",
        base_url, contract_address, api_key
    );

    let response = client.get(&url).send().await?;
    let token_response: ApiResponse<String> = response.json().await?;

    Ok(token_response)
}

pub async fn get_bep20_token_total_supply(
    client: &reqwest::Client,
    base_url: &str,
    api_key: &str,
    contract_address: &str,
) -> Result<ApiResponse<String>, Error> {
    let url = format!(
        "{}/api?module=stats&action=tokensupply&contractaddress={}&apikey={}",
        base_url, contract_address, api_key
    );

    let response = client.get(&url).send().await?;
    let token_response: ApiResponse<String> = response.json().await?;

    Ok(token_response)
}

pub async fn get_bep20_token_account_balance(
    client: &reqwest::Client,
    base_url: &str,
    api_key: &str,
    contract_address: &str,
    address: &str,
) -> Result<ApiResponse<String>, Error> {
    let url = format!(
        "{}/api?module=account&action=tokenbalance&contractaddress={}&address={}&tag=latest&apikey={}",base_url,
        contract_address, address, api_key
    );

    let response = client.get(&url).send().await?;
    let token_response: ApiResponse<String> = response.json().await?;

    Ok(token_response)
}

pub async fn get_historical_bep20_token_total_supply(
    client: &reqwest::Client,
    base_url: &str,
    api_key: &str,
    contract_address: &str,
    block_no: u64,
) -> Result<ApiResponse<String>, Error> {
    let url = format!(
        "{}/api?module=stats&action=tokensupplyhistory&contractaddress={}&blockno={}&apikey={}",
        base_url, contract_address, block_no, api_key
    );

    let response = client.get(&url).send().await?;
    let token_response: ApiResponse<String> = response.json().await?;

    Ok(token_response)
}

pub async fn get_historical_bep20_token_account_balance(
    client: &reqwest::Client,
    base_url: &str,
    api_key: &str,
    contract_address: &str,
    address: &str,
    block_no: u64,
) -> Result<ApiResponse<String>, Error> {
    let url = format!(
        "{}/api?module=account&action=tokenbalancehistory&contractaddress={}&address={}&blockno={}&apikey={}",base_url,
        contract_address, address, block_no, api_key
    );

    let response = client.get(&url).send().await?;
    let token_response: ApiResponse<String> = response.json().await?;

    Ok(token_response)
}

pub async fn get_token_holder_list(
    client: &reqwest::Client,
    base_url: &str,
    api_key: &str,
    contract_address: &str,
    page: u64,
    offset: u64,
) -> Result<ApiResponse<Vec<TokenHolder>>, Error> {
    let url = format!(
        "{}/api?module=token&action=tokenholderlist&contractaddress={}&page={}&offset={}&apikey={}",
        base_url, contract_address, page, offset, api_key
    );

    let response = client.get(&url).send().await?;
    let token_holders_response: ApiResponse<Vec<TokenHolder>> = response.json().await?;

    Ok(token_holders_response)
}

pub async fn get_token_info(
    client: &reqwest::Client,
    base_url: &str,
    api_key: &str,
    contract_address: &str,
) -> Result<ApiResponse<Vec<TokenInfo>>, Error> {
    let url = format!(
        "{}/api?module=token&action=tokeninfo&contractaddress={}&apikey={}",
        base_url, contract_address, api_key
    );

    let response = client.get(&url).send().await?;
    let token_info_response: ApiResponse<Vec<TokenInfo>> = response.json().await?;

    Ok(token_info_response)
}

pub async fn get_address_bep20_token_holding(
    client: &reqwest::Client,
    base_url: &str,
    api_key: &str,
    address: &str,
    page: u64,
    offset: u64,
) -> Result<ApiResponse<Vec<AddressTokenBalance>>, Error> {
    let url = format!(
        "{}/api?module=account&action=addresstokenbalance&address={}&page={}&offset={}&apikey={}",
        base_url, address, page, offset, api_key
    );

    let response = client.get(&url).send().await?;
    let address_token_balance_response: ApiResponse<Vec<AddressTokenBalance>> =
        response.json().await?;

    Ok(address_token_balance_response)
}

pub async fn get_address_bep721_token_holding(
    client: &reqwest::Client,
    base_url: &str,
    api_key: &str,
    address: &str,
    page: u64,
    offset: u64,
) -> Result<ApiResponse<Vec<AddressNFTBalance>>, Error> {
    let url = format!(
        "{}/api?module=account&action=addresstokennftbalance&address={}&page={}&offset={}&apikey={}",base_url,
        address, page, offset, api_key
    );

    let response = client.get(&url).send().await?;
    let address_nft_balance_response: ApiResponse<Vec<AddressNFTBalance>> = response.json().await?;

    Ok(address_nft_balance_response)
}

pub async fn get_address_bep721_token_inventory(
    client: &reqwest::Client,
    base_url: &str,
    api_key: &str,
    address: &str,
    contract_address: &str,
    page: u64,
    offset: u64,
) -> Result<ApiResponse<Vec<AddressNFTInventory>>, Error> {
    let url = format!(
        "{}/api?module=account&action=addresstokennftinventory&address={}&contractaddress={}&page={}&offset={}&apikey={}",base_url,
        address, contract_address, page, offset, api_key
    );

    let response = client.get(&url).send().await?;
    let address_nft_inventory_response: ApiResponse<Vec<AddressNFTInventory>> =
        response.json().await?;

    Ok(address_nft_inventory_response)
}
