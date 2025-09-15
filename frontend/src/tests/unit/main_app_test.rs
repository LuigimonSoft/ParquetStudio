use crate::app::App;
use leptos::prelude::*;

#[test]
fn given_default_state_when_render_app_then_should_not_panic() {
    let _ = view! { <App/> };
}
