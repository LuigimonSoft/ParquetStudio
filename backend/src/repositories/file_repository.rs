use crate::models::SchemaField;
use anyhow::Result;

#[cfg_attr(test, mockall::automock)]
pub trait FileRepository: Send + Sync {
    fn read_schema(&self, path: &str) -> Result<Vec<SchemaField>>;
    fn create_file(&self, path: &str, schema: Vec<SchemaField>) -> Result<()>;
}
