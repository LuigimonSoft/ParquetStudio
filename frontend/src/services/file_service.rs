use crate::{
    models::{File, SchemaField},
    tauri_sys::tauri::invoke,
};
use serde::Serialize;
use wasm_bindgen::JsValue;

pub async fn open_file(path: &str) -> Result<File, JsValue> {
    invoke("open_file", &path).await
}

#[derive(Serialize)]
struct CreateFileArgs<'a> {
    path: &'a str,
    schema: &'a [SchemaField],
}

pub async fn create_file(path: &str, schema: Vec<SchemaField>) -> Result<(), JsValue> {
    let args = CreateFileArgs {
        path,
        schema: &schema,
    };
    invoke("create_file", &args).await
}
