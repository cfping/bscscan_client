# bscscan_client 使用文档

#### 1. 安装

首先，将 `bscscan_client` 添加到你的 `Cargo.toml` 文件中：

```toml
[dependencies]
bscscan_client = "0.1.0"
```

#### 2. 初始化客户端

在使用 `bscscan_client` 之前，你需要初始化一个客户端实例。你需要提供你的 BscScan API 密钥。

```rust
use bscscan_client::BscScanClient;

#[tokio::main]
async fn main() {
    let api_key = "YOUR_BSCSCAN_API_KEY";
    let client = BscScanClient::new(api_key);
}
```

#### 3. 获取 BNB 余额

你可以使用 `get_bnb_balance` 方法来获取指定地址的 BNB 余额。

```rust
match client.get_bnb_balance("0xYourAddress").await {
    Ok(balance) => println!("BNB Balance: {}", balance),
    Err(e) => eprintln!("Error: {}", e),
}
```

#### 4. 获取普通交易列表

你可以使用 `get_normal_transactions` 方法来获取指定地址的普通交易列表。

```rust
match client.get_normal_transactions("0xYourAddress","0","99999999").await {
    Ok(transactions) => {
        for tx in transactions {
            println!("Transaction: {:?}", tx);
        }
    },
    Err(e) => eprintln!("Error: {}", e),
}
```

#### 5. 获取内部交易列表

你可以使用 `get_internal_transactions` 方法来获取指定地址的内部交易列表。

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

#### 6. 示例代码

以下是一个完整的示例代码，展示了如何使用 `BscScanRustClient` 获取 BNB 余额、普通交易列表和内部交易列表。

```rust
use bscscan_client::BscScanClient;

#[tokio::main]
async fn main() {
    let api_key = "YOUR_BSCSCAN_API_KEY";
    let client = BscScanClient::new(api_key);

    // 获取 BNB 余额
    match client.get_bnb_balance("0xYourAddress").await {
        Ok(balance) => println!("BNB Balance: {}", balance),
        Err(e) => eprintln!("Error: {}", e),
    }

    // 获取普通交易列表
    match client.get_normal_transactions("0xYourAddress","0","99999999").await {
        Ok(transactions) => {
            for tx in transactions {
                println!("Transaction: {:?}", tx);
            }
        },
        Err(e) => eprintln!("Error: {}", e),
    }

    // 获取内部交易列表
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

### 7. 错误处理

在调用 API 方法时，始终处理可能的错误。每个方法都会返回一个 `Result`，你可以使用 `match` 或 `?` 操作符来处理错误。

```rust
match client.get_bnb_balance("0xYourAddress").await {
    Ok(balance) => println!("BNB Balance: {}", balance),
    Err(e) => eprintln!("Error: {}", e),
}
```

### 8. 贡献

如果你发现任何问题或有改进建议，欢迎提交 Issue 或 Pull Request。

### 9. 许可证

本项目采用 MIT 许可证。

### 10. 声明

本项目仅供 Rust 学习研究使用。

---

通过这些步骤，你应该能够使用 `bscscan_client` 库与 BNB 社区 API 进行交互。希望这些信息对你有所帮助！