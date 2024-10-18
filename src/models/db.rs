use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

// the actions we perform for the database
#[derive(Debug, Serialize, Deserialize)]
pub enum DatabaseAction {
    Insert,
    Delete,
    Update,
    Retrieve,
}

// the ordering direction
#[derive(Debug, Serialize, Deserialize)]
pub enum OrderDirection {
    Asc,
    Desc,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OrderBy {
    pub column: String,
    pub direction: OrderDirection,
}

// the filters aka the search conditions
#[derive(Debug, Serialize, Deserialize)]
pub struct Filters {
    pub where_clause: Option<HashMap<String, Value>>,
    pub order_by: Option<OrderBy>,
    pub limit: Option<u32>,
    pub offset: Option<u32>,
}

// the request struct itself
#[derive(Debug, Serialize, Deserialize)]
pub struct DatabaseRequest {
    pub table: String,
    pub action: DatabaseAction,
    pub values: Option<HashMap<String, Value>>,
    pub filters: Option<Filters>,
}
