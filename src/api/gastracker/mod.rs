use reqwest::Error;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct GasOracleResponse {
    status: String,
    message: String,
    result: GasOracleResult,
}

#[derive(Deserialize)]
struct GasOracleResult {
    LastBlock: String,
    SafeGasPrice: String,
    ProposeGasPrice: String,
    FastGasPrice: String,
    UsdPrice: String,
}

#[derive(Deserialize)]
pub struct DailyAverageGasLimitResponse {
    status: String,
    message: String,
    result: Vec<DailyAverageGasLimit>,
}

#[derive(Deserialize)]
struct DailyAverageGasLimit {
    UTCDate: String,
    unixTimeStamp: String,
    gasLimit: String,
}

#[derive(Deserialize)]
pub struct DailyTotalGasUsedResponse {
    status: String,
    message: String,
    result: Vec<DailyTotalGasUsed>,
}

#[derive(Deserialize)]
struct DailyTotalGasUsed {
    UTCDate: String,
    unixTimeStamp: String,
    gasUsed: String,
}

#[derive(Deserialize)]
pub struct DailyAverageGasPriceResponse {
    status: String,
    message: String,
    result: Vec<DailyAverageGasPrice>,
}

#[derive(Deserialize)]
struct DailyAverageGasPrice {
    UTCDate: String,
    unixTimeStamp: String,
    maxGasPrice_Wei: String,
    minGasPrice_Wei: String,
    avgGasPrice_Wei: String,
}

pub async fn get_gas_oracle(client:&reqwest::Client, base_url: &str,api_key: &str) -> Result<GasOracleResponse, Error> {
    let url = format!(
        "{}/api?module=gastracker&action=gasoracle&apikey={}",base_url,
        api_key
    );

    let response = client.get(&url).send().await?;
    let gas_oracle_response: GasOracleResponse = response.json().await?;

    Ok(gas_oracle_response)
}

pub async fn get_daily_average_gas_limit(client:&reqwest::Client, base_url: &str,
    api_key: &str,
    start_date: &str,
    end_date: &str,
    sort: &str,
) -> Result<DailyAverageGasLimitResponse, Error> {
    let url = format!(
        "{}/api?module=stats&action=dailyavggaslimit&startdate={}&enddate={}&sort={}&apikey={}",base_url,
        start_date, end_date, sort, api_key
    );

    let response = client.get(&url).send().await?;
    let daily_average_gas_limit_response: DailyAverageGasLimitResponse = response.json().await?;

    Ok(daily_average_gas_limit_response)
}

pub async fn get_daily_total_gas_used(client:&reqwest::Client, base_url: &str,
    api_key: &str,
    start_date: &str,
    end_date: &str,
    sort: &str,
) -> Result<DailyTotalGasUsedResponse, Error> {
    let url = format!(
        "{}/api?module=stats&action=dailygasused&startdate={}&enddate={}&sort={}&apikey={}",base_url,
        start_date, end_date, sort, api_key
    );

    let response = client.get(&url).send().await?;
    let daily_total_gas_used_response: DailyTotalGasUsedResponse = response.json().await?;

    Ok(daily_total_gas_used_response)
}

pub async fn get_daily_average_gas_price(client:&reqwest::Client, base_url: &str,
    api_key: &str,
    start_date: &str,
    end_date: &str,
    sort: &str,
) -> Result<DailyAverageGasPriceResponse, Error> {
    let url = format!(
        "{}/api?module=stats&action=dailyavggasprice&startdate={}&enddate={}&sort={}&apikey={}",base_url,
        start_date, end_date, sort, api_key
    );

    let response = client.get(&url).send().await?;
    let daily_average_gas_price_response: DailyAverageGasPriceResponse = response.json().await?;

    Ok(daily_average_gas_price_response)
}