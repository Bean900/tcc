mod dashboard;
mod details;

use std::collections::HashMap;

pub use dashboard::Dashboard;
pub use details::ProjectDetailPage;

use dioxus::prelude::*;
use dioxus::signals::{Readable, Signal};
use gloo_timers::future::TimeoutFuture;
use uuid::Uuid;

const DISABLED_BUTTON: &str = "bg-gray-300 text-gray-500 rounded-lg px-2 py-2 cursor-not-allowed";
const ENABLED_BUTTON_BLUE: &str =
    "bg-blue-500 text-white rounded-lg px-2 py-2 hover:bg-blue-600 transition-all cursor-pointer";
const ENABLED_BUTTON_GREEN: &str =
    "bg-green-500 text-white rounded-lg px-2 py-2 hover:bg-green-600 transition-all cursor-pointer";
const ENABLED_BUTTON_RED: &str =
    "bg-red-500 text-white rounded-lg px-2 py-2 hover:bg-red-600 transition-all cursor-pointer";
const ENABLED_BUTTON_RED_HOLLOW: &str =
    "border border-red-500 text-red-500 px-4 py-2 rounded hover:bg-red-100 cursor-pointer";

#[derive(Clone, PartialEq)]
enum ButtonColor {
    Blue,
    Green,
    Red,
    RedHollow,
}

#[component]
pub(crate) fn BlueButton(
    text: String,
    error_signal: Option<Signal<String>>,
    onclick: Option<EventHandler<MouseEvent>>,
) -> Element {
    rsx! {
        CustomButton {
            color: ButtonColor::Blue,
            text: text.clone(),
            error_signal: error_signal.clone(),
            onclick: onclick.clone(),
        }
    }
}

#[component]
pub(crate) fn GreenButton(
    text: String,
    error_signal: Option<Signal<String>>,
    onclick: Option<EventHandler<MouseEvent>>,
) -> Element {
    rsx! {
        CustomButton {
            color: ButtonColor::Green,
            text: text.clone(),
            error_signal: error_signal.clone(),
            onclick: onclick.clone(),
        }
    }
}

#[component]
pub(crate) fn RedButton(
    text: String,
    error_signal: Option<Signal<String>>,
    onclick: Option<EventHandler<MouseEvent>>,
) -> Element {
    rsx! {
        CustomButton {
            color: ButtonColor::Red,
            text: text.clone(),
            error_signal: error_signal.clone(),
            onclick: onclick.clone(),
        }
    }
}

#[component]
pub(crate) fn RedHollowButton(
    text: String,
    error_signal: Option<Signal<String>>,
    onclick: Option<EventHandler<MouseEvent>>,
) -> Element {
    rsx! {
        CustomButton {
            color: ButtonColor::RedHollow,
            text: text.clone(),
            error_signal: error_signal.clone(),
            onclick: onclick.clone(),
        }
    }
}

#[component]
fn CustomButton(
    color: ButtonColor,
    text: String,
    error_signal: Option<Signal<String>>,
    onclick: Option<EventHandler<MouseEvent>>,
) -> Element {
    let mut loading_signal = use_signal(|| false);
    let on_click_function = move |event: Event<MouseData>| {
        if error_signal.map_or(true, |s| s.read().is_empty()) {
            if let Some(onclick) = &onclick {
                loading_signal.set(true);

                let mut loading_signal_clone = loading_signal.clone();
                let onclick = onclick.clone();
                let event = event.clone();
                spawn(async move {
                    onclick.call(event);
                    loading_signal_clone.set(false);
                });
            }
        }
    };

    let enable_button = match color {
        ButtonColor::Blue => ENABLED_BUTTON_BLUE,
        ButtonColor::Green => ENABLED_BUTTON_GREEN,
        ButtonColor::Red => ENABLED_BUTTON_RED,
        ButtonColor::RedHollow => ENABLED_BUTTON_RED_HOLLOW,
    };

    rsx! {
        if *loading_signal.read() {
            div {
                role: "status",
                class: "flex justify-center items-center h-12",
                svg {
                    class: "w-8 h-8 text-gray-200 animate-spin dark:text-gray-600 fill-blue-600",
                    view_box: "0 0 100 101",
                    fill: "none",
                    xmlns: "http://www.w3.org/2000/svg",
                    path {
                        d: "M100 50.5908C100 78.2051 77.6142 100.591 50 100.591C22.3858 100.591 0 78.2051 0 50.5908C0 22.9766 22.3858 0.59082 50 0.59082C77.6142 0.59082 100 22.9766 100 50.5908ZM9.08144 50.5908C9.08144 73.1895 27.4013 91.5094 50 91.5094C72.5987 91.5094 90.9186 73.1895 90.9186 50.5908C90.9186 27.9921 72.5987 9.67226 50 9.67226C27.4013 9.67226 9.08144 27.9921 9.08144 50.5908Z",
                        fill: "currentColor",
                    }
                    path {
                        d: "M93.9676 39.0409C96.393 38.4038 97.8624 35.9116 97.0079 33.5539C95.2932 28.8227 92.871 24.3692 89.8167 20.348C85.8452 15.1192 80.8826 10.7238 75.2124 7.41289C69.5422 4.10194 63.2754 1.94025 56.7698 1.05124C51.7666 0.367541 46.6976 0.446843 41.7345 1.27873C39.2613 1.69328 37.813 4.19778 38.4501 6.62326C39.0873 9.04874 41.5694 10.4717 44.0505 10.1071C47.8511 9.54855 51.7191 9.52689 55.5402 10.0491C60.8642 10.7766 65.9928 12.5457 70.6331 15.2552C75.2735 17.9648 79.3347 21.5619 82.5849 25.841C84.9175 28.9121 86.7997 32.2913 88.1811 35.8758C89.083 38.2158 91.5421 39.6781 93.9676 39.0409Z",
                        fill: "currentFill",
                    }
                }
            }
        } else {

            button {
                class: if error_signal.is_some() && !error_signal.expect("Expect signal").read().is_empty() { DISABLED_BUTTON } else { enable_button },
                disabled: error_signal.is_some() && !error_signal.expect("Expect signal").read().is_empty(),
                onclick: on_click_function,
                "{text}"
            }
        }
    }
}

#[component]
fn CloseButton(onclick: EventHandler<MouseEvent>) -> Element {
    rsx! {
        button {
            class: "hover:text-gray-600 absolute top-3 right-3 cursor-pointer",
            onclick: move |event| {
                onclick.call(event);
            },
            svg {
                class: "w-6 h-6",
                stroke: "currentColor",
                xmlns: "http://www.w3.org/2000/svg",
                view_box: "0 0 24 24",
                path { d: "M6 18L18 6M6 6l12 12" }
            }
        }
    }
}

#[component]
pub(crate) fn DeleteButton(
    error_signal: Option<Signal<String>>,
    onclick: Option<EventHandler<MouseEvent>>,
) -> Element {
    let mut loading_signal = use_signal(|| false);
    let on_click_function = move |event: Event<MouseData>| {
        if error_signal.map_or(true, |s| s.read().is_empty()) {
            if let Some(onclick) = &onclick {
                loading_signal.set(true);

                let mut loading_signal_clone = loading_signal.clone();
                let onclick = onclick.clone();
                let event = event.clone();
                spawn(async move {
                    onclick.call(event);
                    loading_signal_clone.set(false);
                });
            }
        }
    };

    rsx! {
        if *loading_signal.read() {
            div {
                role: "status",
                class: "flex justify-center items-center h-12",
                svg {
                    class: "w-8 h-8 text-gray-200 animate-spin dark:text-gray-600 fill-red-600",
                    view_box: "0 0 100 101",
                    fill: "none",
                    xmlns: "http://www.w3.org/2000/svg",
                    path {
                        d: "M100 50.5908C100 78.2051 77.6142 100.591 50 100.591C22.3858 100.591 0 78.2051 0 50.5908C0 22.9766 22.3858 0.59082 50 0.59082C77.6142 0.59082 100 22.9766 100 50.5908ZM9.08144 50.5908C9.08144 73.1895 27.4013 91.5094 50 91.5094C72.5987 91.5094 90.9186 73.1895 90.9186 50.5908C90.9186 27.9921 72.5987 9.67226 50 9.67226C27.4013 9.67226 9.08144 27.9921 9.08144 50.5908Z",
                        fill: "currentColor",
                    }
                    path {
                        d: "M93.9676 39.0409C96.393 38.4038 97.8624 35.9116 97.0079 33.5539C95.2932 28.8227 92.871 24.3692 89.8167 20.348C85.8452 15.1192 80.8826 10.7238 75.2124 7.41289C69.5422 4.10194 63.2754 1.94025 56.7698 1.05124C51.7666 0.367541 46.6976 0.446843 41.7345 1.27873C39.2613 1.69328 37.813 4.19778 38.4501 6.62326C39.0873 9.04874 41.5694 10.4717 44.0505 10.1071C47.8511 9.54855 51.7191 9.52689 55.5402 10.0491C60.8642 10.7766 65.9928 12.5457 70.6331 15.2552C75.2735 17.9648 79.3347 21.5619 82.5849 25.841C84.9175 28.9121 86.7997 32.2913 88.1811 35.8758C89.083 38.2158 91.5421 39.6781 93.9676 39.0409Z",
                        fill: "currentFill",
                    }
                }
            }
        } else {
            button { class: ENABLED_BUTTON_RED, onclick: on_click_function,
                svg {
                    xmlns: "http://www.w3.org/2000/svg",
                    fill: "none",
                    view_box: "0 0 24 24",
                    stroke_width: "2",
                    stroke: "currentColor",
                    class: "w-6 h-6",
                    path {
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                        d: "M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6M9 7V4a1 1 0 011-1h4a1 1 0 011 1v3m4 0H5",
                    }
                }
            }
        }
    }
}

#[component]
pub(crate) fn Input(
    place_holer: Option<String>,
    value: String,
    is_error: bool,
    oninput: EventHandler<dioxus::prelude::Event<FormData>>,
) -> Element {
    rsx! {

        input {
            class: if is_error { "w-full border border-red-500 text-red-500 rounded-lg p-2 mb-4 focus:outline-none focus:ring-2 focus:ring-red-500" } else { "w-full border border-gray-300 rounded-lg p-2 mb-4 focus:outline-none focus:ring-2 focus:ring-blue-500" },
            r#type: "text",
            placeholder: if place_holer.is_some() { place_holer.expect("Expected place holder") } else { "" },
            value,
            oninput: move |e| {
                oninput.call(e);
            },
        }
    }
}

#[component]
pub(crate) fn InputMultirow(
    place_holer: Option<String>,
    value: String,
    error_signal: Option<Signal<String>>,
    oninput: EventHandler<dioxus::prelude::Event<FormData>>,
) -> Element {
    rsx! {
        textarea {
            class: if error_signal.is_some()
    && !error_signal.expect("Expect error signal").read().is_empty() { "w-full border border-red-500 text-red-500 rounded-lg p-2 mb-4 focus:outline-none focus:ring-2 focus:ring-red-500" } else { "w-full border border-gray-300 rounded-lg p-2 mb-4 focus:outline-none focus:ring-2 focus:ring-blue-500" },
            placeholder: if place_holer.is_some() { place_holer.expect("Expected place holder") } else { "" },
            rows: "3",
            value,
            oninput: move |e| {
                oninput.call(e);
            },
        }
    }
}

#[component]
pub(crate) fn InputNumber(
    place_holer: Option<String>,
    value: String,
    error_signal: Option<Signal<String>>,
    oninput: EventHandler<dioxus::prelude::Event<FormData>>,
) -> Element {
    rsx! {
        input {
            class: if error_signal.is_some()
    && !error_signal.expect("Expect error signal").read().is_empty() { "w-full border border-red-500 text-red-500 rounded-lg p-2 mb-4 focus:outline-none focus:ring-2 focus:ring-red-500" } else { "w-full border border-gray-300 rounded-lg p-2 mb-4 focus:outline-none focus:ring-2 focus:ring-blue-500" },
            r#type: "number",
            placeholder: if place_holer.is_some() { place_holer.expect("Expected place holder") } else { "" },
            value,
            oninput: move |e| {
                oninput.call(e);
            },
        }
    }
}

#[component]
pub(crate) fn InputTime(
    place_holer: Option<String>,
    value: String,
    is_error: bool,
    oninput: EventHandler<dioxus::prelude::Event<FormData>>,
) -> Element {
    rsx! {
        input {
            class: if is_error { "w-full border border-red-500 text-red-500 rounded-lg p-2 mb-4 focus:outline-none focus:ring-2 focus:ring-red-500" } else { "w-full border border-gray-300 rounded-lg p-2 mb-4 focus:outline-none focus:ring-2 focus:ring-blue-500" },
            r#type: "time",
            placeholder: if place_holer.is_some() { place_holer.expect("Expected place holder") } else { "" },
            value,
            oninput: move |e| {
                oninput.call(e);
            },
        }
    }
}

#[component]
pub(crate) fn InputError(error: String) -> Element {
    rsx! {
        if !error.is_empty() {
            div { class: "flex items-center text-red-500 text-sm mb-4",
                svg {
                    class: "w-5 h-5 mr-2",
                    fill: "none",
                    stroke: "currentColor",
                    xmlns: "http://www.w3.org/2000/svg",
                    view_box: "0 0 24 24",
                    path {
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                        stroke_width: "2",
                        d: "M12 9v2m0 4h.01M12 2a10 10 0 100 20 10 10 0 000-20z",
                    }
                }
                span { "{error}" }
            }
        }
    }
}

#[component]
pub(crate) fn SavingIcon(saving: bool, error: String) -> Element {
    rsx! {
        if !error.is_empty() {
            div { title: error,
                svg {
                    class: "w-5 h-5 text-red-600",
                    xmlns: "http://www.w3.org/2000/svg",
                    fill: "none",
                    view_box: "0 0 24 24",
                    stroke_width: "2",
                    stroke: "currentColor",

                    circle {
                        cx: "12",
                        cy: "12",
                        r: "10",
                        stroke: "currentColor",
                        stroke_width: "2",
                    }
                    line {
                        x1: "12",
                        y1: "8",
                        x2: "12",
                        y2: "12",
                        stroke: "currentColor",
                        stroke_width: "2",
                        stroke_linecap: "round",
                    }
                    line {
                        x1: "12",
                        y1: "16",
                        x2: "12",
                        y2: "16",
                        stroke: "currentColor",
                        stroke_width: "2",
                        stroke_linecap: "round",
                    }
                }
            
            }
        } else if saving {
            div {
                class: "loader w-5 h-5 border-2 border-blue-600 border-t-transparent rounded-full animate-spin",
                title: "Saving in progress...",
            }
        }
    }
}

pub(crate) fn debounce<I, F>(value_signal: Signal<I>, mut running_signal: Signal<bool>, callback: F)
where
    I: Clone + PartialEq + 'static,
    F: Fn(I) + 'static,
{
    let value_on_creation = value_signal.read().clone();
    spawn(async move {
        running_signal.set(true);
        TimeoutFuture::new(500).await;
        let value_now = value_signal.read().clone();
        if value_now == value_on_creation {
            callback(value_now);
            running_signal.set(false);
        }
    });
}
