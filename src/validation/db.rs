use crate::{
    models::db::{DatabaseAction, DatabaseRequest, Filters, OrderDirection, QueryBuilder},
    to_string_,
};
use anyhow::Result;
use serde_json::Value;
use std::str::FromStr;

impl FromStr for DatabaseAction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "insert" => Ok(DatabaseAction::Insert),
            "delete" => Ok(DatabaseAction::Delete),
            "update" => Ok(DatabaseAction::Update),
            _ => Err(format!("invalid database_action type: {}", s)),
        }
    }
}

impl FromStr for OrderDirection {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "asc" => Ok(OrderDirection::Asc),
            "desc" => Ok(OrderDirection::Desc),
            _ => Err(format!("invalid order_direction type: {}", s)),
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

impl DatabaseRequest {
    pub fn validate(&mut self) -> Result<(), String> {
        // eleminate spaces
        self.table = self.table.split_whitespace().collect::<String>();

        // check if all characters are either a-z 0-9 or contain a '_'
        if !self
            .table
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '_')
        {
            return Err(to_string_!("Table name contains invalid characters. Only alphanumeric characters and underscores are allowed."));
        }

        // conver to lowercase chars
        self.table = self.table.to_ascii_lowercase();

        match self.action {
            DatabaseAction::Insert => {
                if self.values.is_none() || self.values.as_ref().unwrap().is_empty() {
                    return Err(to_string_!("Insert action requires non-empty values."));
                }
            }
            DatabaseAction::Delete => {}
            DatabaseAction::Update => {
                if self.values.is_none() || self.values.as_ref().unwrap().is_empty() {
                    return Err(to_string_!("Update action requires non-empty values."));
                }
            }
            DatabaseAction::Retrieve => {}
        }

        Ok(())
    }
}
