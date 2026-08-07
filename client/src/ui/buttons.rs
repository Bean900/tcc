use std::{future::Future, pin::Pin, sync::Arc};

use dioxus::prelude::*;
use uuid::Uuid;

// Token für einheitliches Focus-State Design einbinden
use crate::ui::tokens::FOCUS_RING;

// ─────────────────────────────────────────────
//  Button Styles & Variants
// ─────────────────────────────────────────────

#[derive(Clone, PartialEq)]
pub enum ButtonColor {
    Primary,
    Secondary,
    Confirm,
    Warn,
    RedHollow,
}

const ENABLED_BUTTON_PRIMARY: &str = "bg-amber-500 text-white rounded-xl px-4 py-2.5 \
     text-sm font-semibold hover:bg-amber-600 active:scale-[0.98] \
     transition-all duration-150 cursor-pointer shadow-xs hover:shadow";

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
//  Button Props & Components
// ─────────────────────────────────────────────

#[derive(Props, Clone)]
pub struct ButtonProps {
    pub text: String,
    #[props(default)]
    pub icon: Option<Element>,
    #[props(default)]
    pub action: Option<AsyncAction>,
    #[props(default)]
    pub onclick: Option<EventHandler<MouseEvent>>,
    #[props(default)]
    pub error_signal: Option<Signal<String>>,
}

impl PartialEq for ButtonProps {
    fn eq(&self, other: &Self) -> bool {
        self.text == other.text
    }
}

#[component]
pub fn PrimaryButton(props: ButtonProps) -> Element {
    rsx! {
        CustomButton {
            color: ButtonColor::Primary,
            text: props.text.clone(),
            icon: props.icon.clone(),
            error_signal: props.error_signal.clone(),
            action: props.action.clone(),
            onclick: props.onclick.clone(),
        }
    }
}

#[component]
pub fn SecondaryButton(props: ButtonProps) -> Element {
    rsx! {
        CustomButton {
            color: ButtonColor::Secondary,
            text: props.text.clone(),
            icon: props.icon.clone(),
            error_signal: props.error_signal.clone(),
            action: props.action.clone(),
            onclick: props.onclick.clone(),
        }
    }
}

#[component]
pub fn ConfirmButton(props: ButtonProps) -> Element {
    rsx! {
        CustomButton {
            color: ButtonColor::Confirm,
            text: props.text.clone(),
            icon: props.icon.clone(),
            error_signal: props.error_signal.clone(),
            action: props.action.clone(),
            onclick: props.onclick.clone(),
        }
    }
}

#[component]
pub fn WarnButton(props: ButtonProps) -> Element {
    rsx! {
        CustomButton {
            color: ButtonColor::Warn,
            text: props.text.clone(),
            icon: props.icon.clone(),
            error_signal: props.error_signal.clone(),
            action: props.action.clone(),
            onclick: props.onclick.clone(),
        }
    }
}

#[component]
pub fn RedHollowButton(props: ButtonProps) -> Element {
    rsx! {
        CustomButton {
            color: ButtonColor::RedHollow,
            text: props.text.clone(),
            icon: props.icon.clone(),
            error_signal: props.error_signal.clone(),
            action: props.action.clone(),
            onclick: props.onclick.clone(),
        }
    }
}

#[derive(Props, Clone)]
pub struct CustomButtonProps {
    pub color: ButtonColor,
    pub text: String,
    #[props(default)]
    pub icon: Option<Element>,
    #[props(default)]
    pub action: Option<AsyncAction>,
    #[props(default)]
    pub onclick: Option<EventHandler<MouseEvent>>,
    #[props(default)]
    pub error_signal: Option<Signal<String>>,
}

impl PartialEq for CustomButtonProps {
    fn eq(&self, other: &Self) -> bool {
        self.text == other.text && self.color == other.color
    }
}

#[component]
fn CustomButton(props: CustomButtonProps) -> Element {
    let mut is_loading = use_signal(|| false);
    let on_click_function = move |event: MouseEvent| {
        if is_loading() {
            return;
        }
        if props.error_signal.map_or(false, |s| !s.read().is_empty()) {
            return;
        }
        if let Some(onclick) = &props.onclick {
            onclick.call(event);
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
        ButtonColor::Primary => ENABLED_BUTTON_PRIMARY,
        ButtonColor::Secondary => ENABLED_BUTTON_SECONDARY,
        ButtonColor::Confirm => ENABLED_BUTTON_CONFIRM,
        ButtonColor::Warn => ENABLED_BUTTON_WARN,
        ButtonColor::RedHollow => ENABLED_BUTTON_RED_HOLLOW,
    };

    let is_disabled = props.error_signal.is_some()
        && !props.error_signal.expect("Expect signal").read().is_empty();

    rsx! {
        if *is_loading.read() {
            div { role: "status", class: "flex justify-center items-center h-9",
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
                r#type: "button",
                class: if is_disabled { DISABLED_BUTTON.to_string() } else { format!(
                    "{enable_button} {FOCUS_RING} inline-flex items-center justify-center gap-2",
                ) },
                disabled: is_disabled,
                onclick: on_click_function,
                if let Some(icon) = &props.icon {
                    {icon.clone()}
                }
                "{props.text}"
            }
        }
    }
}

// ─────────────────────────────────────────────
//  Toolbar & Controls Buttons
// ─────────────────────────────────────────────

#[derive(Props, Clone)]
pub struct ToolbarButtonProps {
    pub text: String,
    pub is_active: bool,
    pub onclick: EventHandler<MouseEvent>,
}

impl PartialEq for ToolbarButtonProps {
    fn eq(&self, other: &Self) -> bool {
        self.text == other.text && self.is_active == other.is_active
    }
}

#[component]
pub fn ToolbarButton(props: ToolbarButtonProps) -> Element {
    let active_style = "px-3 py-1.5 text-xs font-semibold rounded-lg bg-white text-amber-900 shadow-xs border border-amber-200/50 cursor-pointer transition-all duration-150";
    let inactive_style = "px-3 py-1.5 text-xs font-medium rounded-lg text-zinc-600 hover:text-amber-800 cursor-pointer transition-all duration-150";

    rsx! {
        button {
            r#type: "button",
            class: if props.is_active { format!("{active_style} {FOCUS_RING}") } else { format!("{inactive_style} {FOCUS_RING}") },
            onclick: move |e| props.onclick.call(e),
            "{props.text}"
        }
    }
}

// ─────────────────────────────────────────────
//  Card Grid Action Buttons
// ─────────────────────────────────────────────

#[derive(Props, Clone)]
pub struct DashedActionButtonProps {
    pub text: String,
    #[props(default)]
    pub icon_text: Option<String>,
    pub onclick: EventHandler<MouseEvent>,
}

impl PartialEq for DashedActionButtonProps {
    fn eq(&self, other: &Self) -> bool {
        self.text == other.text && self.icon_text == other.icon_text
    }
}

#[component]
pub fn DashedActionButton(props: DashedActionButtonProps) -> Element {
    let icon = props.icon_text.unwrap_or_else(|| "+".to_string());
    rsx! {
        button {
            r#type: "button",
            class: "flex flex-col items-center justify-center gap-2 min-h-[148px] h-full w-full p-4 rounded-2xl border-2 border-dashed border-amber-300/80 bg-amber-50/20 text-amber-600 hover:text-amber-700 hover:border-amber-400 hover:bg-amber-50/60 transition-all duration-200 cursor-pointer group {FOCUS_RING}",
            onclick: move |e| props.onclick.call(e),
            div { class: "w-10 h-10 rounded-full bg-amber-100 text-amber-700 flex items-center justify-center font-bold text-xl group-hover:scale-110 transition-transform",
                "{icon}"
            }
            span { class: "text-sm font-semibold", "{props.text}" }
        }
    }
}

// ─────────────────────────────────────────────
//  Close Button
// ─────────────────────────────────────────────

#[component]
pub fn CloseButton(onclick: EventHandler<MouseEvent>) -> Element {
    rsx! {
        button {
            r#type: "button",
            class: "absolute top-3 right-3 p-1.5 rounded-xl text-zinc-400 hover:text-zinc-600 hover:bg-zinc-100 transition-colors duration-150 cursor-pointer {FOCUS_RING}",
            aria_label: "Close",
            onclick: move |event| {
                onclick.call(event);
            },
            svg {
                class: "w-5 h-5",
                stroke: "currentColor",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
                xmlns: "http://www.w3.org/2000/svg",
                view_box: "0 0 24 24",
                path { d: "M6 18L18 6M6 6l12 12" }
            }
        }
    }
}

// ─────────────────────────────────────────────
//  Delete Button
// ─────────────────────────────────────────────

#[derive(Props, Clone)]
pub struct DeleteButtonProps {
    pub id: Uuid,
    #[props(default)]
    pub action: Option<AsyncAction>,
    #[props(default)]
    pub error_signal: Option<Signal<String>>,
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
pub fn DeleteButton(props: DeleteButtonProps) -> Element {
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
            div { role: "status", class: "flex justify-center items-center h-9",
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
                r#type: "button",
                class: "{ENABLED_BUTTON_RED_HOLLOW} {FOCUS_RING}",
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