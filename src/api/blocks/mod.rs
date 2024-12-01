use reqwest::Error;
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct BlockReward {
    pub blockNumber: String,
    pub timeStamp: String,
    pub blockMiner: String,
    pub blockReward: String,
    pub uncles: Vec<String>,
    pub uncleInclusionReward: String,
}

#[derive(Deserialize, Debug)]
pub struct BlockRewardResponse {
    pub status: String,
    pub message: String,
    pub result: BlockReward,
}

#[derive(Deserialize, Debug, Clone)]
pub struct BlockCountdown {
    pub currentBlock: String,
    pub countdownBlock: String,
    pub remainingBlock: String,
    pub estimateTimeInSec: String,
}

#[derive(Deserialize, Debug)]
pub struct BlockCountdownResponse {
    pub status: String,
    pub message: String,
    pub result: BlockCountdown,
}

#[derive(Deserialize, Debug)]
pub struct BlockNumberResponse {
    pub status: String,
    pub message: String,
    pub result: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct DailyAvgBlockSize {
    pub utcDate: String,
    pub unixTimeStamp: String,
    pub blockSize_bytes: u64,
}

#[derive(Deserialize, Debug)]
pub struct DailyAvgBlockSizeResponse {
    pub status: String,
    pub message: String,
    pub result: Vec<DailyAvgBlockSize>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct DailyBlockCount {
    pub utcDate: String,
    pub unixTimeStamp: String,
    pub blockCount: u64,
    pub blockRewards_Eth: String,
}

#[derive(Deserialize, Debug)]
pub struct DailyBlockCountResponse {
    pub status: String,
    pub message: String,
    pub result: Vec<DailyBlockCount>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct DailyBlockRewards {
    pub utcDate: String,
    pub unixTimeStamp: String,
    pub blockRewards_Eth: String,
}

#[derive(Deserialize, Debug)]
pub struct DailyBlockRewardsResponse {
    pub status: String,
    pub message: String,
    pub result: Vec<DailyBlockRewards>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct DailyAvgBlockTime {
    pub utcDate: String,
    pub unixTimeStamp: String,
    pub blockTime_sec: String,
}

#[derive(Deserialize, Debug)]
pub struct DailyAvgBlockTimeResponse {
    pub status: String,
    pub message: String,
    pub result: Vec<DailyAvgBlockTime>,
}

pub async fn get_block_reward(client:&reqwest::Client, base_url: &str,api_key: &str, block_number: &str) -> Result<BlockReward, Error> {
    let url = format!(
        "{}/api?module=block&action=getblockreward&blockno={}&apikey={}",base_url,
        block_number, api_key
    );

    let response = client.get(&url).send().await?;
    let api_response: BlockRewardResponse = response.json().await?;

    Ok(api_response.result)
}

pub async fn get_block_countdown(client:&reqwest::Client, base_url: &str,api_key: &str, block_number: &str) -> Result<BlockCountdown, Error> {
    let url = format!(
        "{}/api?module=block&action=getblockcountdown&blockno={}&apikey={}",base_url,
        block_number, api_key
    );

    let response = client.get(&url).send().await?;
    let api_response: BlockCountdownResponse = response.json().await?;

    Ok(api_response.result)
}

pub async fn get_block_number_by_time(client:&reqwest::Client, base_url: &str,api_key: &str, timestamp: &str, closest: &str) -> Result<String, Error> {
    let url = format!(
        "{}/api?module=block&action=getblocknobytime&timestamp={}&closest={}&apikey={}",base_url,
        timestamp, closest, api_key
    );

    let response = client.get(&url).send().await?;
    let api_response: BlockNumberResponse = response.json().await?;

    Ok(api_response.result)
}

pub async fn get_daily_avg_block_size(client:&reqwest::Client, base_url: &str,api_key: &str, start_date: &str, end_date: &str, sort: &str) -> Result<Vec<DailyAvgBlockSize>, Error> {
    let url = format!(
        "{}/api?module=stats&action=dailyavgblocksize&startdate={}&enddate={}&sort={}&apikey={}",base_url,
        start_date, end_date, sort, api_key
    );

    let response = client.get(&url).send().await?;
    let api_response: DailyAvgBlockSizeResponse = response.json().await?;

    Ok(api_response.result)
}

pub async fn get_daily_block_count(client:&reqwest::Client, base_url: &str,api_key: &str, start_date: &str, end_date: &str, sort: &str) -> Result<Vec<DailyBlockCount>, Error> {
    let url = format!(
        "{}/api?module=stats&action=dailyblkcount&startdate={}&enddate={}&sort={}&apikey={}",base_url,
        start_date, end_date, sort, api_key
    );

    let response = client.get(&url).send().await?;
    let api_response: DailyBlockCountResponse = response.json().await?;

    Ok(api_response.result)
}

pub async fn get_daily_block_rewards(client:&reqwest::Client, base_url: &str,api_key: &str, start_date: &str, end_date: &str, sort: &str) -> Result<Vec<DailyBlockRewards>, Error> {
    let url = format!(
        "{}/api?module=stats&action=dailyblockrewards&startdate={}&enddate={}&sort={}&apikey={}",base_url,
        start_date, end_date, sort, api_key
    );

    let response = client.get(&url).send().await?;
    let api_response: DailyBlockRewardsResponse = response.json().await?;

    Ok(api_response.result)
}

pub async fn get_daily_avg_block_time(client:&reqwest::Client, base_url: &str,api_key: &str, start_date: &str, end_date: &str, sort: &str) -> Result<Vec<DailyAvgBlockTime>, Error> {
    let url = format!(
        "{}/api?module=stats&action=dailyavgblocktime&startdate={}&enddate={}&sort={}&apikey={}",base_url,
        start_date, end_date, sort, api_key
    );

    let response = client.get(&url).send().await?;
    let api_response: DailyAvgBlockTimeResponse = response.json().await?;

    Ok(api_response.result)
}