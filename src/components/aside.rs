use leptos::html::Aside as HtmlAside;
use leptos::prelude::*;
use leptos::web_sys::{HtmlElement, PointerEvent};
use wasm_bindgen::JsCast;

use super::{files::Files, schema::Schema};

const BASE_CLASS: &str = "bg-[#1E1E2F] h-full border-r border-gray-700 p-2 flex flex-col";

/// Resolves the CSS classes that should be applied to the aside container.
///
/// This helper keeps the formatting logic isolated so that it can be covered
/// by unit tests without the need to mount the actual Leptos component.
pub fn resolve_aside_class(class: Option<String>) -> String {
    class
        .filter(|value| !value.is_empty())
        .map(|value| format!("{BASE_CLASS} {value}"))
        .unwrap_or_else(|| BASE_CLASS.to_string())
}

#[component]
pub fn Aside(#[prop(optional)] class: Option<String>) -> impl IntoView {
    let merged_class = resolve_aside_class(class);

    let aside_ref = create_node_ref::<HtmlAside>();
    let (files_ratio, set_files_ratio) = create_signal(0.55_f64);
    let (is_resizing, set_is_resizing) = create_signal(false);
    let (start_ratio, set_start_ratio) = create_signal(files_ratio.get_untracked());
    let (start_pointer_y, set_start_pointer_y) = create_signal(0.0_f64);

    view! {
        <aside
            node_ref=aside_ref
            class=merged_class
            on:pointermove=move |ev: PointerEvent| {
                if !is_resizing.get_untracked() {
                    return;
                }

                if let Some(aside_el) = aside_ref.get() {
                    let height = (aside_el.unchecked_ref::<HtmlElement>()).client_height() as f64;
                    if height > 0.0 {
                        let delta = ev.client_y() as f64 - start_pointer_y.get_untracked();
                        let new_ratio = start_ratio.get_untracked() + delta / height;
                        let clamped = new_ratio.clamp(0.15, 0.85);
                        set_files_ratio.set(clamped);
                    }
                }
            }
            on:pointerup=move |_| {
                set_is_resizing.set(false);
            }
            on:pointerleave=move |_| {
                set_is_resizing.set(false);
            }
        >
            <div
                class="flex-1 min-h-0 grid"
                style=move || {
                    let top = files_ratio.get();
                    let bottom = (1.0 - top).max(0.05);
                    format!("grid-template-rows: {top}fr 0.75rem {bottom}fr;")
                }
            >
                <div class="min-h-[3rem] overflow-hidden flex flex-col">
                    <Files />
                </div>
                <div
                    class="flex items-center justify-center cursor-row-resize select-none py-2"
                    role="separator"
                    aria-orientation="horizontal"
                    style="touch-action: none;"
                    on:pointerdown=move |ev: PointerEvent| {
                        ev.prevent_default();
                        let _ = ev.target()
                            .and_then(|target| target.dyn_into::<HtmlElement>().ok())
                            .map(|el| el.set_pointer_capture(ev.pointer_id()));
                        set_start_ratio.set(files_ratio.get_untracked());
                        set_start_pointer_y.set(ev.client_y() as f64);
                        set_is_resizing.set(true);
                    }
                >
                    <span class="h-px w-full bg-gray-700"></span>
                </div>
                <div class="min-h-[3rem] overflow-hidden flex flex-col">
                    <Schema />
                </div>
            </div>
        </aside>
    }
}
