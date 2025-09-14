use crate::components::query_editor::QueryEditor;
use leptos::ssr::render_to_string;
use leptos::*;

#[test]
fn given_query_editor_when_render_then_should_show_run_button() {
    let html = render_to_string(|| view! { <QueryEditor on_run=Callback::new(|_| {}) /> });
    assert!(html.contains("Run"));
}
