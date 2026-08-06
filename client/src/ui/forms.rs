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