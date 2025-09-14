use crate::components::layout::topbar::Topbar;
use leptos::ssr::render_to_string;
use leptos::*;

#[test]
fn given_topbar_when_render_then_should_show_buttons() {
    let html = render_to_string(
        || view! { <Topbar on_open=Callback::new(|_| {}) on_save=Callback::new(|_| {}) /> },
    );
    assert!(html.contains("Open"));
    assert!(html.contains("Save"));
}
