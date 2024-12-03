use serde::Deserialize;

#[derive(Deserialize)]
pub struct Validator {
    #[serde(rename(serialize = "validatorAddress", deserialize = "validatorAddress"))]
    pub validator_address: String,
    #[serde(rename(serialize = "validatorName", deserialize = "validatorName"))]
    pub validator_name: String,
    #[serde(rename(serialize = "validatorStatus", deserialize = "validatorStatus"))]
    pub validator_status: String,
    #[serde(rename(
        serialize = "validatorVotingPower",
        deserialize = "validatorVotingPower"
    ))]
    pub validator_voting_power: String,
    #[serde(rename(
        serialize = "validatorVotingPowerProportion",
        deserialize = "validatorVotingPowerProportion"
    ))]
    pub validator_voting_power_proportion: String,
}

#[derive(Deserialize)]
pub struct BnbPriceResult {
    pub ethbtc: String,
    pub ethbtc_timestamp: String,
    pub ethusd: String,
    pub ethusd_timestamp: String,
}



#[derive(Deserialize)]
pub struct BnbHistoricalPrice {
    #[serde(rename(serialize = "UTCDate", deserialize = "UTCDate"))]
    pub utc_date: String,
    #[serde(rename(serialize = "unixTimeStamp", deserialize = "unixTimeStamp"))]
    pub unix_time_stamp: String,
    pub value: String,
}

#[derive(Deserialize)]
pub struct DailyTransactionFee {
    #[serde(rename(serialize = "UTCDate", deserialize = "UTCDate"))]
    pub utc_date: String,
    #[serde(rename(serialize = "unixTimeStamp", deserialize = "unixTimeStamp"))]
    pub unix_time_stamp: String,
    #[serde(rename(serialize = "transactionFee_Eth", deserialize = "transactionFee_Eth"))]
    pub transaction_fee_eth: String,
}

#[derive(Deserialize)]
pub struct DailyNewAddress {
    #[serde(rename(serialize = "UTCDate", deserialize = "UTCDate"))]
    pub utc_date: String,
    #[serde(rename(serialize = "unixTimeStamp", deserialize = "unixTimeStamp"))]
    pub unix_time_stamp: String,
    #[serde(rename(serialize = "newAddressCount", deserialize = "newAddressCount"))]
    pub new_address_count: u64,
}



#[derive(Deserialize)]
pub struct DailyNetworkUtilization {
    #[serde(rename(serialize = "UTCDate", deserialize = "UTCDate"))]
    pub utc_date: String,
    #[serde(rename(serialize = "unixTimeStamp", deserialize = "unixTimeStamp"))]
    pub unix_time_stamp: String,
    #[serde(rename(serialize = "networkUtilization", deserialize = "networkUtilization"))]
    pub network_utilization: String,
}


#[derive(Deserialize)]
pub struct DailyTransactionCount {
    #[serde(rename(serialize = "UTCDate", deserialize = "UTCDate"))]
    pub utc_date: String,
    #[serde(rename(serialize = "unixTimeStamp", deserialize = "unixTimeStamp"))]
    pub unix_time_stamp: String,
    #[serde(rename(serialize = "transactionCount", deserialize = "transactionCount"))]
    pub transaction_count: u64,
}
