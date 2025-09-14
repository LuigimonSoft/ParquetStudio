use crate::{
    components::layout::sidebar::Sidebar,
    models::{File, SchemaField},
};
use leptos::ssr::render_to_string;
use leptos::*;

#[test]
fn given_files_and_schema_when_render_sidebar_then_should_display_entries() {
    let files = vec![File {
        name: "ventas.parquet".into(),
    }];
    let schema = vec![SchemaField {
        name: "id".into(),
        data_type: "Int64".into(),
    }];
    let html = render_to_string(|| view! { <Sidebar files=files schema=schema /> });
    assert!(html.contains("ventas.parquet"));
    assert!(html.contains("Int64"));
}
