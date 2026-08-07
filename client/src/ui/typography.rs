use dioxus::prelude::*;

// ─────────────────────────────────────────────
//  Typography Components
// ─────────────────────────────────────────────

/// Hauptüberschrift (H1) - Für Seitentitel, Dashboards oder Hauptseiten.
#[component]
pub fn Headline1(
    headline: String,
    #[props(default)] subtitle: Option<String>,
    #[props(default)] class: Option<String>,
) -> Element {
    let custom_class = class.unwrap_or_default();
    rsx! {
        div { class: "font-sans leading-tight {custom_class}",
            h1 { class: "text-2xl sm:text-3xl font-bold tracking-tight text-zinc-900",
                "{headline}"
            }
            if let Some(sub) = subtitle {
                p { class: "mt-1 text-sm font-normal text-zinc-500", "{sub}" }
            }
        }
    }
}

/// Sektionsüberschrift (H2) - Für Modals, Cards oder Hauptabschnitte.
#[component]
pub fn Headline2(
    headline: String,
    #[props(default)] subtitle: Option<String>,
    #[props(default)] class: Option<String>,
) -> Element {
    let custom_class = class.unwrap_or_default();
    rsx! {
        div { class: "font-sans leading-tight {custom_class}",
            h2 { class: "text-lg sm:text-xl font-semibold text-[#70513E] tracking-tight",
                "{headline}"
            }
            if let Some(sub) = subtitle {
                p { class: "mt-0.5 text-xs text-zinc-500", "{sub}" }
            }
        }
    }
}

/// Untergruppen-Überschrift (H3) - Für Sub-Abschnitte oder Gruppen-Titel.
#[component]
pub fn Headline3(
    headline: String,
    #[props(default)] class: Option<String>,
) -> Element {
    let custom_class = class.unwrap_or_default();
    rsx! {
        h3 { class: "text-base font-semibold text-[#70513E] tracking-tight {custom_class}",
            "{headline}"
        }
    }
}

/// Standard Fließtext / Paragraph.
#[component]
pub fn Text(
    text: String,
    #[props(default)] class: Option<String>,
) -> Element {
    let custom_class = class.unwrap_or_default();
    rsx! {
        p { class: "text-sm text-zinc-700 font-sans leading-relaxed {custom_class}",
            "{text}"
        }
    }
}

/// Dezent gehaltener Hilfetext oder Untertext für Modals / Erklärungen.
#[component]
pub fn SubText(
    text: String,
    #[props(default)] class: Option<String>,
) -> Element {
    let custom_class = class.unwrap_or_default();
    rsx! {
        p { class: "text-xs text-zinc-500 font-sans leading-normal {custom_class}",
            "{text}"
        }
    }
}

/// Standardisierte Formular-Beschriftung (Label).
#[component]
pub fn FieldLabel(
    text: String,
    #[props(default)] for_id: Option<String>,
    #[props(default)] class: Option<String>,
) -> Element {
    let custom_class = class.unwrap_or_default();
    rsx! {
        label {
            r#for: for_id,
            class: "block text-[11px] font-semibold tracking-wider uppercase text-amber-800/80 mb-1.5 {custom_class}",
            "{text}"
        }
    }
}

/// Kompakter Text für Badges, Zähler oder Zeitstempel.
#[component]
pub fn CaptionText(
    text: String,
    #[props(default)] class: Option<String>,
) -> Element {
    let custom_class = class.unwrap_or_default();
    rsx! {
        span { class: "text-xs font-medium text-zinc-400 font-sans {custom_class}", "{text}" }
    }
}