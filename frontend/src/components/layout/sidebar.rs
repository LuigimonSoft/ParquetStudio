use crate::models::{File, SchemaField};
use leptos::prelude::*;

#[component]
pub fn Sidebar(files: Vec<File>, schema: Vec<SchemaField>) -> impl IntoView {
    view! {
        <aside class="w-64 bg-gray-800 text-white p-4 overflow-y-auto">
            <h2 class="text-xs font-semibold text-gray-400 uppercase mb-2">"Files"</h2>
            <ul class="mb-4">
                {files.into_iter().map(|f| view! { <li class="px-2 py-1 rounded-md hover:bg-gray-700 cursor-pointer">{f.name}</li> }).collect_view()}
            </ul>
            <h2 class="text-xs font-semibold text-gray-400 uppercase mb-2">"Schema"</h2>
            <ul>
                {schema.into_iter().map(|s| view! { <li class="flex gap-2 text-sm"><span>{s.name}</span><span class="text-gray-400">{"("}{s.data_type}{")"}</span></li> }).collect_view()}
            </ul>
        </aside>
    }
}
