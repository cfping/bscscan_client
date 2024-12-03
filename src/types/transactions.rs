use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct ContractExecutionStatus {
    #[serde(rename(serialize = "isError", deserialize = "isError"))]
    pub is_error: String,
    #[serde(rename(serialize = "errDescription", deserialize = "errDescription"))]
    pub err_description: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct TransactionReceiptStatus {
    pub status: String,
}