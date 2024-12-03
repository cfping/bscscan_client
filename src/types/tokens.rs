use serde::Deserialize;


#[derive(Deserialize)]
pub struct TokenHolder {
    #[serde(rename(serialize = "TokenHolderAddress", deserialize = "TokenHolderAddress"))]
    pub token_holder_address: String,
    #[serde(rename(serialize = "TokenHolderQuantity", deserialize = "TokenHolderQuantity"))]
    pub token_holder_quantity: String,
}


#[derive(Deserialize)]
pub struct TokenInfo {
    #[serde(rename(serialize = "contractAddress", deserialize = "contractAddress"))]
    pub contract_address: String,
    #[serde(rename(serialize = "tokenName", deserialize = "tokenName"))]
    pub token_name: String,
    pub symbol: String,
    pub divisor: String,
    #[serde(rename(serialize = "tokenType", deserialize = "tokenType"))]
    pub token_type: String,
    #[serde(rename(serialize = "totalSupply", deserialize = "totalSupply"))]
    pub total_supply: String,
    #[serde(rename(serialize = "blueCheckmark", deserialize = "blueCheckmark"))]
    pub blue_check_mark: String,
    pub description: String,
    pub website: String,
    pub email: String,
    pub blog: String,
    pub reddit: String,
    pub slack: String,
    pub facebook: String,
    pub twitter: String,
    #[serde(rename(serialize = "bitcointalk", deserialize = "bitcointalk"))]
    pub bitcoin_talk: String,
    pub github: String,
    pub telegram: String,
    pub wechat: String,
    pub linkedin: String,
    pub discord: String,
    pub whitepaper: String,
    #[serde(rename(serialize = "tokenPriceUSD", deserialize = "tokenPriceUSD"))]
    pub token_price_usd: String,
}



#[derive(Deserialize)]
pub struct AddressTokenBalance {
    #[serde(rename(serialize = "TokenAddress", deserialize = "TokenAddress"))]
    pub token_address: String,
    #[serde(rename(serialize = "TokenName", deserialize = "TokenName"))]
    pub token_name: String,
    #[serde(rename(serialize = "TokenSymbol", deserialize = "TokenSymbol"))]
    pub token_symbol: String,
    #[serde(rename(serialize = "TokenQuantity", deserialize = "TokenQuantity"))]
    pub token_quantity: String,
    #[serde(rename(serialize = "TokenDivisor", deserialize = "TokenDivisor"))]
    pub token_divisor: String,
}



#[derive(Deserialize)]
pub struct AddressNFTBalance {
    #[serde(rename(serialize = "TokenAddress", deserialize = "TokenAddress"))]
    pub token_address: String,
    #[serde(rename(serialize = "TokenName", deserialize = "TokenName"))]
    pub token_name: String,
    #[serde(rename(serialize = "TokenSymbol", deserialize = "TokenSymbol"))]
    pub token_symbol: String,
    #[serde(rename(serialize = "TokenQuantity", deserialize = "TokenQuantity"))]
    pub token_quantity: String,
}


#[derive(Deserialize)]
pub struct AddressNFTInventory {
    #[serde(rename(serialize = "TokenAddress", deserialize = "TokenAddress"))]
    pub token_address: String,
    #[serde(rename(serialize = "TokenId", deserialize = "TokenId"))]
    pub token_id: String,
}
