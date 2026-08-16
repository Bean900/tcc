use dioxus::prelude::*;
use crate::ui::buttons::CloseButton;
use crate::ui::tokens::SAVE_GLOW_CSS;

#[derive(Props, Clone, PartialEq)]
pub struct ModalProps {
    pub title: String,
    pub on_close: EventHandler<()>,
    pub children: Element,
    #[props(default)]
    pub max_width: Option<String>,
    /// Akzentbalken links vom Titel statt eines schlichten `h3` – der
    /// Standard-Look für Formular-Dialoge (Add/Edit/Share Team).
    #[props(default)]
    pub accent: bool,
    /// Begrenzt das Modal auf `max-h-[90vh]`; nur der Body scrollt, Header
    /// bleibt fixiert. Für lange Formulare gedacht.
    #[props(default)]
    pub scrollable: bool,
    /// Schließt das Modal bei Klick auf den Hintergrund.
    #[props(default = true)]
    pub close_on_backdrop_click: bool,
    /// Aktiviert den grünen "Erfolgreich gespeichert"-Glow-Effekt
    /// (Border-Puls + eingefärbter Header/Akzent/Titel für 2s).
    #[props(default)]
    pub glow: bool,
}

/// Standardisiertes Overlay-Modal mit Abdunkelung, Zentrierung und Close-Button.
#[component]
pub fn Modal(props: ModalProps) -> Element {
    let max_w = props.max_width.clone().unwrap_or_else(|| "max-w-lg".to_string());

    let panel_scroll_class = if props.scrollable { "max-h-[90vh] flex flex-col" } else { "" };
    let panel_border_class = if props.glow { "border save-glow-card" } else { "border border-amber-100" };

    let header_bg_class = if props.glow {
        "save-glow-header"
    } else {
        "bg-amber-50/70 border-amber-100"
    };
    let accent_class = if props.glow {
        "w-1.5 h-5 rounded-full save-glow-accent"
    } else {
        "w-1.5 h-5 rounded-full bg-amber-400/70"
    };
    let title_class = if props.glow {
        "text-base font-semibold save-glow-title"
    } else {
        "text-base font-semibold text-zinc-800"
    };

    let body_class = if props.scrollable {
        "px-6 py-5 overflow-y-auto flex-1"
    } else {
        "p-5 sm:p-6"
    };

    rsx! {
        if props.glow {
            style { dangerous_inner_html: SAVE_GLOW_CSS }
        }

        div {
            class: "fixed inset-0 z-50 flex items-center justify-center p-4 bg-zinc-900/40 backdrop-blur-xs animate-in fade-in duration-200",
            onclick: move |_| {
                if props.close_on_backdrop_click {
                    props.on_close.call(());
                }
            },

            div {
                class: "bg-white rounded-2xl {panel_border_class} shadow-xl w-full {max_w} {panel_scroll_class} overflow-hidden transform transition-all",
                onclick: move |evt| evt.stop_propagation(),

                // Modal Header
                div { class: "px-6 py-4 {header_bg_class} border-b flex items-center justify-between gap-2.5 shrink-0 relative",
                    if props.accent {
                        div { class: "flex items-center gap-2.5",
                            div { class: "{accent_class}" }
                            span { class: "{title_class}", "{props.title}" }
                        }
                    } else {
                        h3 { class: "text-base font-semibold text-zinc-800", "{props.title}" }
                    }
                    CloseButton { onclick: move |_| props.on_close.call(()) }
                }

                // Modal Body
                div { class: "{body_class}", {props.children} }
            }
        }
    }
}