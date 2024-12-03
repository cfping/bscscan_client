use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct LogEntry {
    pub address: String,
    pub topics: Vec<String>,
    pub data: String,
    #[serde(rename(serialize = "blockNumber", deserialize = "blockNumber"))]
    pub block_number: String,
    #[serde(rename(serialize = "blockHash", deserialize = "blockHash"))]
    pub block_hash: String,
    #[serde(rename(serialize = "timeStamp", deserialize = "timeStamp"))]
    pub time_stamp: String,
    #[serde(rename(serialize = "gasPrice", deserialize = "gasPrice"))]
    pub gas_price: String,
    #[serde(rename(serialize = "gasUsed", deserialize = "gasUsed"))]
    pub gas_used: String,
    #[serde(rename(serialize = "logIndex", deserialize = "logIndex"))]
    pub log_index: String,
    #[serde(rename(serialize = "transactionHash", deserialize = "transactionHash"))]
    pub transaction_hash: String,
    #[serde(rename(serialize = "transactionIndex", deserialize = "transactionIndex"))]
    pub transaction_index: String,
}
