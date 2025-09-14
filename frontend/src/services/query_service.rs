use crate::tauri_sys::tauri::invoke;
use wasm_bindgen::JsValue;

pub async fn run_query(sql: &str) -> Result<Vec<Vec<String>>, JsValue> {
    invoke("run_query", &sql).await
}
