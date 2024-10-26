use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::postgres::PgRow;
use sqlx::Row;
use std::collections::HashMap;

use crate::db::TableModel;
use crate::to_string_;

use acid4sigmas_attr::TableName;

#[derive(Clone, Debug, Serialize, Deserialize, TableName)]
#[table_name = "users"]
pub struct User {
    pub uid: i64,
    pub email: String,
    pub owner: bool,
    pub email_verified: bool,
    pub username: String,
}

impl TableModel for User {
    fn from_row(row: &PgRow) -> Result<Self, sqlx::Error> {
        Ok(User {
            uid: row.try_get("uid")?,
            email: row.try_get("email")?,
            owner: row.try_get("owner")?,
            email_verified: row.try_get("email_verified")?,
            username: row.try_get("username")?,
        })
    }
    fn table_name() -> &'static str {
        let table = User::table_name_();
        table
    }
    fn debug_string(&self) -> String {
        format!(
            "User {{ uid: {}, email: {}, owner: {}, email_verified: {}, username: {} }}",
            self.uid, self.email, self.owner, self.email_verified, self.username
        )
    }
    fn as_value(&self) -> serde_json::Value {
        json!({
            "uid": self.uid,
            "email": self.email,
            "owner": self.owner,
            "email_verified": self.email_verified,
            "username": self.username
        })
    }
    fn as_hash_map(&self) -> HashMap<String, serde_json::Value> {
        let mut hashmap = HashMap::new();
        hashmap.insert(to_string_!("uid"), json!(self.uid));
        hashmap.insert(to_string_!("email"), json!(self.email));
        hashmap.insert(to_string_!("owner"), json!(self.owner));
        hashmap.insert(to_string_!("email_verified"), json!(self.email_verified));
        hashmap.insert(to_string_!("username"), json!(self.username));
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
