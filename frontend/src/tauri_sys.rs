use serde::Serialize;
use wasm_bindgen::JsValue;

pub mod tauri {
    use super::*;

    pub async fn invoke<T, A>(_cmd: &str, _args: &A) -> Result<T, JsValue>
    where
        T: serde::de::DeserializeOwned,
        A: Serialize + ?Sized,
    {
        Err(JsValue::NULL)
    }
}
