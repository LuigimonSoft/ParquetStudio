use frontend::components::{
    data_grid::DataGrid,
    layout::{sidebar::Sidebar, topbar::Topbar},
    query_editor::QueryEditor,
};
use frontend::models::{File, SchemaField};
use leptos::*;

#[component]
fn App() -> impl IntoView {
    let files = vec![File {
        name: "example.parquet".into(),
    }];
    let schema = vec![SchemaField {
        name: "id".into(),
        data_type: "Int64".into(),
    }];
    let rows = vec![vec!["1".into(), "Alice".into()]];

    view! {
        <div class="flex h-screen">
            <Sidebar files=files schema=schema />
            <div class="flex-1 flex flex-col">
                <Topbar on_open=Callback::new(|_| {}) on_save=Callback::new(|_| {}) />
                <div class="flex-1 p-4 space-y-4 overflow-auto">
                    <QueryEditor on_run=Callback::new(|_| {}) />
                    <DataGrid rows=rows />
                </div>
            </div>
        </div>
    }
}

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(start)]
pub fn main() {
    mount_to_body(|| view! { <App/> });
}

#[cfg(not(target_arch = "wasm32"))]
fn main() {}
