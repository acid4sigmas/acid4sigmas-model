use super::{auth::TwoFaType, redis::RedisClient};
use serde::{Deserialize, Serialize};

pub struct TotpGen {
    pub secret: String,
}

pub struct TotpStorage {
    pub totp: Option<String>,
    pub redis_client: RedisClient,
    pub redis_key: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum TotpRedisAction {
    Insert,
    Delete,
    Retrieve,
}

pub struct TotpRequest {
    twofa_type: TwoFaType,
    action: TotpRedisAction,
}
