use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct FileModel {
    pub name: String,
    pub schema: Vec<crate::models::schema_field::SchemaField>,
}
