use parquetstudio_ui::components::resolve_aside_class;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen_test::wasm_bindgen_test;

#[cfg(target_arch = "wasm32")]
wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn given_no_custom_class_when_resolve_aside_class_then_should_return_base_styles() {
    let resolved = resolve_aside_class(None);

    assert!(resolved.contains("bg-[#1E1E2F]"));
    assert!(!resolved.contains("extra-gap"));
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn given_custom_class_when_resolve_aside_class_then_should_append_custom_styles() {
    let resolved = resolve_aside_class(Some("extra-gap".into()));

    assert!(resolved.contains("bg-[#1E1E2F]"));
    assert!(resolved.ends_with("extra-gap"));
}
