pub mod accounts;
pub mod blocks;
pub mod contracts;
pub mod gastracker;
pub mod logs;
pub mod stats;
pub mod tokens;
pub mod transactions;

use serde::{Deserialize, Serialize};
#[derive(Deserialize, Serialize, Debug)]
pub struct ApiResponse<T> {
    pub status: String,
    pub message: String,
    pub result: T,
}
