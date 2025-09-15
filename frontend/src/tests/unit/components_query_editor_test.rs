use crate::components::query_editor::{highlight_sql, QueryEditor};
use leptos::prelude::*;

#[test]
fn given_query_editor_when_render_then_should_not_panic() {
    let _ = view! { <QueryEditor on_run=Callback::new(|_| {}) /> };
}

#[test]
fn given_sql_with_keywords_when_highlight_then_should_wrap_in_span() {
    let html = highlight_sql("select * from t");
    assert!(html.contains("<span"));
}
