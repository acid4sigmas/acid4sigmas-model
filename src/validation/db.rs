use crate::{
    models::db::{DatabaseAction, DatabaseRequest, Filters, OrderDirection},
    to_string_,
};
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
    pub fn validate(&self) -> Result<(), String> {
        match self.action {
            DatabaseAction::Insert => {
                if self.values.is_none() || self.values.as_ref().unwrap().is_empty() {
                    return Err(to_string_!("Insert action requires non-empty values."));
                }
            }
            DatabaseAction::Delete => {}
            DatabaseAction::Update => {
                if self.values.is_none() || self.values.as_ref().unwrap().is_empty() {
                    return Err("Update action requires non-empty values.".to_string());
                }
            }
            DatabaseAction::Retrieve => {}
        }
        Ok(())
    }
}
