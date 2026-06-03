mod callback;
mod dashboard;
mod details;
mod legal;

use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

pub use callback::Callback;
pub use dashboard::Dashboard;
pub use details::calculate::Calculate;
pub use details::courses::Courses;
pub use details::overview::Overview;
pub use details::plan::Plan;
pub use details::share_team::ShareRegisterPage;
pub use details::startend::StartEnd;
pub use details::teams::Teams;
pub use details::Menu;
pub use legal::CookieBanner;
pub use legal::CookieSettings;
pub use legal::Impressum;
pub use legal::Privacy;

use dioxus::prelude::*;
use dioxus::signals::Signal;
use gloo_timers::future::TimeoutFuture;
use uuid::Uuid;

// ─────────────────────────────────────────────
//  Button style constants
//
//  All buttons share the same shape tokens (rounded-xl, px-4 py-2, text-sm
//  font-medium) so the toolbar always looks like a coherent family.
//
//  Confirm  → amber  #D67229 / #C66741  (primary action)
//  Secondary→ warm zinc-100 / zinc-200  (neutral secondary)
//  Warn     → red-500 / red-600         (destructive)
//  RedHollow→ red outline               (soft destructive)
//  Disabled → amber-100 / amber-400     (warm, never cold gray)
// ─────────────────────────────────────────────

const DISABLED_BUTTON: &str = "bg-amber-100 text-amber-400 rounded-xl px-4 py-2 \
     text-sm font-medium cursor-not-allowed";

const ENABLED_BUTTON_SECONDARY: &str = "bg-zinc-100 text-zinc-700 rounded-xl px-4 py-2 \
     text-sm font-medium hover:bg-zinc-200 \
     transition-colors duration-150 cursor-pointer";

const ENABLED_BUTTON_CONFIRM: &str = "bg-[#D67229] text-white rounded-xl px-4 py-2 \
     text-sm font-medium hover:bg-[#C66741] \
     transition-colors duration-150 cursor-pointer";

const ENABLED_BUTTON_WARN: &str = "bg-red-500 text-white rounded-xl px-4 py-2 \
     text-sm font-medium hover:bg-red-600 \
     transition-colors duration-150 cursor-pointer";

const ENABLED_BUTTON_RED_HOLLOW: &str = "border border-red-400 text-red-600 rounded-xl px-4 py-2 \
     text-sm font-medium hover:bg-red-50 \
     transition-colors duration-150 cursor-pointer";

// ─────────────────────────────────────────────
//  Input style constants
//
//  Normal  → amber-200 border, amber-50/40 bg, amber focus ring
//  Error   → red-300 border, red-50/40 bg, red focus ring
//  mb-3 instead of mb-4 for tighter vertical rhythm
// ─────────────────────────────────────────────

const INPUT_NORMAL: &str = "w-full px-3 py-2 border border-amber-200 rounded-xl bg-amber-50/40 \
     text-sm text-zinc-800 placeholder-zinc-400 \
     focus:outline-none focus:ring-2 focus:ring-amber-400/40 focus:border-amber-400 \
     transition-colors duration-150 mb-3";

const INPUT_ERROR: &str = "w-full px-3 py-2 border border-red-300 rounded-xl bg-red-50/40 \
     text-sm text-red-700 placeholder-red-300 \
     focus:outline-none focus:ring-2 focus:ring-red-400/40 focus:border-red-400 \
     transition-colors duration-150 mb-3";

// ─────────────────────────────────────────────
//  Async helper types
// ─────────────────────────────────────────────

pub type AsyncAction = Arc<dyn Fn() -> Pin<Box<dyn Future<Output = ()>>>>;

#[macro_export]
macro_rules! async_action {
    ($logic:expr) => {
        std::sync::Arc::new(move || {
            let fut = async move { $logic };
            let boxed = std::boxed::Box::pin(fut);
            boxed as std::pin::Pin<Box<dyn std::future::Future<Output = ()>>>
        }) as AsyncAction
    };
}

// ─────────────────────────────────────────────
//  Buttons
// ─────────────────────────────────────────────

#[derive(Clone, PartialEq)]
enum ButtonColor {
    Secondary,
    Confirm,
    Warn,
    RedHollow,
}

#[derive(Props, Clone)]
pub struct ButtonProps {
    text: String,
    #[props(default)]
    action: Option<AsyncAction>,
    #[props(default)]
    error_signal: Option<Signal<String>>,
}

impl PartialEq for ButtonProps {
    fn eq(&self, other: &Self) -> bool {
        self.text == other.text
    }
}

#[component]
pub(crate) fn SecondaryButton(props: ButtonProps) -> Element {
    rsx! {
        CustomButton {
            color: ButtonColor::Secondary,
            text: props.text.clone(),
            error_signal: props.error_signal.clone(),
            action: props.action.clone(),
        }
    }
}

#[component]
pub(crate) fn ConfirmButton(props: ButtonProps) -> Element {
    rsx! {
        CustomButton {
            color: ButtonColor::Confirm,
            text: props.text.clone(),
            error_signal: props.error_signal.clone(),
            action: props.action.clone(),
        }
    }
}

#[component]
pub(crate) fn WarnButton(props: ButtonProps) -> Element {
    rsx! {
        CustomButton {
            color: ButtonColor::Warn,
            text: props.text.clone(),
            error_signal: props.error_signal.clone(),
            action: props.action.clone(),
        }
    }
}

#[component]
pub(crate) fn RedHollowButton(props: ButtonProps) -> Element {
    rsx! {
        CustomButton {
            color: ButtonColor::RedHollow,
            text: props.text.clone(),
            error_signal: props.error_signal.clone(),
            action: props.action.clone(),
        }
    }
}

#[derive(Props, Clone)]
pub struct CustomButtonProps {
    color: ButtonColor,
    text: String,
    #[props(default)]
    action: Option<AsyncAction>,
    #[props(default)]
    error_signal: Option<Signal<String>>,
}

impl PartialEq for CustomButtonProps {
    fn eq(&self, other: &Self) -> bool {
        self.text == other.text && self.color == other.color
    }
}

#[component]
fn CustomButton(props: CustomButtonProps) -> Element {
    let mut is_loading = use_signal(|| false);
    let on_click_function = move |_| {
        if is_loading() || props.action.is_none() {
            return;
        }
        if props.error_signal.map_or(false, |s| !s.read().is_empty()) {
            return;
        }
        if let Some(action_fn) = &props.action {
            is_loading.set(true);
            let action_fn = action_fn.clone();
            spawn(async move {
                (action_fn)().await;
                is_loading.set(false);
            });
        }
    };

    let enable_button = match props.color {
        ButtonColor::Secondary => ENABLED_BUTTON_SECONDARY,
        ButtonColor::Confirm => ENABLED_BUTTON_CONFIRM,
        ButtonColor::Warn => ENABLED_BUTTON_WARN,
        ButtonColor::RedHollow => ENABLED_BUTTON_RED_HOLLOW,
    };

    rsx! {
        if *is_loading.read() {
            // Amber spinner – matches ConfirmButton color
            div {
                role: "status",
                class: "flex justify-center items-center h-9",
                svg {
                    class: "w-6 h-6 text-amber-100 animate-spin fill-[#D67229]",
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
                class: if props.error_signal.is_some()
                    && !props.error_signal.expect("Expect signal").read().is_empty()
                    { DISABLED_BUTTON } else { enable_button },
                disabled: props.error_signal.is_some()
                    && !props.error_signal.expect("Expect signal").read().is_empty(),
                onclick: on_click_function,
                "{props.text}"
            }
        }
    }
}

// ─────────────────────────────────────────────
//  Close button
// ─────────────────────────────────────────────

#[component]
fn CloseButton(onclick: EventHandler<MouseEvent>) -> Element {
    rsx! {
        button {
            class: "absolute top-3 right-3 p-1 rounded-lg \
                    text-zinc-400 hover:text-zinc-600 hover:bg-zinc-100 \
                    transition-colors duration-150 cursor-pointer",
            onclick: move |event| { onclick.call(event); },
            svg {
                class: "w-5 h-5",
                stroke: "currentColor",
                stroke_width: "2",
                stroke_linecap: "round",
                xmlns: "http://www.w3.org/2000/svg",
                view_box: "0 0 24 24",
                path { d: "M6 18L18 6M6 6l12 12" }
            }
        }
    }
}

// ─────────────────────────────────────────────
//  Delete button
// ─────────────────────────────────────────────

#[derive(Props, Clone)]
pub struct DeleteButtonProps {
    id: Uuid,
    #[props(default)]
    action: Option<AsyncAction>,
    #[props(default)]
    error_signal: Option<Signal<String>>,
}

impl DeleteButtonProps {
    pub fn new(action: AsyncAction, error_signal: Option<Signal<String>>) -> Element {
        rsx! {
            DeleteButton { id: Uuid::new_v4(), action: Some(action), error_signal }
        }
    }
}

impl PartialEq for DeleteButtonProps {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

#[component]
pub(crate) fn DeleteButton(props: DeleteButtonProps) -> Element {
    let mut is_loading = use_signal(|| false);
    let on_click_function = move |_| {
        if is_loading() || props.action.is_none() {
            return;
        }
        if props.error_signal.map_or(false, |s| !s.read().is_empty()) {
            return;
        }
        if let Some(action_fn) = &props.action {
            is_loading.set(true);
            let action_fn = action_fn.clone();
            spawn(async move {
                (action_fn)().await;
                is_loading.set(false);
            });
        }
    };

    rsx! {
        if *is_loading.read() {
            // Red spinner – signals destructive action in progress
            div {
                role: "status",
                class: "flex justify-center items-center h-9",
                svg {
                    class: "w-6 h-6 text-red-100 animate-spin fill-red-500",
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
                class: ENABLED_BUTTON_RED_HOLLOW,
                onclick: on_click_function,
                svg {
                    xmlns: "http://www.w3.org/2000/svg",
                    fill: "none",
                    view_box: "0 0 24 24",
                    stroke_width: "2",
                    stroke: "currentColor",
                    class: "w-5 h-5",
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

// ─────────────────────────────────────────────
//  Input components
//  All share INPUT_NORMAL / INPUT_ERROR constants above.
// ─────────────────────────────────────────────

#[component]
pub(crate) fn Input(
    place_holer: Option<String>,
    value: String,
    is_error: Option<bool>,
    oninput: EventHandler<dioxus::prelude::Event<FormData>>,
) -> Element {
    rsx! {
        input {
            class: if is_error.is_some_and(|e| e) { INPUT_ERROR } else { INPUT_NORMAL },
            r#type: "text",
            placeholder: if place_holer.is_some() { place_holer.expect("Expected placeholder") } else { "" },
            value,
            oninput: move |e| { oninput.call(e); },
        }
    }
}

#[component]
pub(crate) fn InputMultirow(
    place_holer: Option<String>,
    value: String,
    is_error: Option<bool>,
    oninput: EventHandler<dioxus::prelude::Event<FormData>>,
) -> Element {
    rsx! {
        textarea {
            class: if is_error.is_some_and(|e| e) { INPUT_ERROR } else { INPUT_NORMAL },
            placeholder: if place_holer.is_some() { place_holer.expect("Expected placeholder") } else { "" },
            rows: "3",
            value,
            oninput: move |e| { oninput.call(e); },
        }
    }
}

#[component]
pub(crate) fn InputNumber(
    place_holer: Option<String>,
    value: String,
    is_error: Option<bool>,
    oninput: EventHandler<dioxus::prelude::Event<FormData>>,
) -> Element {
    rsx! {
        input {
            class: if is_error.is_some_and(|e| e) { INPUT_ERROR } else { INPUT_NORMAL },
            r#type: "number",
            placeholder: if place_holer.is_some() { place_holer.expect("Expected placeholder") } else { "" },
            value,
            oninput: move |e| { oninput.call(e); },
        }
    }
}

#[component]
pub(crate) fn InputPhoneNumber(
    place_holer: Option<String>,
    value: String,
    is_error: Option<bool>,
    oninput: EventHandler<dioxus::prelude::Event<FormData>>,
) -> Element {
    rsx! {
        input {
            class: if is_error.is_some_and(|e| e) { INPUT_ERROR } else { INPUT_NORMAL },
            r#type: "tel",
            placeholder: if place_holer.is_some() { place_holer.expect("Expected placeholder") } else { "" },
            value,
            oninput: move |e| { oninput.call(e); },
        }
    }
}

#[component]
pub(crate) fn InputTime(
    place_holer: Option<String>,
    value: String,
    is_error: Option<bool>,
    oninput: EventHandler<dioxus::prelude::Event<FormData>>,
) -> Element {
    rsx! {
        input {
            class: if is_error.is_some_and(|e| e) { INPUT_ERROR } else { INPUT_NORMAL },
            r#type: "time",
            placeholder: if place_holer.is_some() { place_holer.expect("Expected placeholder") } else { "" },
            value,
            oninput: move |e| { oninput.call(e); },
        }
    }
}

#[component]
pub(crate) fn InputDate(
    place_holer: Option<String>,
    value: String,
    is_error: Option<bool>,
    oninput: EventHandler<dioxus::prelude::Event<FormData>>,
) -> Element {
    rsx! {
        input {
            class: if is_error.is_some_and(|e| e) { INPUT_ERROR } else { INPUT_NORMAL },
            r#type: "date",
            placeholder: if place_holer.is_some() { place_holer.expect("Expected placeholder") } else { "" },
            value,
            oninput: move |e| { oninput.call(e); },
        }
    }
}

// ─────────────────────────────────────────────
//  Input error message
// ─────────────────────────────────────────────

#[component]
pub(crate) fn InputError(error: String) -> Element {
    rsx! {
        if !error.is_empty() {
            div { class: "flex items-center gap-1.5 text-red-500 text-xs mb-3 -mt-1",
                ErrorSVG {}
                span { "{error}" }
            }
        }
    }
}

// ─────────────────────────────────────────────
//  Saving indicator
// ─────────────────────────────────────────────

#[component]
pub(crate) fn SavingIcon(saving: bool, error: String) -> Element {
    rsx! {
        if !error.is_empty() {
            div { title: error, ErrorSVG {} }
        } else if saving {
            div {
                class: "w-5 h-5 border-2 border-amber-200 border-t-[#D67229] \
                        rounded-full animate-spin",
                title: "Saving…",
            }
        }
    }
}

// ─────────────────────────────────────────────
//  Typography
// ─────────────────────────────────────────────

#[component]
pub(crate) fn Headline1(headline: String) -> Element {
    rsx!(
        div { class: "text-zinc-900 font-sans leading-tight",
            h1 { class: "text-3xl font-bold mb-2", "{headline}" }
        }
    )
}

#[component]
pub(crate) fn Headline2(headline: String) -> Element {
    rsx!(
        div { class: "font-sans leading-tight",
            h2 { class: "text-xl font-semibold text-[#70513E] mb-1", "{headline}" }
        }
    )
}

#[component]
pub(crate) fn Headline3(headline: String) -> Element {
    rsx!(
        label { class: "block text-sm font-semibold text-[#70513E] ml-2", "{headline}" }
    )
}

#[component]
pub(crate) fn Text(text: String) -> Element {
    rsx!(
        div { class: "text-zinc-700 font-sans leading-relaxed",
            p { class: "text-sm mb-3", {text} }
        }
    )
}

// ─────────────────────────────────────────────
//  SVG icons
// ─────────────────────────────────────────────

#[component]
pub(crate) fn StartSVG() -> Element {
    rsx!(
        svg {
            class: "w-5 h-5",
            xmlns: "http://www.w3.org/2000/svg",
            fill: "none",
            view_box: "0 0 24 24",
            stroke_width: "2",
            stroke: "#D67229",
            path {
                stroke_linecap: "round",
                stroke_linejoin: "round",
                d: "M12 2l3.09 6.26L22 9.27l-5 4.87L18.18 22 12 18.27 5.82 22 7 14.14l-5-4.87 6.91-1.01L12 2z",
            }
        }
    )
}

#[component]
pub(crate) fn EndSVG() -> Element {
    rsx!(
        svg {
            class: "w-5 h-5",
            xmlns: "http://www.w3.org/2000/svg",
            fill: "none",
            view_box: "0 0 24 24",
            stroke_width: "2",
            stroke: "#D67229",
            path {
                stroke_linecap: "round",
                stroke_linejoin: "round",
                d: "M4 2v20m0-18h10l-2 4 2 4H4",
            }
            circle {
                cx: "4",
                cy: "20",
                r: "1",
                fill: "#D67229",
            }
        }
    )
}

#[component]
pub(crate) fn DownloadSVG() -> Element {
    rsx!(
        svg {
            class: "w-5 h-5",
            fill: "none",
            stroke: "currentColor",
            stroke_width: "2",
            view_box: "0 0 24 24",
            xmlns: "http://www.w3.org/2000/svg",
            path { d: "M4 16v2a2 2 0 002 2h12a2 2 0 002-2v-2M7 10l5 5 5-5M12 15V3" }
        }
    )
}

#[component]
pub(crate) fn AddressSVG() -> Element {
    rsx!(
        svg {
            class: "w-4 h-4 shrink-0",
            xmlns: "http://www.w3.org/2000/svg",
            fill: "#D67229",
            view_box: "0 0 24 24",
            path { d: "M12 2C8.134 2 5 5.134 5 9c0 4.418 7 13 7 13s7-8.582 7-13c0-3.866-3.134-7-7-7zm0 9.5a2.5 2.5 0 1 1 0-5 2.5 2.5 0 0 1 0 5z" }
        }
    )
}

#[component]
pub(crate) fn WarningSVG() -> Element {
    rsx!(
        svg {
            class: "w-5 h-5",
            xmlns: "http://www.w3.org/2000/svg",
            fill: "none",
            view_box: "0 0 24 24",
            stroke: "#D67229",
            path {
                stroke_linecap: "round",
                stroke_linejoin: "round",
                stroke_width: "2",
                d: "M10.29 3.86L1.82 18a2 2 0 001.71 3h16.94a2 2 0 001.71-3L13.71 3.86a2 2 0 00-3.42 0zM12 9v4m0 4h.01",
            }
        }
    )
}

#[component]
pub(crate) fn ErrorSVG() -> Element {
    rsx!(
        svg {
            class: "w-4 h-4 text-red-500 shrink-0",
            xmlns: "http://www.w3.org/2000/svg",
            fill: "none",
            view_box: "0 0 24 24",
            stroke_width: "2",
            stroke: "currentColor",
            circle { cx: "12", cy: "12", r: "10", stroke: "currentColor", stroke_width: "2" }
            line { x1: "12", y1: "8", x2: "12", y2: "12", stroke: "currentColor", stroke_width: "2", stroke_linecap: "round" }
            line { x1: "12", y1: "16", x2: "12", y2: "16", stroke: "currentColor", stroke_width: "2", stroke_linecap: "round" }
        }
    )
}

#[component]
pub(crate) fn InfoSVG() -> Element {
    rsx!(
        svg {
            class: "w-5 h-5 text-amber-500",
            xmlns: "http://www.w3.org/2000/svg",
            fill: "none",
            view_box: "0 0 24 24",
            stroke_width: "2",
            stroke: "currentColor",
            path {
                stroke_linecap: "round",
                stroke_linejoin: "round",
                d: "M13 16h-1v-4h-1m1-4h.01M12 2a10 10 0 100 20 10 10 0 000-20z",
            }
        }
    )
}

#[component]
pub(crate) fn CourseSVG() -> Element {
    rsx!(
        svg {
            class: "w-5 h-5",
            xmlns: "http://www.w3.org/2000/svg",
            fill: "none",
            view_box: "0 0 24 24",
            stroke: "#D67229",
            path {
                stroke_linecap: "round",
                stroke_linejoin: "round",
                stroke_width: "2",
                d: "M4 6h1v12H4zM7 6h1v12H7zM21 12a9 9 0 11-18 0 9 9 0 0118 0zm-6 0a3 3 0 11-6 0 3 3 0 016 0z",
            }
        }
    )
}

#[component]
pub(crate) fn TimeSVG() -> Element {
    rsx!(
        svg {
            class: "w-5 h-5",
            xmlns: "http://www.w3.org/2000/svg",
            fill: "none",
            stroke: "#D67229",
            path {
                stroke_linecap: "round",
                stroke_linejoin: "round",
                stroke_width: "2",
                d: "M12 8v4l3 3M12 2a10 10 0 100 20 10 10 0 000-20z",
            }
        }
    )
}

#[component]
pub(crate) fn PhoneSVG() -> Element {
    rsx!(
        svg {
            xmlns: "http://www.w3.org/2000/svg",
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "#D67229",
            stroke_width: "2",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            class: "w-5 h-5",
            path { d: "M22 16.92v3a2 2 0 0 1-2.18 2A19.86 19.86 0 0 1 3.1 5.18 2 2 0 0 1 5 3h3a2 2 0 0 1 2 1.72c.12.81.31 1.6.57 2.35a2 2 0 0 1-.45 2.11L9.03 10.91a16 16 0 0 0 6.06 6.06l1.73-1.09a2 2 0 0 1 2.11-.45c.75.26 1.54.45 2.35.57a2 2 0 0 1 1.72 2z" }
        }
    )
}

#[component]
pub(crate) fn GroupSVG() -> Element {
    rsx!(
        svg {
            class: "w-5 h-5",
            xmlns: "http://www.w3.org/2000/svg",
            fill: "none",
            stroke: "#D67229",
            path {
                stroke_linecap: "round",
                stroke_linejoin: "round",
                stroke_width: "2",
                d: "M17 20h5v-2a4 4 0 00-5-4m-4-2a4 4 0 100-8 4 4 0 000 8zm6 6v2m-6-2v2m-6-2v2H2v-2a4 4 0 015-4m0 0a4 4 0 014 4",
            }
        }
    )
}

#[component]
pub(crate) fn PersonSVG() -> Element {
    rsx!(
        svg {
            class: "w-5 h-5",
            xmlns: "http://www.w3.org/2000/svg",
            fill: "none",
            stroke: "#D67229",
            path {
                stroke_linecap: "round",
                stroke_linejoin: "round",
                stroke_width: "2",
                d: "M12 4.5a3.5 3.5 0 110 7 3.5 3.5 0 010-7zM12 14.5c-4.418 0-8 1.79-8 4v1h16v-1c0-2.21-3.582-4-8-4z",
            }
        }
    )
}

// ─────────────────────────────────────────────
//  Debounce utility (unchanged)
// ─────────────────────────────────────────────

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
