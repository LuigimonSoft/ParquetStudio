#[cfg(target_arch = "wasm32")]
use frontend::app::App;
#[cfg(target_arch = "wasm32")]
use leptos::*;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(start)]
pub fn main() {
    mount_to_body(|| view! { <App/> });
}

#[cfg(not(target_arch = "wasm32"))]
fn main() {}
