use leptos::*;

#[component]
pub fn CreateFileModal() -> impl IntoView {
    view! {
        <div class="p-4 bg-gray-800 rounded">
            <h2 class="text-sm text-gray-200 mb-2">"Create Parquet File"</h2>
            <button class="px-3 py-1 bg-emerald-600 rounded text-white">"Save"</button>
        </div>
    }
}
