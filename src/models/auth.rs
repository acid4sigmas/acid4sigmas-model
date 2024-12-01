use crate::to_string_;
use crate::{db::TableModel, utils::deserializer::custom_deserialize};
use acid4sigmas_attr::TableName;
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::postgres::PgRow;
use sqlx::Row;
use std::collections::HashMap;

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

#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
pub enum TwoFaType {
    Password,
    Email,
}

#[derive(Clone, Debug, Serialize, Deserialize, TableName)]
#[table_name = "auth_users"]
pub struct AuthUser {
    pub uid: i64,
    pub email: String,
    pub email_verified: bool,
    pub username: String,
    pub password_hash: String,
}

impl TableModel for AuthUser {
    fn from_row(row: &PgRow) -> Result<Self, sqlx::Error> {
        Ok(AuthUser {
            uid: row.try_get("uid")?,
            email: row.try_get("email")?,
            email_verified: row.try_get("email_verified")?,
            username: row.try_get("username")?,
            password_hash: row.try_get("password_hash")?,
        })
    }
    fn table_name() -> &'static str {
        let table = AuthUser::table_name_();
        table
    }
    fn debug_string(&self) -> String {
        format!(
            "AuthUser {{ uid: {}, email: {}, email_verified: {}, username: {}, password_hash: {} }}",
            self.uid, self.email, self.email_verified, self.username, self.password_hash
        )
    }
    fn as_value(&self) -> serde_json::Value {
        json!({
            "uid": self.uid,
            "email": self.email,
            "email_verified": self.email_verified,
            "username": self.username,
            "password_hash": self.password_hash
        })
    }
    fn as_hash_map(&self) -> HashMap<String, serde_json::Value> {
        let mut hashmap = HashMap::new();
        hashmap.insert(to_string_!("uid"), json!(self.uid));
        hashmap.insert(to_string_!("email"), json!(self.email));
        hashmap.insert(to_string_!("email_verified"), json!(self.email_verified));
        hashmap.insert(to_string_!("username"), json!(self.username));
        hashmap.insert(to_string_!("password_hash"), json!(self.password_hash));
        hashmap
    }
    fn get_keys_as_hashmap(&self, keys: Vec<&str>) -> HashMap<String, serde_json::Value> {
        let map = self.as_hash_map();
        let mut hashmap = HashMap::new();

        for key in keys {
            if let Some(value) = map.get(key) {
                hashmap.insert(key.to_string(), value.clone());
            }
        }

        hashmap
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, TableName)]
#[table_name = "auth_tokens"]
pub struct AuthTokens {
    pub jti: String,
    pub uid: i64,
    pub expires_at: i64,
}

impl TableModel for AuthTokens {
    fn from_row(row: &PgRow) -> Result<Self, sqlx::Error> {
        Ok(AuthTokens {
            jti: row.try_get("jti")?,
            uid: row.try_get("uid")?,
            expires_at: row.try_get("expires_at")?,
        })
    }
    fn table_name() -> &'static str {
        let table = AuthTokens::table_name_();
        table
    }
    fn debug_string(&self) -> String {
        format!(
            "AuthToken {{ uid: {}, jti: {}, expires_at: {} }}",
            self.uid, self.jti, self.expires_at
        )
    }
    fn as_value(&self) -> serde_json::Value {
        json!({
            "uid": self.uid,
            "jti": self.jti,
            "expires_at": self.expires_at
        })
    }
    fn as_hash_map(&self) -> HashMap<String, serde_json::Value> {
        let mut hashmap = HashMap::new();
        hashmap.insert(to_string_!("uid"), json!(self.uid));
        hashmap.insert(to_string_!("jti"), json!(self.jti));
        hashmap.insert(to_string_!("expires_at"), json!(self.expires_at));
        hashmap
    }
    fn get_keys_as_hashmap(&self, keys: Vec<&str>) -> HashMap<String, serde_json::Value> {
        let map = self.as_hash_map();
        let mut hashmap = HashMap::new();

        for key in keys {
            if let Some(value) = map.get(key) {
                hashmap.insert(key.to_string(), value.clone());
            }
        }

        hashmap
    }
}
