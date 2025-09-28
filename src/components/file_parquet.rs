use leptos::prelude::*;
use leptos::web_sys::{MouseEvent, PointerEvent};

use crate::models::FileModel;

const BASE_CLASS: &str = "px-2 py-1 rounded cursor-pointer hover:bg-gray-700";

#[component]
pub fn FileParquet(#[prop()] file: FileModel) -> impl IntoView {
    let selected_file =
        use_context::<ReadSignal<Option<FileModel>>>().expect("selected_file signal not provided");
    let set_selected_file = use_context::<WriteSignal<Option<FileModel>>>()
        .expect("set_selected_file signal not provided");

    let file_name = file.name.clone();
    let display_name = file_name.clone();
    let (context_open, set_context_open) = signal(false);
    let (menu_position, set_menu_position) = signal((0.0_f64, 0.0_f64));
    let (query_menu_open, set_query_menu_open) = signal(false);

    let select_file = {
        let file_for_select = file.clone();
        move |_| {
            set_selected_file.set(Some(file_for_select.clone()));
            set_context_open.set(false);
        }
    };

    let open_context_menu = {
        let file_for_select = file.clone();
        move |ev: MouseEvent| {
            ev.prevent_default();
            ev.stop_propagation();
            set_selected_file.set(Some(file_for_select.clone()));
            set_menu_position.set((ev.client_x() as f64, ev.client_y() as f64));
            set_query_menu_open.set(false);
            set_context_open.set(true);
        }
    };

    let close_context_menu = move |_| {
        set_context_open.set(false);
    };

    let class = move || {
        let is_selected = selected_file.with(|maybe_selected| {
            maybe_selected
                .as_ref()
                .map(|selected| selected.name == file_name)
                .unwrap_or(false)
        });

        if is_selected {
            format!("{BASE_CLASS} bg-gray-700")
        } else {
            BASE_CLASS.to_string()
        }
    };

    view! {
        <li
            class=class
            on:click=select_file
            on:contextmenu=open_context_menu
        >
            <div class="flex items-center gap-2">
                <i class="fas fa-database text-gray-400"></i>
                <span>{display_name}</span>
            </div>
        </li>
        <Show when=move || context_open.get() fallback=|| ()>
            <>
                <div
                    class="fixed inset-0 z-40"
                    on:click=close_context_menu
                    on:contextmenu=move |ev: MouseEvent| {
                        ev.prevent_default();
                        ev.stop_propagation();
                        set_context_open.set(false);
                    }
                ></div>
                <div
                    class="fixed z-50 min-w-[10rem] rounded-md border border-gray-600 bg-gray-800 shadow-lg text-sm"
                    style=move || {
                        let (x, y) = menu_position.get();
                        format!("top: {:.0}px; left: {:.0}px;", y, x)
                    }
                    on:click=move |ev: MouseEvent| {
                        ev.stop_propagation();
                    }
                    on:contextmenu=move |ev: MouseEvent| {
                        ev.prevent_default();
                        ev.stop_propagation();
                    }
                >
                    <ul class="py-1">
                        <li
                            class="px-3 py-1.5 hover:bg-gray-700 cursor-pointer"
                            on:click=move |_| {
                                set_context_open.set(false);
                                set_query_menu_open.set(false);
                            }
                        >New query</li>
                        <li
                            class="relative px-3 py-1.5 hover:bg-gray-700 cursor-pointer select-none"
                            on:pointerenter=move |ev: PointerEvent| {
                                ev.stop_propagation();
                                set_query_menu_open.set(true);
                            }
                            on:pointerleave=move |_| {
                                set_query_menu_open.set(false);
                            }
                        >
                            <div class="flex items-center justify-between font-semibold">
                                <span>{"Script as"}</span>
                                <i class="fas fa-chevron-right text-[10px]"></i>
                            </div>
                            <Show when=move || query_menu_open.get() fallback=|| ()>
                                <ul
                                    class="absolute top-0 left-full ml-1 min-w-[8rem] rounded-md border border-gray-600 bg-gray-800 shadow-lg text-xs py-1"
                                >
                                    <li
                                        class="px-3 py-1 hover:bg-gray-700 rounded cursor-pointer"
                                        on:click=move |_| {
                                            set_context_open.set(false);
                                            set_query_menu_open.set(false);
                                        }
                                    >SELECT</li>
                                    <li
                                        class="px-3 py-1 hover:bg-gray-700 rounded cursor-pointer"
                                        on:click=move |_| {
                                            set_context_open.set(false);
                                            set_query_menu_open.set(false);
                                        }
                                    >UPDATE</li>
                                    <li
                                        class="px-3 py-1 hover:bg-gray-700 rounded cursor-pointer"
                                        on:click=move |_| {
                                            set_context_open.set(false);
                                            set_query_menu_open.set(false);
                                        }
                                    >INSERT</li>
                                    <li
                                        class="px-3 py-1 hover:bg-gray-700 rounded cursor-pointer"
                                        on:click=move |_| {
                                            set_context_open.set(false);
                                            set_query_menu_open.set(false);
                                        }
                                    >DELETE</li>
                                </ul>
                            </Show>
                        </li>
                    </ul>
                </div>
            </>
        </Show>
    }
}
