use dioxus::prelude::*;
use crate::ui::tokens::NATIVE_INPUT;

#[component]
pub fn Input(
    #[props(default)] place_holer: Option<String>,
    value: String,
    is_error: bool,
    oninput: EventHandler<FormEvent>,
) -> Element {
    let error_class = if is_error {
        "border-red-400 bg-red-50/30 focus:ring-red-400/50 focus:border-red-400 text-red-900"
    } else {
        ""
    };

    rsx! {
        input {
            r#type: "text",
            class: "{NATIVE_INPUT} {error_class}",
            placeholder: place_holer.unwrap_or_default(),
            value: "{value}",
            oninput: move |e| oninput.call(e),
        }
    }
}

#[component]
pub fn SearchInput(
    #[props(default)] placeholder: Option<String>,
    value: String,
    oninput: EventHandler<String>,
    #[props(default)] class: String,
) -> Element {
    let val_for_clear = value.clone();
    rsx! {
        div { class: "relative w-full flex items-center {class}",
            // Such-Icon (Lupe)
            div { class: "absolute left-3.5 pointer-events-none text-amber-700/50 flex items-center justify-center",
                svg {
                    class: "w-4 h-4",
                    fill: "none",
                    stroke: "currentColor",
                    stroke_width: "2",
                    view_box: "0 0 24 24",
                    path {
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                        d: "M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z",
                    }
                }
            }
            input {
                r#type: "text",
                class: "{NATIVE_INPUT} pl-10 pr-9 py-2 text-sm w-full bg-white/90 border border-amber-200/80 rounded-xl focus:border-amber-400 focus:ring-2 focus:ring-amber-400/20 transition-all",
                placeholder: placeholder.unwrap_or_else(|| "Suchen…".to_string()),
                value: "{value}",
                oninput: move |e: FormEvent| oninput.call(e.value()),
            }
            // Clear Button (erscheint nur wenn Suchtext vorhanden)
            if !val_for_clear.is_empty() {
                button {
                    r#type: "button",
                    class: "absolute right-2.5 p-1 text-zinc-400 hover:text-zinc-600 rounded-lg hover:bg-amber-100/50 transition-colors",
                    onclick: move |_| oninput.call(String::new()),
                    aria_label: "Suche zurücksetzen",
                    svg {
                        class: "w-3.5 h-3.5",
                        fill: "none",
                        stroke: "currentColor",
                        stroke_width: "2",
                        view_box: "0 0 24 24",
                        path {
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            d: "M6 18L18 6M6 6l12 12",
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn InputDate(
    value: String,
    oninput: EventHandler<FormEvent>,
) -> Element {
    rsx! {
        input {
            r#type: "date",
            class: "{NATIVE_INPUT} w-full block",
            value: "{value}",
            oninput: move |e| oninput.call(e),
        }
    }
}

#[component]
pub fn InputError(error: String) -> Element {
    if error.is_empty() {
        return rsx! {
            div { class: "h-5" }
        }; 
    }

    rsx! {
        div { class: "flex items-center gap-1.5 h-5 mt-1",
            svg {
                class: "w-3.5 h-3.5 text-red-500 shrink-0",
                fill: "currentColor",
                view_box: "0 0 20 20",
                path {
                    fill_rule: "evenodd",
                    clip_rule: "evenodd",
                    d: "M18 10a8 8 0 11-16 0 8 8 0 0116 0zm-7 4a1 1 0 11-2 0 1 1 0 012 0zm-1-9a1 1 0 00-1 1v4a1 1 0 102 0V6a1 1 0 00-1-1z",
                }
            }
            span { class: "text-xs font-medium text-red-500", "{error}" }
        }
    }
}