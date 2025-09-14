use leptos::prelude::*;

#[component]
pub fn Topbar(on_open: Callback<(), ()>, on_save: Callback<(), ()>) -> impl IntoView {
    view! {
        <header class="flex items-center justify-between bg-gray-900 text-white px-4 py-2">
            <div class="font-semibold">"Parquet Studio"</div>
            <div class="flex gap-2">
                <button class="px-3 py-1 bg-blue-600 rounded" on:click=move |_| on_open.run(())>"Open"</button>
                <button class="px-3 py-1 bg-green-600 rounded" on:click=move |_| on_save.run(())>"Save"</button>
            </div>
        </header>
    }
}
