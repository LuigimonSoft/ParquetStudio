use crate::components::data_grid::DataGrid;
use leptos::prelude::*;

#[test]
fn given_rows_when_render_data_grid_then_should_not_panic() {
    let rows = vec![vec!["1".into(), "Alice".into()]];
    let _ = view! { <DataGrid rows=rows /> };
}
