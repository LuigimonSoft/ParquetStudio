use crate::components::create_file_modal::CreateFileModal;
use leptos::prelude::*;

#[test]
fn given_modal_when_render_then_should_not_panic() {
    let _ = view! { <CreateFileModal/> };
}
