use serde::Deserialize;


#[derive(Deserialize)]
pub struct GasOracleResult {
    #[serde(rename(serialize = "LastBlock", deserialize = "LastBlock"))]
    pub last_block: String,
    #[serde(rename(serialize = "SafeGasPrice", deserialize = "SafeGasPrice"))]
    pub safe_gas_price: String,
    #[serde(rename(serialize = "ProposeGasPrice", deserialize = "ProposeGasPrice"))]
    pub propose_gas_price: String,
    #[serde(rename(serialize = "FastGasPrice", deserialize = "FastGasPrice"))]
    pub fast_gas_price: String,
    #[serde(rename(serialize = "UsdPrice", deserialize = "UsdPrice"))]
    pub usd_price: String,
}

#[derive(Deserialize)]
pub struct DailyAverageGasLimit {
    #[serde(rename(serialize = "UTCDate", deserialize = "UTCDate"))]
    pub utc_date: String,
    #[serde(rename(serialize = "unixTimeStamp", deserialize = "unixTimeStamp"))]
    pub unix_time_stamp: String,
    #[serde(rename(serialize = "gasLimit", deserialize = "gasLimit"))]
    pub gas_limit: String,
}


#[derive(Deserialize)]
pub struct DailyTotalGasUsed {
    #[serde(rename(serialize = "UTCDate", deserialize = "UTCDate"))]
    pub utc_date: String,
    #[serde(rename(serialize = "unixTimeStamp", deserialize = "unixTimeStamp"))]
    pub unix_time_stamp: String,
    #[serde(rename(serialize = "gasUsed", deserialize = "gasUsed"))]
    pub gas_used: String,
}

#[derive(Deserialize)]
pub struct DailyAverageGasPrice {
    #[serde(rename(serialize = "UTCDate", deserialize = "UTCDate"))]
    pub utc_date: String,
    #[serde(rename(serialize = "unixTimeStamp", deserialize = "unixTimeStamp"))]
    pub unix_time_stamp: String,
    #[serde(rename(serialize = "maxGasPrice_Wei", deserialize = "maxGasPrice_Wei"))]
    pub max_gas_price_wei: String,
    #[serde(rename(serialize = "minGasPrice_Wei", deserialize = "minGasPrice_Wei"))]
    pub min_gas_price_wei: String,
    #[serde(rename(serialize = "avgGasPrice_Wei", deserialize = "avgGasPrice_Wei"))]
    pub avg_gas_price_wei: String,
}
