use bscscan_client::BscScanClient;

#[tokio::main]
async fn main() {
    let api_key = "YOUR_BSCSCAN_API_KEY";
    let client = BscScanClient::new(api_key.to_string());
    //0x5E9881c4C73E759599687CE5825EDadbbf0C0E26
    let address="0xYourAddress";

    match client
        .get_bnb_balance("address")
        .await
    {
        Ok(balance) => println!("BSC Balance: {}", balance),
        Err(e) => eprintln!("Error: {}", e),
    }
    // 获取普通交易列表
    match client.get_normal_transactions(address,"0","99999999").await {
        Ok(transactions) => {
            for tx in transactions {
                println!("Transaction: {:?}", tx);
            }
        }
        Err(e) => eprintln!("Error: {}", e),
    }

    // 获取内部交易列表
    match client.get_internal_transactions(address,"0","99999999").await {
        Ok(transactions) => {
            for tx in transactions {
                println!("Internal Transaction: {:?}", tx);
            }
        }
        Err(e) => eprintln!("Error: {}", e),
    }
}
