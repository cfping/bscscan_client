use reqwest::Error;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct TokenResponse {
    status: String,
    message: String,
    result: String,
}

#[derive(Deserialize)]
pub struct TokenHoldersResponse {
    status: String,
    message: String,
    result: Vec<TokenHolder>,
}

#[derive(Deserialize)]
struct TokenHolder {
    TokenHolderAddress: String,
    TokenHolderQuantity: String,
}

#[derive(Deserialize)]
pub struct TokenInfoResponse {
    status: String,
    message: String,
    result: Vec<TokenInfo>,
}

#[derive(Deserialize)]
struct TokenInfo {
    contractAddress: String,
    tokenName: String,
    symbol: String,
    divisor: String,
    tokenType: String,
    totalSupply: String,
    blueCheckmark: String,
    description: String,
    website: String,
    email: String,
    blog: String,
    reddit: String,
    slack: String,
    facebook: String,
    twitter: String,
    bitcointalk: String,
    github: String,
    telegram: String,
    wechat: String,
    linkedin: String,
    discord: String,
    whitepaper: String,
    tokenPriceUSD: String,
}

#[derive(Deserialize)]
pub struct AddressTokenBalanceResponse {
    status: String,
    message: String,
    result: Vec<AddressTokenBalance>,
}

#[derive(Deserialize)]
struct AddressTokenBalance {
    TokenAddress: String,
    TokenName: String,
    TokenSymbol: String,
    TokenQuantity: String,
    TokenDivisor: String,
}

#[derive(Deserialize)]
pub struct AddressNFTBalanceResponse {
    status: String,
    message: String,
    result: Vec<AddressNFTBalance>,
}

#[derive(Deserialize)]
struct AddressNFTBalance {
    TokenAddress: String,
    TokenName: String,
    TokenSymbol: String,
    TokenQuantity: String,
}

#[derive(Deserialize)]
pub struct AddressNFTInventoryResponse {
    status: String,
    message: String,
    result: Vec<AddressNFTInventory>,
}

#[derive(Deserialize)]
struct AddressNFTInventory {
    TokenAddress: String,
    TokenId: String,
}

pub async fn get_bep20_token_circulating_supply(
    client: &reqwest::Client,
    base_url: &str,
    api_key: &str,
    contract_address: &str,
) -> Result<TokenResponse, Error> {
    let url = format!(
        "{}/api?module=stats&action=tokenCsupply&contractaddress={}&apikey={}",
        base_url, contract_address, api_key
    );

    let response = client.get(&url).send().await?;
    let token_response: TokenResponse = response.json().await?;

    Ok(token_response)
}

pub async fn get_bep20_token_total_supply(
    client: &reqwest::Client,
    base_url: &str,
    api_key: &str,
    contract_address: &str,
) -> Result<TokenResponse, Error> {
    let url = format!(
        "{}/api?module=stats&action=tokensupply&contractaddress={}&apikey={}",
        base_url, contract_address, api_key
    );

    let response = client.get(&url).send().await?;
    let token_response: TokenResponse = response.json().await?;

    Ok(token_response)
}

pub async fn get_bep20_token_account_balance(
    client: &reqwest::Client,
    base_url: &str,
    api_key: &str,
    contract_address: &str,
    address: &str,
) -> Result<TokenResponse, Error> {
    let url = format!(
        "{}/api?module=account&action=tokenbalance&contractaddress={}&address={}&tag=latest&apikey={}",base_url,
        contract_address, address, api_key
    );

    let response = client.get(&url).send().await?;
    let token_response: TokenResponse = response.json().await?;

    Ok(token_response)
}

pub async fn get_historical_bep20_token_total_supply(
    client: &reqwest::Client,
    base_url: &str,
    api_key: &str,
    contract_address: &str,
    block_no: u64,
) -> Result<TokenResponse, Error> {
    let url = format!(
        "{}/api?module=stats&action=tokensupplyhistory&contractaddress={}&blockno={}&apikey={}",
        base_url, contract_address, block_no, api_key
    );

    let response = client.get(&url).send().await?;
    let token_response: TokenResponse = response.json().await?;

    Ok(token_response)
}

pub async fn get_historical_bep20_token_account_balance(
    client: &reqwest::Client,
    base_url: &str,
    api_key: &str,
    contract_address: &str,
    address: &str,
    block_no: u64,
) -> Result<TokenResponse, Error> {
    let url = format!(
        "{}/api?module=account&action=tokenbalancehistory&contractaddress={}&address={}&blockno={}&apikey={}",base_url,
        contract_address, address, block_no, api_key
    );

    let response = client.get(&url).send().await?;
    let token_response: TokenResponse = response.json().await?;

    Ok(token_response)
}

pub async fn get_token_holder_list(
    client: &reqwest::Client,
    base_url: &str,
    api_key: &str,
    contract_address: &str,
    page: u64,
    offset: u64,
) -> Result<TokenHoldersResponse, Error> {
    let url = format!(
        "{}/api?module=token&action=tokenholderlist&contractaddress={}&page={}&offset={}&apikey={}",
        base_url, contract_address, page, offset, api_key
    );

    let response = client.get(&url).send().await?;
    let token_holders_response: TokenHoldersResponse = response.json().await?;

    Ok(token_holders_response)
}

pub async fn get_token_info(
    client: &reqwest::Client,
    base_url: &str,
    api_key: &str,
    contract_address: &str,
) -> Result<TokenInfoResponse, Error> {
    let url = format!(
        "{}/api?module=token&action=tokeninfo&contractaddress={}&apikey={}",
        base_url, contract_address, api_key
    );

    let response = client.get(&url).send().await?;
    let token_info_response: TokenInfoResponse = response.json().await?;

    Ok(token_info_response)
}

pub async fn get_address_bep20_token_holding(
    client: &reqwest::Client,
    base_url: &str,
    api_key: &str,
    address: &str,
    page: u64,
    offset: u64,
) -> Result<AddressTokenBalanceResponse, Error> {
    let url = format!(
        "{}/api?module=account&action=addresstokenbalance&address={}&page={}&offset={}&apikey={}",
        base_url, address, page, offset, api_key
    );

    let response = client.get(&url).send().await?;
    let address_token_balance_response: AddressTokenBalanceResponse = response.json().await?;

    Ok(address_token_balance_response)
}

pub async fn get_address_bep721_token_holding(
    client: &reqwest::Client,
    base_url: &str,
    api_key: &str,
    address: &str,
    page: u64,
    offset: u64,
) -> Result<AddressNFTBalanceResponse, Error> {
    let url = format!(
        "{}/api?module=account&action=addresstokennftbalance&address={}&page={}&offset={}&apikey={}",base_url,
        address, page, offset, api_key
    );

    let response = client.get(&url).send().await?;
    let address_nft_balance_response: AddressNFTBalanceResponse = response.json().await?;

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
) -> Result<AddressNFTInventoryResponse, Error> {
    let url = format!(
        "{}/api?module=account&action=addresstokennftinventory&address={}&contractaddress={}&page={}&offset={}&apikey={}",base_url,
        address, contract_address, page, offset, api_key
    );

    let response = client.get(&url).send().await?;
    let address_nft_inventory_response: AddressNFTInventoryResponse = response.json().await?;

    Ok(address_nft_inventory_response)
}
