use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::postgres::PgRow;
use sqlx::Row;

use crate::db::TableModel;

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
}
