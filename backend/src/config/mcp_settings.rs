use serde::{Deserialize, Serialize};
use std::{fs, path::Path};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Prompt {
    pub id: String,
    pub description: String,
    pub template: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Datastore {
    pub id: String,
    pub r#type: String,
    pub path: String,
    pub exposed: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Tool {
    pub id: String,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct McpSettings {
    pub prompts: Vec<Prompt>,
    pub datastores: Vec<Datastore>,
    pub tools: Vec<Tool>,
}

impl McpSettings {
    pub fn load(path: &str) -> Self {
        if Path::new(path).exists() {
            let data = fs::read_to_string(path).unwrap_or_default();
            serde_json::from_str(&data).unwrap_or_default()
        } else {
            Self::default()
        }
    }

    pub fn save(&self, path: &str) {
        if let Ok(json) = serde_json::to_string_pretty(self) {
            let _ = fs::write(path, json);
        }
    }
}
