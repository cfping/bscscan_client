use reqwest::Error;
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct Contract {
    pub contractAddress: String,
    pub contractName: String,
    pub compilerVersion: String,
    pub optimizationUsed: String,
    pub runs: String,
    pub sourceCode: String,
    pub abi: String,
    pub licenseType: String,
}

#[derive(Deserialize, Debug)]
pub struct AbiResponse {
    pub status: String,
    pub message: String,
    pub result: String,
}

#[derive(Deserialize, Debug)]
pub struct SourceCodeResponse {
    pub status: String,
    pub message: String,
    pub result: Vec<Contract>,
}

#[derive(Deserialize, Debug)]
pub struct ContractCreationResponse {
    pub status: String,
    pub message: String,
    pub result: Vec<ContractCreation>,
}

#[derive(Deserialize, Debug)]
pub struct ContractCreation {
    pub contractAddress: String,
    pub contractCreator: String,
    pub txHash: String,
}

pub async fn get_contract_abi(client:&reqwest::Client, base_url: &str,api_key: &str, address: &str) -> Result<String, Error> {
    let url = format!(
        "{}/api?module=contract&action=getabi&address={}&apikey={}",base_url,
        address, api_key
    );

    let response = client.get(&url).send().await?;
    let api_response: AbiResponse = response.json().await?;

    Ok(api_response.result)
}

pub async fn get_contract_source_code(client:&reqwest::Client, base_url: &str,api_key: &str, address: &str) -> Result<Vec<Contract>, Error> {
    let url = format!(
        "{}/api?module=contract&action=getsourcecode&address={}&apikey={}",base_url,
        address, api_key
    );

    let response = client.get(&url).send().await?;
    let api_response: SourceCodeResponse = response.json().await?;

    Ok(api_response.result)
}

pub async fn get_contract_creation(client:&reqwest::Client, base_url: &str,api_key: &str, addresses: &[&str]) -> Result<Vec<ContractCreation>, Error> {
    let addresses_str = addresses.join(",");
    let url = format!(
        "{}/api?module=contract&action=getcontractcreation&contractaddresses={}&apikey={}",base_url,
        addresses_str, api_key
    );

    let response = client.get(&url).send().await?;
    let api_response: ContractCreationResponse = response.json().await?;

    Ok(api_response.result)
}