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
        if !self.table.chars().all(|c| c.is_ascii_alphanumeric() || c == '_') {
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

impl QueryBuilder {
    pub fn new(table: String, filters: Option<Filters>) -> Self {
        Self {
            table,
            filters,
            bind_params: Vec::new(),
        }
    }

    pub fn build_query(&mut self) -> Result<(String, Vec<Value>)> {
        /* 
        ==========================================
        = DANGER: POTENTIAL SQL INJECTION RISK! =
        ==========================================
        */
        // if you are directly embedding table names in queries, 
        // this can make your code VULNERABLE TO SQL INJECTION ATTEMPTS.
        // if you use the DatabaseRequest::validate() function 
        // and provide the table argument only if the validate function passed, 
        // you should have nothing to worry about.
        let mut query = format!("SELECT * FROM {}", self.table);

        let mut bind_index = 1;

        if let Some(filters) = &self.filters {
            println!("Filters: {:?}", filters);
            let (where_clause_sql, bind_values) = filters.build_where_caluse(&mut bind_index)?;
            query.push_str(&where_clause_sql);
            self.bind_params.extend(bind_values);

        }

        Ok((query, self.bind_params.clone()))
    }

}

impl Filters {
    fn sanitize_column_name(column: &str) -> Result<String> {
        let is_valid = column.chars().all(|c| c.is_ascii_alphanumeric() || c == '_');
        if !is_valid {
            return Err(anyhow::anyhow!("Invalid column name: {}", column));
        }
        Ok(column.to_string())
    }

    pub fn build_where_caluse(&self, bind_index: &mut usize) -> Result<(String, Vec<Value>)>{
        let mut conditions = Vec::new();
        let mut bind_values = Vec::new();

        if let Some(where_clause) = &self.where_clause {
            for (column, value) in where_clause {
                let sanitized_column = Self::sanitize_column_name(column)?;

                let condition = format!("{} = ${}", sanitized_column, *bind_index);
                conditions.push(condition);

                bind_values.push(value.clone());  
                *bind_index += 1; 
            }
        }

        if !conditions.is_empty() {
            Ok((format!(" WHERE {}", conditions.join(" AND ")), bind_values))
        } else {
            Ok((String::new(), Vec::new()))  // if no conditions, return an empty WHERE clause
        }

    }
}