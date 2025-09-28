use crate::models::FileModel;
use leptos::prelude::*;

#[component]
pub fn Schema() -> impl IntoView {
    let selected_file =
        use_context::<ReadSignal<Option<FileModel>>>().expect("selected_file signal not provided");

    view! {
        <div class="flex items-center justify-between mb-3">
            <h2 class="text-[11px] font-bold text-gray-400 uppercase">Schema</h2>
            <div class="flex gap-1">
                <button class="p-1 rounded hover:bg-gray-700"><i class="fas fa-plus text-gray-400"></i></button>
                <button class="p-1 rounded hover:bg-gray-700"><i class="fas fa-minus text-gray-400"></i></button>
            </div>
        </div>
        <ul class="flex-1 space-y-1 overflow-y-auto text-xs">
        <For
            each=move || {
                selected_file.get().map(|file| file.schema.clone()).unwrap_or_default()
            }
            key=|item| item.name.clone()
            children=|item| view! {
                <li class="grid grid-cols-[minmax(0,1fr)_7rem] items-center gap-2 px-1 py-0.5">
                    <span contenteditable="true"
                        class="truncate border-b border-transparent hover:border-gray-600 focus:outline-none focus:border-blue-500">{item.name.clone()}</span>
                    <select class="w-full bg-transparent text-gray-200 text-xs focus:outline-none cursor-pointer" prop:value={item.data_type.clone().to_ascii_uppercase()}>
                        <option value="BOOLEAN">BOOLEAN</option>
                        <option value="INT32">INT32</option>
                        <option value="INT64">INT64</option>
                        <option value="FLOAT">FLOAT</option>
                        <option value="DOUBLE">DOUBLE</option>
                        <option value="BYTE_ARRAY">BYTE_ARRAY</option>
                        <option value="FIXED_LEN_BYTE_ARRAY">FIXED_LEN_BYTE_ARRAY</option>
                        <option value="UTF8">UTF8 (string)</option>
                        <option value="DATE">DATE</option>
                        <option value="TIME_MILLIS">TIME_MILLIS</option>
                        <option value="TIME_MICROS">TIME_MICROS</option>
                        <option value="TIMESTAMP_MILLIS">TIMESTAMP_MILLIS</option>
                        <option value="TIMESTAMP_MICROS">TIMESTAMP_MICROS</option>
                        <option value="DECIMAL">DECIMAL</option>
                        <option value="UUID">UUID</option>
                        <option value="JSON">JSON</option>
                        <option value="BSON">BSON</option>
                        <option value="LIST">LIST</option>
                        <option value="MAP">MAP</option>
                        <option value="STRUCT">STRUCT</option>
                    </select>
                </li>
            }

        />
        </ul>
    }
}
