use reqwest::Error;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct BnbSupplyResponse {
    status: String,
    message: String,
    result: String,
}

#[derive(Deserialize)]
pub struct ValidatorsResponse {
    status: String,
    message: String,
    result: Vec<Validator>,
}

#[derive(Deserialize)]
struct Validator {
    validatorAddress: String,
    validatorName: String,
    validatorStatus: String,
    validatorVotingPower: String,
    validatorVotingPowerProportion: String,
}

#[derive(Deserialize)]
pub struct BnbPriceResponse {
    status: String,
    message: String,
    result: BnbPriceResult,
}

#[derive(Deserialize)]
struct BnbPriceResult {
    ethbtc: String,
    ethbtc_timestamp: String,
    ethusd: String,
    ethusd_timestamp: String,
}

#[derive(Deserialize)]
pub struct BnbHistoricalPriceResponse {
    status: String,
    message: String,
    result: Vec<BnbHistoricalPrice>,
}

#[derive(Deserialize)]
struct BnbHistoricalPrice {
    UTCDate: String,
    unixTimeStamp: String,
    value: String,
}

#[derive(Deserialize)]
pub struct DailyTransactionFeeResponse {
    status: String,
    message: String,
    result: Vec<DailyTransactionFee>,
}

#[derive(Deserialize)]
struct DailyTransactionFee {
    UTCDate: String,
    unixTimeStamp: String,
    transactionFee_Eth: String,
}

#[derive(Deserialize)]
pub struct DailyNewAddressResponse {
    status: String,
    message: String,
    result: Vec<DailyNewAddress>,
}

#[derive(Deserialize)]
struct DailyNewAddress {
    UTCDate: String,
    unixTimeStamp: String,
    newAddressCount: u64,
}

#[derive(Deserialize)]
pub struct DailyNetworkUtilizationResponse {
    status: String,
    message: String,
    result: Vec<DailyNetworkUtilization>,
}

#[derive(Deserialize)]
struct DailyNetworkUtilization {
    UTCDate: String,
    unixTimeStamp: String,
    networkUtilization: String,
}

#[derive(Deserialize)]
pub struct DailyTransactionCountResponse {
    status: String,
    message: String,
    result: Vec<DailyTransactionCount>,
}

#[derive(Deserialize)]
struct DailyTransactionCount {
    UTCDate: String,
    unixTimeStamp: String,
    transactionCount: u64,
}

pub async fn get_bnb_supply(client:&reqwest::Client, base_url: &str,
    api_key: &str,
) -> Result<BnbSupplyResponse, Error> {
    let url = format!("{}/api?module=stats&action=bnbsupply&apikey={}",base_url, api_key);

    let response = client.get(&url).send().await?;
    let bnb_supply_response: BnbSupplyResponse = response.json().await?;

    Ok(bnb_supply_response)
}

pub async fn get_validators(client:&reqwest::Client, base_url: &str,
    api_key: &str,
) -> Result<ValidatorsResponse, Error> {
    let url = format!("{}/api?module=stats&action=validators&apikey={}",base_url, api_key);

    let response = client.get(&url).send().await?;
    let validators_response: ValidatorsResponse = response.json().await?;

    Ok(validators_response)
}

pub async fn get_bnb_price(client:&reqwest::Client, base_url: &str,
    api_key: &str,
) -> Result<BnbPriceResponse, Error> {
    let url = format!("{}/api?module=stats&action=bnbprice&apikey={}",base_url, api_key);

    let response = client.get(&url).send().await?;
    let bnb_price_response: BnbPriceResponse = response.json().await?;

    Ok(bnb_price_response)
}

pub async fn get_bnb_historical_price(client:&reqwest::Client, base_url: &str,
    api_key: &str,
    start_date: &str,
    end_date: &str,
    sort: &str,
) -> Result<BnbHistoricalPriceResponse, Error> {
    let url = format!(
        "{}/api?module=stats&action=bnbdailyprice&startdate={}&enddate={}&sort={}&apikey={}",base_url,
        start_date, end_date, sort, api_key
    );

    let response = client.get(&url).send().await?;
    let bnb_historical_price_response: BnbHistoricalPriceResponse = response.json().await?;

    Ok(bnb_historical_price_response)
}

pub async fn get_daily_transaction_fee(client:&reqwest::Client, base_url: &str,
    api_key: &str,
    start_date: &str,
    end_date: &str,
    sort: &str,
) -> Result<DailyTransactionFeeResponse, Error> {
    let url = format!(
        "{}/api?module=stats&action=dailytxnfee&startdate={}&enddate={}&sort={}&apikey={}",base_url,
        start_date, end_date, sort, api_key
    );

    let response = client.get(&url).send().await?;
    let daily_transaction_fee_response: DailyTransactionFeeResponse = response.json().await?;

    Ok(daily_transaction_fee_response)
}

pub async fn get_daily_new_address(client:&reqwest::Client, base_url: &str,
    api_key: &str,
    start_date: &str,
    end_date: &str,
    sort: &str,
) -> Result<DailyNewAddressResponse, Error> {
    let url = format!(
        "{}/api?module=stats&action=dailynewaddress&startdate={}&enddate={}&sort={}&apikey={}",base_url,
        start_date, end_date, sort, api_key
    );

    let response = client.get(&url).send().await?;
    let daily_new_address_response: DailyNewAddressResponse = response.json().await?;

    Ok(daily_new_address_response)
}

pub async fn get_daily_network_utilization(client:&reqwest::Client, base_url: &str,
    api_key: &str,
    start_date: &str,
    end_date: &str,
    sort: &str,
) -> Result<DailyNetworkUtilizationResponse, Error> {
    let url = format!(
        "{}/api?module=stats&action=dailynetutilization&startdate={}&enddate={}&sort={}&apikey={}",base_url,
        start_date, end_date, sort, api_key
    );

    let response = client.get(&url).send().await?;
    let daily_network_utilization_response: DailyNetworkUtilizationResponse =
        response.json().await?;

    Ok(daily_network_utilization_response)
}

pub async fn get_daily_transaction_count(client:&reqwest::Client, base_url: &str,
    api_key: &str,
    start_date: &str,
    end_date: &str,
    sort: &str,
) -> Result<DailyTransactionCountResponse, Error> {
    let url = format!(
        "{}/api?module=stats&action=dailytx&startdate={}&enddate={}&sort={}&apikey={}",base_url,
        start_date, end_date, sort, api_key
    );

    let response = client.get(&url).send().await?;
    let daily_transaction_count_response: DailyTransactionCountResponse = response.json().await?;

    Ok(daily_transaction_count_response)
}
