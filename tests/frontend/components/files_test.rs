use parquetstudio_ui::components::is_files_list_empty;
use parquetstudio_ui::models::{FileModel, SchemaField};
use std::collections::HashMap;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen_test::wasm_bindgen_test;

#[cfg(target_arch = "wasm32")]
wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn given_empty_map_when_checking_files_then_should_report_empty_state() {
    let files: HashMap<String, FileModel> = HashMap::new();

    assert!(is_files_list_empty(&files));
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn given_populated_map_when_checking_files_then_should_not_report_empty_state() {
    let mut files: HashMap<String, FileModel> = HashMap::new();
    files.insert(
        "example.parquet".into(),
        FileModel {
            name: "example.parquet".into(),
            schema: vec![SchemaField {
                name: "id".into(),
                data_type: "INT64".into(),
            }],
        },
    );

    assert!(!is_files_list_empty(&files));
}
