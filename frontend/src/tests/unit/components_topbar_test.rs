use crate::components::layout::topbar::Topbar;
use leptos::prelude::*;

#[test]
fn given_topbar_when_render_then_should_not_panic() {
    let _ = view! { <Topbar on_open=Callback::new(|_| {}) on_save=Callback::new(|_| {}) /> };
}
