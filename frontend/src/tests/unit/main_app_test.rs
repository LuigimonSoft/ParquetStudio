use crate::app::App;
use leptos::ssr::render_to_string;
use leptos::*;

#[test]
fn given_default_state_when_render_app_then_should_display_sample_data() {
    let html = render_to_string(|| view! { <App/> });
    assert!(html.contains("example.parquet"));
    assert!(html.contains("Alice"));
}
