#![cfg(target_arch = "wasm32")]

use leptos::prelude::*;
use parquetstudio_ui::app::App;
use wasm_bindgen_test::wasm_bindgen_test;

wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn given_app_component_when_mounted_then_should_render_aside_should_render_aside_component() {
    console_error_panic_hook::set_once();

    let window = web_sys::window().expect("window should be available");
    let document = window.document().expect("document should be available");
    let body = document.body().expect("document should have a body");
    body.set_inner_html("");

    mount_to_body(|| view! { <App /> });

    let aside = document
        .query_selector("aside")
        .expect("selector should not fail");

    assert!(aside.is_some(), "App should render an <aside> element");
}
