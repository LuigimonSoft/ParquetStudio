use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos::web_sys::MouseEvent;
use serde::Serialize;
use serde_wasm_bindgen::to_value;
use wasm_bindgen::prelude::*;

use crate::models::FileModel;

const BASE_CLASS: &str = "px-2 py-1 rounded cursor-pointer hover:bg-gray-700";

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Serialize)]
struct FileContextMenuArgs {
    x: f64,
    y: f64,
}

#[component]
pub fn FileParquet(#[prop()] file: FileModel) -> impl IntoView {
    let selected_file =
        use_context::<ReadSignal<Option<FileModel>>>().expect("selected_file signal not provided");
    let set_selected_file = use_context::<WriteSignal<Option<FileModel>>>()
        .expect("set_selected_file signal not provided");

    let file_name = file.name.clone();
    let display_name = file_name.clone();

    let select_file = {
        let file_for_select = file.clone();
        move |_| {
            set_selected_file.set(Some(file_for_select.clone()));
        }
    };

    let class = move || {
        let is_selected = selected_file.with(|maybe_selected| {
            maybe_selected
                .as_ref()
                .map(|selected| selected.name == file_name)
                .unwrap_or(false)
        });

        if is_selected {
            format!("{BASE_CLASS} bg-gray-700")
        } else {
            BASE_CLASS.to_string()
        }
    };

    view! {
        <li
            class=class
            on:click=select_file
            on:contextmenu={
                let file_for_select = file.clone();
                move |ev: MouseEvent| {
                    ev.prevent_default();
                    ev.stop_propagation();
                    set_selected_file.set(Some(file_for_select.clone()));

                    let args = FileContextMenuArgs {
                        x: ev.client_x() as f64,
                        y: ev.client_y() as f64,
                    };

                    spawn_local(async move {
                        if let Ok(payload) = to_value(&args) {
                            let _ = invoke("show_file_context_menu", payload).await;
                        }
                    });
                }
            }
        >
            <div class="flex items-center gap-2">
                <i class="fas fa-database text-gray-400"></i>
                <span>{display_name}</span>
            </div>
        </li>
    }
}
