# BscScanRustClient Usage Documentation


[中文](README-zh.MD)

#### 1. Installation

First, add `bscscan_client` to your `Cargo.toml` file:

```toml
[dependencies]
bscscan_client = "0.1.0"
```

#### 2. Initialize the Client

Before using `bscscan_client`, you need to initialize a client instance. You need to provide your BscScan API key.

```rust
use bscscan_client::BscScanClient;

#[tokio::main]
async fn main() {
    let api_key = "YOUR_BSCSCAN_API_KEY";
    let client = BscScanClient::new(api_key);
}
```

#### 3. Get BNB Balance

You can use the `get_bnb_balance` method to get the BNB balance of a specified address.

```rust
match client.get_bnb_balance("0xYourAddress").await {
    Ok(balance) => println!("BNB Balance: {}", balance),
    Err(e) => eprintln!("Error: {}", e),
}
```

#### 4. Get Normal Transaction List

You can use the `get_normal_transactions` method to get the normal transaction list of a specified address.

```rust
match client.get_normal_transactions("0xYourAddress").await {
    Ok(transactions) => {
        for tx in transactions {
            println!("Transaction: {:?}", tx);
        }
    },
    Err(e) => eprintln!("Error: {}", e),
}
```

#### 5. Get Internal Transaction List

You can use the `get_internal_transactions` method to get the internal transaction list of a specified address.

```rust
match client.get_internal_transactions("0xYourAddress","0","99999999").await {
    Ok(transactions) => {
        for tx in transactions {
            println!("Internal Transaction: {:?}", tx);
        }
    },
    Err(e) => eprintln!("Error: {}", e),
}
```

#### 6. Example Code

Here is a complete example code showing how to use `BscScanRustClient` to get the BNB balance, normal transaction list, and internal transaction list.

```rust
use bscscan_client::BscScanClient;

#[tokio::main]
async fn main() {
    let api_key = "YOUR_BSCSCAN_API_KEY";
    let client = BscScanClient::new(api_key);

    // Get BNB Balance
    match client.get_bnb_balance("0xYourAddress").await {
        Ok(balance) => println!("BNB Balance: {}", balance),
        Err(e) => eprintln!("Error: {}", e),
    }

    // Get Normal Transaction List
    match client.get_normal_transactions("0xYourAddress","0","99999999").await {
        Ok(transactions) => {
            for tx in transactions {
                println!("Transaction: {:?}", tx);
            }
        },
        Err(e) => eprintln!("Error: {}", e),
    }

    // Get Internal Transaction List
    match client.get_internal_transactions("0xYourAddress","0","99999999").await {
        Ok(transactions) => {
            for tx in transactions {
                println!("Internal Transaction: {:?}", tx);
            }
        },
        Err(e) => eprintln!("Error: {}", e),
    }
}
```

### 7. Error Handling

Always handle possible errors when calling API methods. Each method returns a `Result`, and you can use `match` or the `?` operator to handle errors.

```rust
match client.get_bnb_balance("0xYourAddress").await {
    Ok(balance) => println!("BNB Balance: {}", balance),
    Err(e) => eprintln!("Error: {}", e),
}
```

### 8. Contributing

If you find any issues or have improvement suggestions, feel free to submit an Issue or Pull Request.

### 9. License

This project is licensed under the MIT License.

### 10. Disclaimer

This project is for Rust learning and research purposes only.

---

With these steps, you should be able to use the `bscscan_client` library to interact with the BNB community API. We hope this information is helpful!