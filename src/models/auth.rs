use crate::utils::deserializer::custom_deserialize;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct RegisterRequest {
    pub username: String,
    pub password: String,
    pub email: String,
}

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    #[serde(deserialize_with = "custom_deserialize")]
    pub identifier: LoginIdentifier,
    pub password: String,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum LoginIdentifier {
    Username(String),
    Email(String),
}
