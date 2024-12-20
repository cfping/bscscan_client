use crate::{
    api,
    types::{
        accounts::{Balance, Transaction},
        blocks::{
            BlockCountdown, BlockReward, DailyAvgBlockSize, DailyAvgBlockTime, DailyBlockCount,
            DailyBlockRewards,
        },
        contracts::{Contract, ContractCreation},
        gastracker::{
            DailyAverageGasLimit, DailyAverageGasPrice, DailyTotalGasUsed, GasOracleResult,
        },
        logs::LogEntry,
        stats::{
            BnbHistoricalPrice, BnbPriceResult, DailyNetworkUtilization, DailyNewAddress,
            DailyTransactionCount, DailyTransactionFee, Validator,
        },
        tokens::{
            AddressNFTBalance, AddressNFTInventory, AddressTokenBalance, TokenHolder, TokenInfo,
        },
        transactions::{ContractExecutionStatus, TransactionReceiptStatus},
        ApiResponse,
    },
};

pub struct BscScanClient {
    base_url: String,
    api_key: String,
    client: reqwest::Client,
}

impl BscScanClient {
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            client: reqwest::Client::new(),
            base_url: "https://api.bscscan.com".to_owned(),
        }
    }
    pub fn set_client(&mut self, client: reqwest::Client) {
        self.client = client;
    }

    pub async fn get_bnb_balance(&self, address: &str) -> Result<String, reqwest::Error> {
        api::accounts::get_bnb_balance(&self.client, &self.base_url, &self.api_key, address).await
    }

    pub async fn get_bnb_balance_multiple(
        &self,
        addresses: &[&str],
    ) -> Result<Vec<Balance>, reqwest::Error> {
        api::accounts::get_bnb_balance_multiple(
            &self.client,
            &self.base_url,
            &self.api_key,
            addresses,
        )
        .await
    }

    pub async fn get_normal_transactions(
        &self,
        address: &str,
        start_block: &str,
        end_block: &str,
    ) -> Result<Vec<Transaction>, reqwest::Error> {
        api::accounts::get_normal_transactions(
            &self.client,
            &self.base_url,
            &self.api_key,
            address,
            start_block,
            end_block,
        )
        .await
    }

    pub async fn get_internal_transactions(
        &self,
        address: &str,
        start_block: &str,
        end_block: &str,
    ) -> Result<Vec<Transaction>, reqwest::Error> {
        api::accounts::get_internal_transactions(
            &self.client,
            &self.base_url,
            &self.api_key,
            address,
            start_block,
            end_block,
        )
        .await
    }

    pub async fn get_token_transfers(
        &self,
        address: &str,
        start_block: &str,
        end_block: &str,
    ) -> Result<Vec<Transaction>, reqwest::Error> {
        api::accounts::get_token_transfers(
            &self.client,
            &self.base_url,
            &self.api_key,
            address,
            start_block,
            end_block,
        )
        .await
    }
    //合约调用api
    pub async fn get_contract_abi(&self, address: &str) -> Result<String, reqwest::Error> {
        api::contracts::get_contract_abi(&self.client, &self.base_url, &self.api_key, address).await
    }

    pub async fn get_contract_source_code(
        &self,
        address: &str,
    ) -> Result<Vec<Contract>, reqwest::Error> {
        api::contracts::get_contract_source_code(
            &self.client,
            &self.base_url,
            &self.api_key,
            address,
        )
        .await
    }

    pub async fn get_contract_creation(
        &self,
        addresses: &[&str],
    ) -> Result<Vec<ContractCreation>, reqwest::Error> {
        api::contracts::get_contract_creation(
            &self.client,
            &self.base_url,
            &self.api_key,
            addresses,
        )
        .await
    }
    //stats
    //交易
    pub async fn get_contract_execution_status(
        &self,
        txhash: &str,
    ) -> Result<ContractExecutionStatus, reqwest::Error> {
        api::transactions::get_contract_execution_status(
            &self.client,
            &self.base_url,
            &self.api_key,
            txhash,
        )
        .await
    }

    pub async fn get_transaction_receipt_status(
        &self,
        txhash: &str,
    ) -> Result<TransactionReceiptStatus, reqwest::Error> {
        api::transactions::get_transaction_receipt_status(
            &self.client,
            &self.base_url,
            &self.api_key,
            txhash,
        )
        .await
    }
    //blocks
    pub async fn get_block_reward(
        &self,
        block_number: &str,
    ) -> Result<BlockReward, reqwest::Error> {
        api::blocks::get_block_reward(&self.client, &self.base_url, &self.api_key, block_number)
            .await
    }

    pub async fn get_block_countdown(
        &self,
        block_number: &str,
    ) -> Result<BlockCountdown, reqwest::Error> {
        api::blocks::get_block_countdown(&self.client, &self.base_url, &self.api_key, block_number)
            .await
    }

    pub async fn get_block_number_by_time(
        &self,
        timestamp: &str,
        closest: &str,
    ) -> Result<String, reqwest::Error> {
        api::blocks::get_block_number_by_time(
            &self.client,
            &self.base_url,
            &self.api_key,
            timestamp,
            closest,
        )
        .await
    }

    pub async fn get_daily_avg_block_size(
        &self,
        start_date: &str,
        end_date: &str,
        sort: &str,
    ) -> Result<Vec<DailyAvgBlockSize>, reqwest::Error> {
        api::blocks::get_daily_avg_block_size(
            &self.client,
            &self.base_url,
            &self.api_key,
            start_date,
            end_date,
            sort,
        )
        .await
    }

    pub async fn get_daily_block_count(
        &self,
        start_date: &str,
        end_date: &str,
        sort: &str,
    ) -> Result<Vec<DailyBlockCount>, reqwest::Error> {
        api::blocks::get_daily_block_count(
            &self.client,
            &self.base_url,
            &self.api_key,
            start_date,
            end_date,
            sort,
        )
        .await
    }

    pub async fn get_daily_block_rewards(
        &self,
        start_date: &str,
        end_date: &str,
        sort: &str,
    ) -> Result<Vec<DailyBlockRewards>, reqwest::Error> {
        api::blocks::get_daily_block_rewards(
            &self.client,
            &self.base_url,
            &self.api_key,
            start_date,
            end_date,
            sort,
        )
        .await
    }

    pub async fn get_daily_avg_block_time(
        &self,
        start_date: &str,
        end_date: &str,
        sort: &str,
    ) -> Result<Vec<DailyAvgBlockTime>, reqwest::Error> {
        api::blocks::get_daily_avg_block_time(
            &self.client,
            &self.base_url,
            &self.api_key,
            start_date,
            end_date,
            sort,
        )
        .await
    }
    //logs
    pub async fn fetch_logs(
        &self,
        from_block: u64,
        to_block: u64,
        address: &str,
        topic0: &str,
    ) -> Result<ApiResponse<Vec<LogEntry>>, reqwest::Error> {
        api::logs::fetch_logs(
            &self.client,
            &self.base_url,
            &self.api_key,
            from_block,
            to_block,
            address,
            topic0,
        )
        .await
    }
    //geth_parity_proxy
    pub async fn eth_block_number(&self) -> Result<ApiResponse<serde_json::Value>, reqwest::Error> {
        api::geth_parity_proxy::eth_block_number(&self.client, &self.base_url, &self.api_key).await
    }

    pub async fn eth_get_block_by_number(
        &self,
        tag: &str,
        boolean: bool,
    ) -> Result<ApiResponse<serde_json::Value>, reqwest::Error> {
        api::geth_parity_proxy::eth_get_block_by_number(
            &self.client,
            &self.base_url,
            tag,
            boolean,
            &self.api_key,
        )
        .await
    }

    pub async fn eth_get_block_transaction_count_by_number(
        &self,
        tag: &str,
    ) -> Result<ApiResponse<serde_json::Value>, reqwest::Error> {
        api::geth_parity_proxy::eth_get_block_transaction_count_by_number(
            &self.client,
            &self.base_url,
            tag,
            &self.api_key,
        )
        .await
    }

    pub async fn eth_get_transaction_by_hash(
        &self,
        txhash: &str,
    ) -> Result<ApiResponse<serde_json::Value>, reqwest::Error> {
        api::geth_parity_proxy::eth_get_transaction_by_hash(
            &self.client,
            &self.base_url,
            txhash,
            &self.api_key,
        )
        .await
    }

    pub async fn eth_get_transaction_by_block_number_and_index(
        &self,
        tag: &str,
        index: &str,
    ) -> Result<ApiResponse<serde_json::Value>, reqwest::Error> {
        api::geth_parity_proxy::eth_get_transaction_by_block_number_and_index(
            &self.client,
            &self.base_url,
            tag,
            index,
            &self.api_key,
        )
        .await
    }

    pub async fn eth_get_transaction_count(
        &self,
        address: &str,
        tag: &str,
    ) -> Result<ApiResponse<serde_json::Value>, reqwest::Error> {
        api::geth_parity_proxy::eth_get_transaction_count(
            &self.client,
            &self.base_url,
            address,
            tag,
            &self.api_key,
        )
        .await
    }

    pub async fn eth_send_raw_transaction(
        &self,
        hex: &str,
    ) -> Result<ApiResponse<serde_json::Value>, reqwest::Error> {
        api::geth_parity_proxy::eth_send_raw_transaction(
            &self.client,
            &self.base_url,
            hex,
            &self.api_key,
        )
        .await
    }

    pub async fn eth_get_transaction_receipt(
        &self,
        txhash: &str,
    ) -> Result<ApiResponse<serde_json::Value>, reqwest::Error> {
        api::geth_parity_proxy::eth_get_transaction_receipt(
            &self.client,
            &self.base_url,
            txhash,
            &self.api_key,
        )
        .await
    }

    pub async fn eth_call(
        &self,
        to: &str,
        data: &str,
        tag: &str,
    ) -> Result<ApiResponse<serde_json::Value>, reqwest::Error> {
        api::geth_parity_proxy::eth_call(&self.client, &self.base_url, to, data, tag, &self.api_key)
            .await
    }

    pub async fn eth_get_code(
        &self,
        address: &str,
        tag: &str,
    ) -> Result<ApiResponse<serde_json::Value>, reqwest::Error> {
        api::geth_parity_proxy::eth_get_code(
            &self.client,
            &self.base_url,
            address,
            tag,
            &self.api_key,
        )
        .await
    }

    pub async fn eth_get_storage_at(
        &self,
        address: &str,
        position: &str,
        tag: &str,
    ) -> Result<ApiResponse<serde_json::Value>, reqwest::Error> {
        api::geth_parity_proxy::eth_get_storage_at(
            &self.client,
            &self.base_url,
            address,
            position,
            tag,
            &self.api_key,
        )
        .await
    }

    pub async fn eth_gas_price(&self) -> Result<ApiResponse<serde_json::Value>, reqwest::Error> {
        api::geth_parity_proxy::eth_gas_price(&self.client, &self.base_url, &self.api_key).await
    }

    pub async fn eth_estimate_gas(
        &self,
        data: &str,
        to: &str,
        value: &str,
        gas_price: &str,
        gas: &str,
    ) -> Result<ApiResponse<serde_json::Value>, reqwest::Error> {
        api::geth_parity_proxy::eth_estimate_gas(
            &self.client,
            &self.base_url,
            data,
            to,
            value,
            gas_price,
            gas,
            &self.api_key,
        )
        .await
    }
    pub async fn get_bnb_supply(&self) -> Result<ApiResponse<String>, reqwest::Error> {
        api::stats::get_bnb_supply(&self.client, &self.base_url, &self.api_key).await
    }

    pub async fn get_validators(&self) -> Result<ApiResponse<Vec<Validator>>, reqwest::Error> {
        api::stats::get_validators(&self.client, &self.base_url, &self.api_key).await
    }

    pub async fn get_bnb_price(&self) -> Result<ApiResponse<Vec<BnbPriceResult>>, reqwest::Error> {
        api::stats::get_bnb_price(&self.client, &self.base_url, &self.api_key).await
    }

    pub async fn get_bnb_historical_price(
        &self,
        start_date: &str,
        end_date: &str,
        sort: &str,
    ) -> Result<ApiResponse<Vec<BnbHistoricalPrice>>, reqwest::Error> {
        api::stats::get_bnb_historical_price(
            &self.client,
            &self.base_url,
            &self.api_key,
            start_date,
            end_date,
            sort,
        )
        .await
    }

    pub async fn get_daily_transaction_fee(
        &self,
        start_date: &str,
        end_date: &str,
        sort: &str,
    ) -> Result<ApiResponse<Vec<DailyTransactionFee>>, reqwest::Error> {
        api::stats::get_daily_transaction_fee(
            &self.client,
            &self.base_url,
            &self.api_key,
            start_date,
            end_date,
            sort,
        )
        .await
    }

    pub async fn get_daily_new_address(
        &self,
        start_date: &str,
        end_date: &str,
        sort: &str,
    ) -> Result<ApiResponse<Vec<DailyNewAddress>>, reqwest::Error> {
        api::stats::get_daily_new_address(
            &self.client,
            &self.base_url,
            &self.api_key,
            start_date,
            end_date,
            sort,
        )
        .await
    }

    pub async fn get_daily_network_utilization(
        &self,
        start_date: &str,
        end_date: &str,
        sort: &str,
    ) -> Result<ApiResponse<Vec<DailyNetworkUtilization>>, reqwest::Error> {
        api::stats::get_daily_network_utilization(
            &self.client,
            &self.base_url,
            &self.api_key,
            start_date,
            end_date,
            sort,
        )
        .await
    }

    pub async fn get_daily_transaction_count(
        &self,
        start_date: &str,
        end_date: &str,
        sort: &str,
    ) -> Result<ApiResponse<Vec<DailyTransactionCount>>, reqwest::Error> {
        api::stats::get_daily_transaction_count(
            &self.client,
            &self.base_url,
            &self.api_key,
            start_date,
            end_date,
            sort,
        )
        .await
    }

    pub async fn get_gas_oracle(&self) -> Result<ApiResponse<GasOracleResult>, reqwest::Error> {
        api::gastracker::get_gas_oracle(&self.client, &self.base_url, &self.api_key).await
    }

    pub async fn get_daily_average_gas_limit(
        &self,
        start_date: &str,
        end_date: &str,
        sort: &str,
    ) -> Result<ApiResponse<Vec<DailyAverageGasLimit>>, reqwest::Error> {
        api::gastracker::get_daily_average_gas_limit(
            &self.client,
            &self.base_url,
            &self.api_key,
            start_date,
            end_date,
            sort,
        )
        .await
    }

    pub async fn get_daily_total_gas_used(
        &self,
        start_date: &str,
        end_date: &str,
        sort: &str,
    ) -> Result<ApiResponse<Vec<DailyTotalGasUsed>>, reqwest::Error> {
        api::gastracker::get_daily_total_gas_used(
            &self.client,
            &self.base_url,
            &self.api_key,
            start_date,
            end_date,
            sort,
        )
        .await
    }

    pub async fn get_daily_average_gas_price(
        &self,
        start_date: &str,
        end_date: &str,
        sort: &str,
    ) -> Result<ApiResponse<Vec<DailyAverageGasPrice>>, reqwest::Error> {
        api::gastracker::get_daily_average_gas_price(
            &self.client,
            &self.base_url,
            &self.api_key,
            start_date,
            end_date,
            sort,
        )
        .await
    }

    pub async fn get_bep20_token_circulating_supply(
        &self,
        contract_address: &str,
    ) -> Result<ApiResponse<String>, reqwest::Error> {
        api::tokens::get_bep20_token_circulating_supply(
            &self.client,
            &self.base_url,
            &self.api_key,
            contract_address,
        )
        .await
    }

    pub async fn get_bep20_token_total_supply(
        &self,
        contract_address: &str,
    ) -> Result<ApiResponse<String>, reqwest::Error> {
        api::tokens::get_bep20_token_total_supply(
            &self.client,
            &self.base_url,
            &self.api_key,
            contract_address,
        )
        .await
    }

    pub async fn get_bep20_token_account_balance(
        &self,
        contract_address: &str,
        address: &str,
    ) -> Result<ApiResponse<String>, reqwest::Error> {
        api::tokens::get_bep20_token_account_balance(
            &self.client,
            &self.base_url,
            &self.api_key,
            contract_address,
            address,
        )
        .await
    }

    pub async fn get_historical_bep20_token_total_supply(
        &self,
        contract_address: &str,
        block_no: u64,
    ) -> Result<ApiResponse<String>, reqwest::Error> {
        api::tokens::get_historical_bep20_token_total_supply(
            &self.client,
            &self.base_url,
            &self.api_key,
            contract_address,
            block_no,
        )
        .await
    }

    pub async fn get_historical_bep20_token_account_balance(
        &self,
        contract_address: &str,
        address: &str,
        block_no: u64,
    ) -> Result<ApiResponse<String>, reqwest::Error> {
        api::tokens::get_historical_bep20_token_account_balance(
            &self.client,
            &self.base_url,
            &self.api_key,
            contract_address,
            address,
            block_no,
        )
        .await
    }

    pub async fn get_token_holder_list(
        &self,
        contract_address: &str,
        page: u64,
        offset: u64,
    ) -> Result<ApiResponse<Vec<TokenHolder>>, reqwest::Error> {
        api::tokens::get_token_holder_list(
            &self.client,
            &self.base_url,
            &self.api_key,
            contract_address,
            page,
            offset,
        )
        .await
    }

    pub async fn get_token_info(
        &self,
        contract_address: &str,
    ) -> Result<ApiResponse<Vec<TokenInfo>>, reqwest::Error> {
        api::tokens::get_token_info(
            &self.client,
            &self.base_url,
            &self.api_key,
            contract_address,
        )
        .await
    }

    pub async fn get_address_bep20_token_holding(
        &self,
        address: &str,
        page: u64,
        offset: u64,
    ) -> Result<ApiResponse<Vec<AddressTokenBalance>>, reqwest::Error> {
        api::tokens::get_address_bep20_token_holding(
            &self.client,
            &self.base_url,
            &self.api_key,
            address,
            page,
            offset,
        )
        .await
    }

    pub async fn get_address_bep721_token_holding(
        &self,
        address: &str,
        page: u64,
        offset: u64,
    ) -> Result<ApiResponse<Vec<AddressNFTBalance>>, reqwest::Error> {
        api::tokens::get_address_bep721_token_holding(
            &self.client,
            &self.base_url,
            &self.api_key,
            address,
            page,
            offset,
        )
        .await
    }

    pub async fn get_address_bep721_token_inventory(
        &self,
        address: &str,
        contract_address: &str,
        page: u64,
        offset: u64,
    ) -> Result<ApiResponse<Vec<AddressNFTInventory>>, reqwest::Error> {
        api::tokens::get_address_bep721_token_inventory(
            &self.client,
            &self.base_url,
            &self.api_key,
            address,
            contract_address,
            page,
            offset,
        )
        .await
    }
}
