use reqwest::Error;

use crate::types::{
    gastracker::{DailyAverageGasLimit, DailyAverageGasPrice, DailyTotalGasUsed, GasOracleResult},
    ApiResponse,
};

pub async fn get_gas_oracle(
    client: &reqwest::Client,
    base_url: &str,
    api_key: &str,
) -> Result<ApiResponse<GasOracleResult>, Error> {
    let url = format!(
        "{}/api?module=gastracker&action=gasoracle&apikey={}",
        base_url, api_key
    );

    let response = client.get(&url).send().await?;
    let gas_oracle_response: ApiResponse<GasOracleResult> = response.json().await?;

    Ok(gas_oracle_response)
}

pub async fn get_daily_average_gas_limit(
    client: &reqwest::Client,
    base_url: &str,
    api_key: &str,
    start_date: &str,
    end_date: &str,
    sort: &str,
) -> Result<ApiResponse<Vec<DailyAverageGasLimit>>, Error> {
    let url = format!(
        "{}/api?module=stats&action=dailyavggaslimit&startdate={}&enddate={}&sort={}&apikey={}",
        base_url, start_date, end_date, sort, api_key
    );

    let response = client.get(&url).send().await?;
    let daily_average_gas_limit_response: ApiResponse<Vec<DailyAverageGasLimit>> =
        response.json().await?;

    Ok(daily_average_gas_limit_response)
}

pub async fn get_daily_total_gas_used(
    client: &reqwest::Client,
    base_url: &str,
    api_key: &str,
    start_date: &str,
    end_date: &str,
    sort: &str,
) -> Result<ApiResponse<Vec<DailyTotalGasUsed>>, Error> {
    let url = format!(
        "{}/api?module=stats&action=dailygasused&startdate={}&enddate={}&sort={}&apikey={}",
        base_url, start_date, end_date, sort, api_key
    );

    let response = client.get(&url).send().await?;
    let daily_total_gas_used_response: ApiResponse<Vec<DailyTotalGasUsed>> =
        response.json().await?;

    Ok(daily_total_gas_used_response)
}

pub async fn get_daily_average_gas_price(
    client: &reqwest::Client,
    base_url: &str,
    api_key: &str,
    start_date: &str,
    end_date: &str,
    sort: &str,
) -> Result<ApiResponse<Vec<DailyAverageGasPrice>>, Error> {
    let url = format!(
        "{}/api?module=stats&action=dailyavggasprice&startdate={}&enddate={}&sort={}&apikey={}",
        base_url, start_date, end_date, sort, api_key
    );

    let response = client.get(&url).send().await?;
    let daily_average_gas_price_response: ApiResponse<Vec<DailyAverageGasPrice>> =
        response.json().await?;

    Ok(daily_average_gas_price_response)
}
