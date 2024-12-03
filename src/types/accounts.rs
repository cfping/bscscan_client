use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Transaction {
    #[serde(rename(serialize = "blockNumber", deserialize = "blockNumber"))]
    pub block_number: String,
    #[serde(rename(serialize = "timeStamp", deserialize = "timeStamp"))]
    pub time_stamp: String,
    pub hash: String,
    pub nonce: String,
    #[serde(rename(serialize = "blockHash", deserialize = "blockHash"))]
    pub block_hash: String,
    #[serde(rename(serialize = "transactionIndex", deserialize = "transactionIndex"))]
    pub transaction_index: String,
    pub from: String,
    pub to: String,
    pub value: String,
    pub gas: String,
    #[serde(rename(serialize = "gasPrice", deserialize = "gasPrice"))]
    pub gas_price: String,
    #[serde(rename(serialize = "isError", deserialize = "isError"))]
    pub is_error: String,
    pub txreceipt_status: String,
    pub input: String,
    #[serde(rename(serialize = "contractAddress", deserialize = "contractAddress"))]
    pub contract_address: String,
    #[serde(rename(serialize = "cumulativeGasUsed", deserialize = "cumulativeGasUsed"))]
    pub cumulative_gas_used: String,
    #[serde(rename(serialize = "gasUsed", deserialize = "gasUsed"))]
    pub gas_used: String,
    pub confirmations: String,
    #[serde(rename(serialize = "methodId", deserialize = "methodId"))]
    pub method_id: String,
    #[serde(rename(serialize = "functionName", deserialize = "functionName"))]
    pub function_name: String,
}

#[derive(Deserialize, Debug)]
pub struct Balance {
    pub account: String,
    pub balance: String,
}
