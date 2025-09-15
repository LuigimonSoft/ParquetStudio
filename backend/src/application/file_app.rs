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

#[cfg(feature = "tauri")]
use crate::repositories::LocalFileRepository;
#[cfg(feature = "tauri")]
use tauri::State;

#[cfg(feature = "tauri")]
#[tauri::command]
pub fn open_file(
    path: String,
    state: State<FileApp<LocalFileRepository>>,
) -> Result<Vec<SchemaField>, String> {
    state.get_schema(&path).map_err(|e| e.to_string())
}

#[cfg(feature = "tauri")]
#[tauri::command]
pub fn create_file(
    path: String,
    schema: Vec<SchemaField>,
    state: State<FileApp<LocalFileRepository>>,
) -> Result<(), String> {
    state.create_file(&path, schema).map_err(|e| e.to_string())
}
