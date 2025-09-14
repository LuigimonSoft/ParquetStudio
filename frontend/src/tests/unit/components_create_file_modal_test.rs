use crate::components::create_file_modal::CreateFileModal;
use leptos::ssr::render_to_string;
use leptos::*;

#[test]
fn given_modal_when_render_then_should_display_title_and_button() {
    let html = render_to_string(|| view! { <CreateFileModal/> });
    assert!(html.contains("Create Parquet File"));
    assert!(html.contains("Save"));
}
