use leptos::*;

#[component]
pub fn QueryEditor(on_run: Callback<String, ()>) -> impl IntoView {
    let (query, set_query) = create_signal(String::new());
    let run = move |_| on_run.call(query.get());

    view! {
        <div class="flex flex-col bg-gray-800 p-4 rounded-md">
            <textarea
                class="flex-1 bg-gray-900 text-white font-mono text-sm p-2 rounded"
                rows="6"
                on:input=move |ev| set_query.set(event_target_value(&ev))
            ></textarea>
            <button class="mt-2 self-start px-3 py-1 bg-emerald-600 text-white rounded" on:click=run>"Run"</button>
        </div>
    }
}
