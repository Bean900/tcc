use dioxus::prelude::*;
use crate::ui::buttons::CloseButton;

#[derive(Props, Clone, PartialEq)]
pub struct ModalProps {
    pub title: String,
    pub on_close: EventHandler<()>,
    pub children: Element,
    #[props(default)]
    pub max_width: Option<String>,
}

/// Standardisiertes Overlay-Modal mit Abdunkelung, Zentrierung und Close-Button.
#[component]
pub fn Modal(props: ModalProps) -> Element {
    let max_w = props.max_width.unwrap_or_else(|| "max-w-lg".to_string());

    rsx! {
        div {
            class: "fixed inset-0 z-50 flex items-center justify-center p-4 bg-zinc-900/40 backdrop-blur-xs animate-in fade-in duration-200",
            onclick: move |_| props.on_close.call(()),

            div {
                class: "bg-white rounded-2xl border border-amber-100 shadow-xl w-full {max_w} overflow-hidden transform transition-all",
                onclick: move |evt| evt.stop_propagation(),

                // Modal Header
                div { class: "px-5 py-4 bg-amber-50/70 border-b border-amber-100 flex items-center justify-between gap-3",
                    h3 { class: "text-base font-semibold text-zinc-800", "{props.title}" }
                    CloseButton { onclick: move |_| props.on_close.call(()) }
                }

                // Modal Body
                div { class: "p-5 sm:p-6", {props.children} }
            }
        }
    }
}