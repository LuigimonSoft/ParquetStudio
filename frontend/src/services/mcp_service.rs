use crate::tauri_sys::tauri::invoke;
use wasm_bindgen::JsValue;

pub async fn start_mcp_server() -> Result<(), JsValue> {
    invoke("start_mcp_server", &()).await
}
