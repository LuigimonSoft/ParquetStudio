use crate::components::query_editor::QueryEditor;
use leptos::prelude::*;

#[test]
fn given_query_editor_when_render_then_should_not_panic() {
    let _ = view! { <QueryEditor on_run=Callback::new(|_| {}) /> };
}
