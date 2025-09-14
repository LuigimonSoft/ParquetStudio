use crate::{
    components::layout::sidebar::Sidebar,
    models::{File, SchemaField},
};
use leptos::prelude::*;

#[test]
fn given_files_and_schema_when_render_sidebar_then_should_not_panic() {
    let files = vec![File {
        name: "ventas.parquet".into(),
    }];
    let schema = vec![SchemaField {
        name: "id".into(),
        data_type: "Int64".into(),
    }];
    let _ = view! { <Sidebar files=files schema=schema /> };
}
