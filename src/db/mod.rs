use sqlx::postgres::PgRow;
use sqlx::Error as SqlxError;
use std::collections::HashMap;

pub trait TableModel: Send + Sync {
    fn from_row(row: &PgRow) -> Result<Self, SqlxError>
    where
        Self: Sized;
    fn table_name() -> &'static str
    where
        Self: Sized;
    fn debug_string(&self) -> String;
    fn as_value(&self) -> serde_json::Value;
}

pub type ModelFactory = fn(&PgRow) -> Box<dyn TableModel + Send + Sync>;

pub struct ModelEntry {
    pub factory: ModelFactory,
}

pub struct ModelRegistry {
    pub models: HashMap<&'static str, ModelEntry>,
}

impl ModelRegistry {
    pub fn new() -> Self {
        Self {
            models: HashMap::new(),
        }
    }

    pub fn register<T: TableModel + 'static>(&mut self) {
        let table_name = T::table_name();
        self.models.insert(
            table_name,
            ModelEntry {
                factory: |row| Box::new(T::from_row(row).unwrap()),
            },
        );
    }

    pub fn get(&self, table_name: &str) -> Option<&ModelEntry> {
        self.models.get(table_name)
    }
}
