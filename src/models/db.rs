use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum DeleteAction {
    DeleteTable,
    DeleteValue,
}

// the actions we perform for the database
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum DatabaseAction {
    Insert,
    Delete(DeleteAction),
    Update,
    Retrieve,
}

// the ordering direction
#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub enum OrderDirection {
    Asc,
    Desc,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OrderBy {
    pub column: String,
    pub direction: OrderDirection,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum WhereClause {
    And(HashMap<String, Value>),
    Or(HashMap<String, Value>),
    Single(HashMap<String, Value>),
}
// the filters aka the search conditions
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Filters {
    #[serde(rename = "where")]
    pub where_clause: Option<WhereClause>, // use in the request actually "where" instead of "where_clause"
    pub order_by: Option<OrderBy>,
    pub limit: Option<u32>,
    pub offset: Option<u32>,
}

// the request struct itself
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DatabaseRequest {
    pub table: String,
    pub action: DatabaseAction,
    pub values: Option<HashMap<String, Value>>,
    pub filters: Option<Filters>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum DatabaseResponse<T> {
    Error { error: String },
    Status { status: String },
    Data(Vec<T>),
}

pub struct QueryBuilder {
    pub table: String,
    pub action: DatabaseAction,
    pub filters: Option<Filters>,
    pub values: Option<HashMap<String, Value>>,
    pub table_columns: Option<HashMap<String, String>>,
    pub bind_params: Vec<Value>,
}

impl DatabaseRequest {
    pub fn to_string(&self) -> Result<String, serde_json::Error> {
        match serde_json::to_string(self) {
            Ok(json_string) => Ok(json_string),
            Err(e) => Err(e),
        }
    }
}

impl Default for Filters {
    fn default() -> Self {
        Filters {
            where_clause: None,
            order_by: None,
            limit: None,
            offset: None,
        }
    }
}

impl Default for DatabaseRequest {
    fn default() -> Self {
        DatabaseRequest {
            table: String::new(),
            action: DatabaseAction::default(),
            values: None,
            filters: None,
        }
    }
}

impl Default for DatabaseAction {
    fn default() -> Self {
        DatabaseAction::Retrieve
    }
}
