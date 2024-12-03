use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct BlockReward {
    #[serde(rename(serialize = "blockNumber", deserialize = "blockNumber"))]
    pub block_number: String,
    #[serde(rename(serialize = "timeStamp", deserialize = "timeStamp"))]
    pub time_stamp: String,
    #[serde(rename(serialize = "blockMiner", deserialize = "blockMiner"))]
    pub block_miner: String,
    #[serde(rename(serialize = "blockReward", deserialize = "blockReward"))]
    pub block_reward: String,
    pub uncles: Vec<String>,
    #[serde(rename(
        serialize = "uncleInclusionReward",
        deserialize = "uncleInclusionReward"
    ))]
    pub uncle_inclusion_reward: String,
}


#[derive(Deserialize, Debug, Clone)]
pub struct BlockCountdown {
    #[serde(rename(serialize = "currentBlock", deserialize = "currentBlock"))]
    pub current_block: String,
    #[serde(rename(serialize = "countdownBlock", deserialize = "countdownBlock"))]
    pub countdown_block: String,
    #[serde(rename(serialize = "remainingBlock", deserialize = "remainingBlock"))]
    pub remaining_block: String,
    #[serde(rename(serialize = "estimateTimeInSec", deserialize = "estimateTimeInSec"))]
    pub estimate_time_in_sec: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct DailyAvgBlockSize {
    #[serde(rename(serialize = "utcDate", deserialize = "utcDate"))]
    pub utc_date: String,
    #[serde(rename(serialize = "unixTimeStamp", deserialize = "unixTimeStamp"))]
    pub unix_time_stamp: String,
    #[serde(rename(serialize = "blockSize_bytes", deserialize = "blockSize_bytes"))]
    pub block_size_bytes: u64,
}

#[derive(Deserialize, Debug, Clone)]
pub struct DailyBlockCount {
    #[serde(rename(serialize = "utcDate", deserialize = "utcDate"))]
    pub utc_date: String,
    #[serde(rename(serialize = "unixTimeStamp", deserialize = "unixTimeStamp"))]
    pub unix_time_stamp: String,
    #[serde(rename(serialize = "blockCount", deserialize = "blockCount"))]
    pub block_count: u64,
    #[serde(rename(serialize = "blockRewards_Eth", deserialize = "blockRewards_Eth"))]
    pub block_rewards_eth: String,
}


#[derive(Deserialize, Debug, Clone)]
pub struct DailyBlockRewards {
    #[serde(rename(serialize = "utcDate", deserialize = "utcDate"))]
    pub utc_date: String,
    #[serde(rename(serialize = "unixTimeStamp", deserialize = "unixTimeStamp"))]
    pub unix_time_stamp: String,
    #[serde(rename(serialize = "blockRewards_Eth", deserialize = "blockRewards_Eth"))]
    pub block_rewards_eth: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct DailyAvgBlockTime {
    #[serde(rename(serialize = "utcDate", deserialize = "utcDate"))]
    pub utc_date: String,
    #[serde(rename(serialize = "unixTimeStamp", deserialize = "unixTimeStamp"))]
    pub unix_time_stamp: String,
    #[serde(rename(serialize = "blockTime_sec", deserialize = "blockTime_sec"))]
    pub block_time_sec: String,
}