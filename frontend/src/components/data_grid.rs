use leptos::prelude::*;

#[component]
pub fn DataGrid(rows: Vec<Vec<String>>) -> impl IntoView {
    view! {
        <table class="min-w-full text-sm text-left text-gray-300">
            <tbody>
                {rows.into_iter().map(|row| view! {
                    <tr class="border-b border-gray-700 hover:bg-gray-800">
                        {row.into_iter().map(|cell| view! { <td class="px-3 py-2">{cell}</td> }).collect_view()}
                    </tr>
                }).collect_view()}
            </tbody>
        </table>
    }
}
