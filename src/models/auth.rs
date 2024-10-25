use crate::{db::TableModel, utils::deserializer::custom_deserialize};
use acid4sigmas_attr::TableName;
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::postgres::PgRow;
use sqlx::Row;

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
}
