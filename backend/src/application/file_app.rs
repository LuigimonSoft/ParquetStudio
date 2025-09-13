use crate::{models::SchemaField, repositories::FileRepository, services::ParquetService};
use anyhow::Result;

pub struct FileApp<R: FileRepository> {
    service: ParquetService<R>,
}

impl<R: FileRepository> FileApp<R> {
    pub fn new(service: ParquetService<R>) -> Self {
        Self { service }
    }

    pub fn get_schema(&self, path: &str) -> Result<Vec<SchemaField>> {
        self.service.get_schema(path)
    }

    pub fn create_file(&self, path: &str, schema: Vec<SchemaField>) -> Result<()> {
        self.service.create_file(path, schema)
    }
}
