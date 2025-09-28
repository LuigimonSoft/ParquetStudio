use leptos::{prelude::*, task::spawn_local};
use std::collections::HashMap;
use wasm_bindgen::prelude::*;

use crate::models::{FileModel, SchemaField};

use super::FileParquet;

const EMPTY_STATE_TEXT: &str = "No files opened";

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[component]
pub fn Files(#[prop(optional)] files: Option<Vec<FileModel>>) -> impl IntoView {
    let open_file = move |_| {
        spawn_local(async {
            let _ = invoke("open_file_dialog", JsValue::NULL).await;
        });
    };

    let files = use_context::<ReadSignal<HashMap<String, FileModel>>>().unwrap();

    view! {
        <div class="flex items-center justify-between my-2">
            <h2 class="text-[11px] font-bold text-gray-400 uppercase">Files</h2>
            <button class="p-1 rounded hover:bg-gray-700" on:click=open_file>
                <i class="fas fa-plus text-gray-400"></i>
            </button>
        </div>
        <ul class="flex-1 space-y-1 overflow-y-auto text-sm">
            <Show when=move || !files.with(|files_map| files_map.is_empty())
                fallback=|| view! { <li class="text-xs text-gray-500">{EMPTY_STATE_TEXT}</li> }>
                    <For each=move || files.with(|files_map| files_map.values().cloned().collect::<Vec<_>>())
                        key=|file| file.name.clone()
                        children=|file| view! { <FileParquet file=file /> } />
            </Show>
        </ul>

    }
}
