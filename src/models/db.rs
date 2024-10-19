use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

// the actions we perform for the database
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum DatabaseAction {
    Insert,
    Delete,
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

// the filters aka the search conditions
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Filters {
    #[serde(rename = "where")]
    pub where_clause: Option<HashMap<String, Value>>, // use in the request actually "where" instead of "where_clause"
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

pub struct QueryBuilder {
    pub table: String,
    pub filters: Option<Filters>,
    pub bind_params: Vec<Value>
}