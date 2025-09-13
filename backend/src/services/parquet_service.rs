use crate::{models::SchemaField, repositories::FileRepository};
use anyhow::Result;

pub struct ParquetService<R: FileRepository> {
    repo: R,
}

impl<R: FileRepository> ParquetService<R> {
    pub fn new(repo: R) -> Self {
        Self { repo }
    }

    pub fn get_schema(&self, path: &str) -> Result<Vec<SchemaField>> {
        let mut fields = self.repo.read_schema(path)?;
        for f in &mut fields {
            f.name = f.name.to_uppercase();
        }
        Ok(fields)
    }

    pub fn create_file(&self, path: &str, mut schema: Vec<SchemaField>) -> Result<()> {
        for f in &mut schema {
            f.name = f.name.to_uppercase();
        }
        self.repo.create_file(path, schema)
    }
}
