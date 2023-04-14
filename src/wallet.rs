use serde::{Deserialize, Serialize};
use crate::miner::Miner;

// json payload
#[derive(Debug, Deserialize, Serialize)]
pub struct Wallet {
    pub address: String,
    pub club_name: String,
    pub total_hash_rate: i32,
    pub total_shares_mined: i32,
    pub total_workers_online: i32,
    pub workers_online: Vec<Miner>,
}

// post request body for new miner
#[derive(Debug, Deserialize, Serialize)]
pub struct NewWallet {
    club_name: String,
}

// DAO object
pub struct WalletDAO {
    pub address: String, 
    pub club_name: String,
}