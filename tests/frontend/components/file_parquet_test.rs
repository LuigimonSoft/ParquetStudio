use parquetstudio_ui::components::resolve_file_parquet_class;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen_test::wasm_bindgen_test;

#[cfg(target_arch = "wasm32")]
wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn given_selected_file_when_resolve_file_parquet_class_then_should_include_selected_styles() {
    let class_name = resolve_file_parquet_class(true);

    assert!(class_name.contains("bg-gray-700"));
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn given_unselected_file_when_resolve_file_parquet_class_then_should_use_base_styles() {
    let class_name = resolve_file_parquet_class(false);

    assert_eq!(class_name, "px-2 py-1 rounded cursor-pointer hover:bg-gray-700");
}
