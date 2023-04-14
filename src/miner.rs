use serde::{Deserialize, Serialize};

// json payload
#[derive(Debug, Deserialize, Serialize)]
pub struct Miner {
    pub id: String,
    pub address: String,
    pub club_name: String,
    pub hash_rate: i32, // MH/s
    pub shares_mined: i32,
}

// post request for new miner
#[derive(Debug, Deserialize, Serialize)]
pub struct NewMiner {
    nickname: String,
}

// database object: DAO (database access object)
pub struct MinerDAO {
    pub id: String,
    pub address: String,
    pub nickname: String,
    pub hash_rate: i32,
    pub shares_mined: i32,
}
