use crate::models::db::{DatabaseAction, Filters, OrderDirection, QueryBuilder, WhereClause};
use anyhow::{anyhow, Result};
use serde_json::Value;
use std::collections::HashMap;

impl QueryBuilder {
    pub fn new(
        table: String,
        action: DatabaseAction,
        values: Option<HashMap<String, Value>>,
        table_columns: Option<HashMap<String, String>>,
        filters: Option<Filters>,
    ) -> Self {
        Self {
            table,
            action,
            values,
            filters,
            table_columns,
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
        let mut query: String;

        let mut bind_index = 1;

        match self.action {
            DatabaseAction::Retrieve => query = format!("SELECT * FROM {}", self.table),
            DatabaseAction::Update => {
                query = self.build_update_set(&mut bind_index)?;
            }
            DatabaseAction::Insert => {
                query = self.build_insert_query(&mut bind_index)?;
            }
            _ => return Err(anyhow!("Action not implemented")),
        }

        // putting it all together
        if let Some(filters) = &self.filters {
            let (where_clause_sql, bind_values) = filters.build_where_caluse(&mut bind_index)?;
            query.push_str(&where_clause_sql);
            self.bind_params.extend(bind_values);

            // only call those function on DatabaseAction::Retrieve
            if self.action == DatabaseAction::Retrieve {
                let order_by_sql = filters.build_order_by()?;
                query.push_str(&order_by_sql);

                let limit_sql = filters.build_limit()?;
                query.push_str(&limit_sql);

                let offset_sql = filters.build_offset()?;
                query.push_str(&offset_sql);
            }
        }

        Ok((query, self.bind_params.clone()))
    }

    fn convert_value(&self, column: &str, value: &Value, expected_type: &str) -> Result<Value> {
        match expected_type {
            "bigint" => {
                if let Some(s) = value.as_str() {
                    s.parse::<i64>()
                        .map(|v| Value::Number(v.into()))
                        .map_err(|_| anyhow!("Failed to convert {} to bigint", s))
                } else if value.is_i64() || value.is_u64() {
                    Ok(Value::Number(value.as_i64().unwrap().into()))
                } else {
                    Err(anyhow!("Expected a string or number for bigint"))
                }
            }
            "text" => {
                if value.is_string() {
                    Ok(value.clone())
                } else {
                    Err(anyhow!("Expected a string for text column {}", column))
                }
            }
            "boolean" => {
                if let Some(b) = value.as_bool() {
                    Ok(Value::Bool(b))
                } else {
                    Err(anyhow!("Expected a boolean for column {}", column))
                }
            }
            _ => Err(anyhow!("Unsupported column type: {}", expected_type)),
        }
    }

    fn build_insert_query(&mut self, _bind_index: &mut usize) -> Result<String> {
        let values = self
            .values
            .as_ref()
            .ok_or(anyhow!("No values provided for insert"))?;
        let table_columns = self
            .table_columns
            .as_ref()
            .ok_or(anyhow!("No table columns provided"))?;

        let columns: Vec<String> = values.keys().cloned().collect();
        let placeholders: Vec<String> = (1..=columns.len()).map(|i| format!("${}", i)).collect();

        for column in &columns {
            let value = values.get(column).unwrap();
            let expected_type = table_columns.get(column).ok_or(anyhow!(
                "Column {} does not exist in table {}",
                column,
                self.table
            ))?;

            let converted_value = self.convert_value(column, value, expected_type)?;
            self.bind_params.push(converted_value);
        }

        Ok(format!(
            "INSERT INTO {} ({}) VALUES ({})",
            self.table,
            columns.join(", "),
            placeholders.join(", ")
        ))
    }

    fn build_update_set(&mut self, bind_index: &mut usize) -> Result<String> {
        let values = self
            .values
            .as_ref()
            .ok_or(anyhow!("No values provided for update"))?;
        let table_columns = self
            .table_columns
            .as_ref()
            .ok_or(anyhow!("No table columns provided"))?;

        let mut set_clauses = Vec::new();

        for (column, value) in values {
            let sanitized_column = Filters::sanitize_column_name(column)?;
            let expected_type = table_columns.get(column).ok_or(anyhow!(
                "Column {} does not exist in table {}",
                column,
                self.table
            ))?;

            let converted_value = self.convert_value(column, value, expected_type)?;
            self.bind_params.push(converted_value);

            set_clauses.push(format!("{} = ${}", sanitized_column, bind_index));
            *bind_index += 1;
        }

        Ok(format!(
            "UPDATE {} SET {}",
            self.table,
            set_clauses.join(", ")
        ))
    }
}

impl Filters {
    fn sanitize_column_name(column: &str) -> Result<String> {
        let is_valid = column
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '_');
        if !is_valid {
            return Err(anyhow::anyhow!("Invalid column name: {}", column));
        }
        Ok(column.to_string())
    }

    pub fn build_where_caluse(&self, bind_index: &mut usize) -> Result<(String, Vec<Value>)> {
        let mut bind_values = Vec::new();

        let where_clause_sql: String = if let Some(where_clause) = &self.where_clause {
            match where_clause {
                WhereClause::And(map) | WhereClause::Single(map) => {
                    let conditions: Vec<String> = map
                        .iter()
                        .map(|(column, value)| {
                            let sanitized_column = Self::sanitize_column_name(column)?;
                            bind_values.push(value.clone());
                            *bind_index += 1;
                            Ok(format!("{} = ${}", sanitized_column, *bind_index - 1))
                        })
                        .collect::<Result<Vec<String>>>()?;

                    format!(" WHERE {}", conditions.join(" AND "))
                }

                WhereClause::Or(map) => {
                    let conditions: Vec<String> = map
                        .iter()
                        .map(|(column, value)| {
                            let sanitized_column = Self::sanitize_column_name(column)?;
                            bind_values.push(value.clone());
                            *bind_index += 1;
                            Ok(format!("{} = ${}", sanitized_column, *bind_index - 1))
                        })
                        .collect::<Result<Vec<String>>>()?;

                    format!(" WHERE ({})", conditions.join(" OR "))
                }
            }
        } else {
            String::new()
        };

        Ok((where_clause_sql, bind_values))
    }

    pub fn build_order_by(&self) -> Result<String> {
        if let Some(order_by) = &self.order_by {
            let sanitized_column = Self::sanitize_column_name(&order_by.column)?;
            let direction = match order_by.direction {
                OrderDirection::Asc => "ASC",
                OrderDirection::Desc => "DESC",
            };
            Ok(format!(" ORDER BY {} {}", sanitized_column, direction))
        } else {
            Ok(String::new()) // no ORDER BY clause if not specified
        }
    }

    pub fn build_limit(&self) -> Result<String> {
        if let Some(limit) = self.limit {
            Ok(format!(" LIMIT {}", limit))
        } else {
            Ok(String::new()) // no LIMIT clause if not specified
        }
    }

    pub fn build_offset(&self) -> Result<String> {
        if let Some(offset) = self.offset {
            Ok(format!(" OFFSET {}", offset))
        } else {
            Ok(String::new())
        }
    }
}
