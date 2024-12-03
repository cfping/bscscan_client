use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct Contract {
    #[serde(rename(serialize = "contractAddress", deserialize = "contractAddress"))]
    pub contract_address: String,
    #[serde(rename(serialize = "contractName", deserialize = "contractName"))]
    pub contract_name: String,
    #[serde(rename(serialize = "compilerVersion", deserialize = "compilerVersion"))]
    pub compiler_version: String,
    #[serde(rename(serialize = "optimizationUsed", deserialize = "optimizationUsed"))]
    pub optimization_used: String,
    pub runs: String,
    #[serde(rename(serialize = "sourceCode", deserialize = "sourceCode"))]
    pub source_code: String,
    pub abi: String,
    #[serde(rename(serialize = "licenseType", deserialize = "licenseType"))]
    pub license_type: String,
}

#[derive(Deserialize, Debug)]
pub struct ContractCreation {
    #[serde(rename(serialize = "contractAddress", deserialize = "contractAddress"))]
    pub contract_address: String,
    #[serde(rename(serialize = "contractCreator", deserialize = "contractCreator"))]
    pub contract_creator: String,
    #[serde(rename(serialize = "txHash", deserialize = "txHash"))]
    pub tx_hash: String,
}
