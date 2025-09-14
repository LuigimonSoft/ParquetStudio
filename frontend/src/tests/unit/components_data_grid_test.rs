use crate::components::data_grid::DataGrid;
use leptos::ssr::render_to_string;
use leptos::*;

#[test]
fn given_rows_when_render_data_grid_then_should_show_cells() {
    let rows = vec![vec!["1".into(), "Alice".into()]];
    let html = render_to_string(|| view! { <DataGrid rows=rows /> });
    assert!(html.contains("Alice"));
}
