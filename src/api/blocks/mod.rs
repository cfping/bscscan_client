use reqwest::Error;

use crate::types::{
    blocks::{
        BlockCountdown, BlockReward, DailyAvgBlockSize, DailyAvgBlockTime, DailyBlockCount,
        DailyBlockRewards,
    },
    ApiResponse,
};

pub async fn get_block_reward(
    client: &reqwest::Client,
    base_url: &str,
    api_key: &str,
    block_number: &str,
) -> Result<BlockReward, Error> {
    let url = format!(
        "{}/api?module=block&action=getblockreward&blockno={}&apikey={}",
        base_url, block_number, api_key
    );

    let response = client.get(&url).send().await?;
    let api_response: ApiResponse<BlockReward> = response.json().await?;

    Ok(api_response.result)
}

pub async fn get_block_countdown(
    client: &reqwest::Client,
    base_url: &str,
    api_key: &str,
    block_number: &str,
) -> Result<BlockCountdown, Error> {
    let url = format!(
        "{}/api?module=block&action=getblockcountdown&blockno={}&apikey={}",
        base_url, block_number, api_key
    );

    let response = client.get(&url).send().await?;
    let api_response: ApiResponse<BlockCountdown> = response.json().await?;

    Ok(api_response.result)
}

pub async fn get_block_number_by_time(
    client: &reqwest::Client,
    base_url: &str,
    api_key: &str,
    timestamp: &str,
    closest: &str,
) -> Result<String, Error> {
    let url = format!(
        "{}/api?module=block&action=getblocknobytime&timestamp={}&closest={}&apikey={}",
        base_url, timestamp, closest, api_key
    );

    let response = client.get(&url).send().await?;
    let api_response: ApiResponse<String> = response.json().await?;

    Ok(api_response.result)
}

pub async fn get_daily_avg_block_size(
    client: &reqwest::Client,
    base_url: &str,
    api_key: &str,
    start_date: &str,
    end_date: &str,
    sort: &str,
) -> Result<Vec<DailyAvgBlockSize>, Error> {
    let url = format!(
        "{}/api?module=stats&action=dailyavgblocksize&startdate={}&enddate={}&sort={}&apikey={}",
        base_url, start_date, end_date, sort, api_key
    );

    let response = client.get(&url).send().await?;
    let api_response: ApiResponse<Vec<DailyAvgBlockSize>> = response.json().await?;

    Ok(api_response.result)
}

pub async fn get_daily_block_count(
    client: &reqwest::Client,
    base_url: &str,
    api_key: &str,
    start_date: &str,
    end_date: &str,
    sort: &str,
) -> Result<Vec<DailyBlockCount>, Error> {
    let url = format!(
        "{}/api?module=stats&action=dailyblkcount&startdate={}&enddate={}&sort={}&apikey={}",
        base_url, start_date, end_date, sort, api_key
    );

    let response = client.get(&url).send().await?;
    let api_response: ApiResponse<Vec<DailyBlockCount>> = response.json().await?;

    Ok(api_response.result)
}

pub async fn get_daily_block_rewards(
    client: &reqwest::Client,
    base_url: &str,
    api_key: &str,
    start_date: &str,
    end_date: &str,
    sort: &str,
) -> Result<Vec<DailyBlockRewards>, Error> {
    let url = format!(
        "{}/api?module=stats&action=dailyblockrewards&startdate={}&enddate={}&sort={}&apikey={}",
        base_url, start_date, end_date, sort, api_key
    );

    let response = client.get(&url).send().await?;
    let api_response: ApiResponse<Vec<DailyBlockRewards>> = response.json().await?;

    Ok(api_response.result)
}

pub async fn get_daily_avg_block_time(
    client: &reqwest::Client,
    base_url: &str,
    api_key: &str,
    start_date: &str,
    end_date: &str,
    sort: &str,
) -> Result<Vec<DailyAvgBlockTime>, Error> {
    let url = format!(
        "{}/api?module=stats&action=dailyavgblocktime&startdate={}&enddate={}&sort={}&apikey={}",
        base_url, start_date, end_date, sort, api_key
    );

    let response = client.get(&url).send().await?;
    let api_response: ApiResponse<Vec<DailyAvgBlockTime>> = response.json().await?;

    Ok(api_response.result)
}
