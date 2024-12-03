use reqwest::Error;

use crate::types::{
    stats::{
        BnbHistoricalPrice, BnbPriceResult, DailyNetworkUtilization, DailyNewAddress,
        DailyTransactionCount, DailyTransactionFee, Validator,
    },
    ApiResponse,
};

pub async fn get_bnb_supply(
    client: &reqwest::Client,
    base_url: &str,
    api_key: &str,
) -> Result<ApiResponse<String>, Error> {
    let url = format!(
        "{}/api?module=stats&action=bnbsupply&apikey={}",
        base_url, api_key
    );

    let response = client.get(&url).send().await?;
    let bnb_supply_response: ApiResponse<String> = response.json().await?;

    Ok(bnb_supply_response)
}

pub async fn get_validators(
    client: &reqwest::Client,
    base_url: &str,
    api_key: &str,
) -> Result<ApiResponse<Vec<Validator>>, Error> {
    let url = format!(
        "{}/api?module=stats&action=validators&apikey={}",
        base_url, api_key
    );

    let response = client.get(&url).send().await?;
    let validators_response: ApiResponse<Vec<Validator>> = response.json().await?;

    Ok(validators_response)
}

pub async fn get_bnb_price(
    client: &reqwest::Client,
    base_url: &str,
    api_key: &str,
) -> Result<ApiResponse<Vec<BnbPriceResult>>, Error> {
    let url = format!(
        "{}/api?module=stats&action=bnbprice&apikey={}",
        base_url, api_key
    );

    let response = client.get(&url).send().await?;
    let bnb_price_response: ApiResponse<Vec<BnbPriceResult>> = response.json().await?;

    Ok(bnb_price_response)
}

pub async fn get_bnb_historical_price(
    client: &reqwest::Client,
    base_url: &str,
    api_key: &str,
    start_date: &str,
    end_date: &str,
    sort: &str,
) -> Result<ApiResponse<Vec<BnbHistoricalPrice>>, Error> {
    let url = format!(
        "{}/api?module=stats&action=bnbdailyprice&startdate={}&enddate={}&sort={}&apikey={}",
        base_url, start_date, end_date, sort, api_key
    );

    let response = client.get(&url).send().await?;
    let bnb_historical_price_response: ApiResponse<Vec<BnbHistoricalPrice>> =
        response.json().await?;

    Ok(bnb_historical_price_response)
}

pub async fn get_daily_transaction_fee(
    client: &reqwest::Client,
    base_url: &str,
    api_key: &str,
    start_date: &str,
    end_date: &str,
    sort: &str,
) -> Result<ApiResponse<Vec<DailyTransactionFee>>, Error> {
    let url = format!(
        "{}/api?module=stats&action=dailytxnfee&startdate={}&enddate={}&sort={}&apikey={}",
        base_url, start_date, end_date, sort, api_key
    );

    let response = client.get(&url).send().await?;
    let daily_transaction_fee_response: ApiResponse<Vec<DailyTransactionFee>> =
        response.json().await?;

    Ok(daily_transaction_fee_response)
}

pub async fn get_daily_new_address(
    client: &reqwest::Client,
    base_url: &str,
    api_key: &str,
    start_date: &str,
    end_date: &str,
    sort: &str,
) -> Result<ApiResponse<Vec<DailyNewAddress>>, Error> {
    let url = format!(
        "{}/api?module=stats&action=dailynewaddress&startdate={}&enddate={}&sort={}&apikey={}",
        base_url, start_date, end_date, sort, api_key
    );

    let response = client.get(&url).send().await?;
    let daily_new_address_response: ApiResponse<Vec<DailyNewAddress>> = response.json().await?;

    Ok(daily_new_address_response)
}

pub async fn get_daily_network_utilization(
    client: &reqwest::Client,
    base_url: &str,
    api_key: &str,
    start_date: &str,
    end_date: &str,
    sort: &str,
) -> Result<ApiResponse<Vec<DailyNetworkUtilization>>, Error> {
    let url = format!(
        "{}/api?module=stats&action=dailynetutilization&startdate={}&enddate={}&sort={}&apikey={}",
        base_url, start_date, end_date, sort, api_key
    );

    let response = client.get(&url).send().await?;
    let daily_network_utilization_response: ApiResponse<Vec<DailyNetworkUtilization>> =
        response.json().await?;

    Ok(daily_network_utilization_response)
}

pub async fn get_daily_transaction_count(
    client: &reqwest::Client,
    base_url: &str,
    api_key: &str,
    start_date: &str,
    end_date: &str,
    sort: &str,
) -> Result<ApiResponse<Vec<DailyTransactionCount>>, Error> {
    let url = format!(
        "{}/api?module=stats&action=dailytx&startdate={}&enddate={}&sort={}&apikey={}",
        base_url, start_date, end_date, sort, api_key
    );

    let response = client.get(&url).send().await?;
    let daily_transaction_count_response: ApiResponse<Vec<DailyTransactionCount>> =
        response.json().await?;

    Ok(daily_transaction_count_response)
}
