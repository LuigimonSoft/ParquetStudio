use std::collections::HashMap;

use crate::components::Aside;
use crate::models::FileModel;
use leptos::html::Div;
use leptos::prelude::*;
use leptos::web_sys::{HtmlElement, PointerEvent};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Serialize, Deserialize)]
struct GreetArgs<'a> {
    name: &'a str,
}

#[component]
pub fn App() -> impl IntoView {
    let (list_files, set_list_files) = signal(HashMap::<String, FileModel>::new());
    let (selected_file, set_selected_file) = signal(None as Option<FileModel>);
    let (aside_width, set_aside_width) = signal(260.0_f64);
    let (is_resizing_aside, set_is_resizing_aside) = signal(false);
    let (start_aside_width, set_start_aside_width) = signal(aside_width.get_untracked());
    let (start_pointer_x, set_start_pointer_x) = signal(0.0_f64);
    let layout_ref = NodeRef::<Div>::new();

    //add file to test
    set_list_files.update(|files| {
        files.insert(
            "example.parquet".into(),
            FileModel {
                name: "example.parquet".into(),
                schema: vec![
                    crate::models::SchemaField {
                        name: "id".into(),
                        data_type: "Int64".into(),
                    },
                    crate::models::SchemaField {
                        name: "name".into(),
                        data_type: "Utf8".into(),
                    },
                ],
            },
        );
        files.insert(
            "example1.parquet".into(),
            FileModel {
                name: "example1.parquet".into(),
                schema: vec![
                    crate::models::SchemaField {
                        name: "id".into(),
                        data_type: "Int32".into(),
                    },
                    crate::models::SchemaField {
                        name: "name".into(),
                        data_type: "Utf8".into(),
                    },
                    crate::models::SchemaField {
                        name: "address".into(),
                        data_type: "Utf8".into(),
                    },
                    crate::models::SchemaField {
                        name: "Telephone".into(),
                        data_type: "Utf8".into(),
                    },
                ],
            },
        );
    });

    provide_context(list_files);
    provide_context(set_list_files);
    provide_context(selected_file);
    provide_context(set_selected_file);

    view! {
        <div
            node_ref=layout_ref
            class="flex flex-1 overflow-hidden"
        >
            <div
                class="flex-shrink-0 h-full"
                style=move || format!("width: {:.0}px;", aside_width.get())
            >
                <Aside />
            </div>
            <div
                class="w-0.5 hover:w-1 bg-gray-700 hover:bg-gray-500 transition-all cursor-col-resize select-none"
                style="touch-action: none;"
                role="separator"
                aria-orientation="vertical"
                on:pointerdown=move |ev: PointerEvent| {
                    ev.prevent_default();
                    set_is_resizing_aside.set(true);
                    set_start_pointer_x.set(ev.client_x() as f64);
                    set_start_aside_width.set(aside_width.get_untracked());
                    if let Some(target) = ev.target().and_then(|t| t.dyn_into::<HtmlElement>().ok()) {
                        let _ = target.set_pointer_capture(ev.pointer_id());
                    }
                }
                on:pointermove=move |ev: PointerEvent| {
                    if !is_resizing_aside.get_untracked() {
                        return;
                    }

                    let delta = ev.client_x() as f64 - start_pointer_x.get_untracked();
                    let new_width = (start_aside_width.get_untracked() + delta).clamp(180.0, 480.0);
                    set_aside_width.set(new_width);
                }
                on:pointerup=move |ev: PointerEvent| {
                    set_is_resizing_aside.set(false);
                    if let Some(target) = ev.target().and_then(|t| t.dyn_into::<HtmlElement>().ok()) {
                        let _ = target.release_pointer_capture(ev.pointer_id());
                    }
                }
                on:pointercancel=move |ev: PointerEvent| {
                    set_is_resizing_aside.set(false);
                    if let Some(target) = ev.target().and_then(|t| t.dyn_into::<HtmlElement>().ok()) {
                        let _ = target.release_pointer_capture(ev.pointer_id());
                    }
                }
            ></div>
            <main class="flex-1 flex flex-col relative">

            </main>
        </div>
    }
}
