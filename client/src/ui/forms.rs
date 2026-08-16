use dioxus::prelude::*;
use crate::ui::{icons::ErrorIcon, tokens::NATIVE_INPUT};

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
pub(crate) fn InputPhoneNumber(
    placeholder: Option<String>,
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
            r#type: "tel",
            class: "{NATIVE_INPUT} {error_class}",
            placeholder: placeholder.unwrap_or_default(),
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
            ErrorIcon {}
            span { class: "text-xs font-medium text-red-500", "{error}" }
        }
    }
}

/// Mehrzeiliges Textfeld (z. B. für Adressen, Allergien, Notizen)
#[component]
pub fn TextArea(
    #[props(default)] placeholder: Option<String>,
    value: String,
    is_error: bool,
    oninput: EventHandler<FormEvent>,
    #[props(default)] rows: Option<usize>,
) -> Element {
    let error_class = if is_error {
        "border-red-400 bg-red-50/30 focus:ring-red-400/50 focus:border-red-400 text-red-900"
    } else {
        ""
    };
    let r = rows.unwrap_or(3);

    rsx! {
        textarea {
            rows: "{r}",
            class: "{NATIVE_INPUT} h-auto py-2.5 resize-y {error_class}",
            placeholder: placeholder.unwrap_or_default(),
            value: "{value}",
            oninput: move |e| oninput.call(e),
        }
    }
}

/// Nummerneingabefeld (z. B. für Personenzahl)
#[component]
pub fn InputNumber(
    #[props(default)] placeholder: Option<String>,
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
            r#type: "number",
            min: "1",
            class: "{NATIVE_INPUT} {error_class}",
            placeholder: placeholder.unwrap_or_default(),
            value: "{value}",
            oninput: move |e| oninput.call(e),
        }
    }
}


#[component]
pub fn InputTime(
    value: String,
    oninput: EventHandler<FormEvent>,
    #[props(default)] class: Option<String>,
) -> Element {
    let custom_class = class.unwrap_or_default();
    rsx! {
        input {
            r#type: "time",
            class: "{NATIVE_INPUT} {custom_class}",
            value: "{value}",
            oninput: move |e| oninput.call(e),
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────
// BAUSTEIN 1/2: In client/src/ui/forms.rs einfügen (z. B. direkt nach InputTime).
//
// Ersetzt die bisher 7× im teams-Modul kopierte
// `input[type=checkbox]` + `label`-Kombination durch eine einheitliche
// Komponente mit zwei visuellen Varianten (Inline / Card).
// ─────────────────────────────────────────────────────────────────────────

/// Visuelle Variante des `Checkbox`-Bausteins.
#[derive(Clone, Copy, PartialEq, Default)]
pub enum CheckboxVariant {
    /// Kompakt, ohne Rahmen – für dicht stehende Options-Listen
    /// (z. B. "Required Fields").
    #[default]
    Inline,
    /// Eigene Karte mit Rahmen und Hover-Hintergrund – für einzeln
    /// hervorgehobene Umschalter (z. B. "Login required").
    Card,
}

/// Einheitliche Checkbox mit Label, in zwei Varianten (`Inline`/`Card`).
#[component]
pub fn Checkbox(
    label: String,
    checked: bool,
    onclick: EventHandler<MouseEvent>,
    #[props(default)] variant: CheckboxVariant,
    #[props(default)] title: Option<String>,
) -> Element {
    let (wrapper_class, label_class) = match variant {
        CheckboxVariant::Inline => (
            "flex items-center gap-2 cursor-pointer group",
            "text-[13px] text-zinc-600 group-hover:text-zinc-800 transition-colors",
        ),
        CheckboxVariant::Card => (
            "flex items-center gap-2.5 px-3 py-2.5 rounded-xl border border-amber-100 bg-amber-50/30 hover:bg-amber-50/60 transition-colors cursor-pointer group",
            "text-[13px] text-zinc-600 group-hover:text-zinc-800 transition-colors",
        ),
    };

    rsx! {
        label { class: wrapper_class, title: title.unwrap_or_default(),
            input {
                r#type: "checkbox",
                checked,
                class: "accent-[#D67229] w-4 h-4 rounded cursor-pointer",
                onclick: move |e| onclick.call(e),
            }
            span { class: label_class, "{label}" }
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────
// BAUSTEIN 2/2: bestehende `InputDate`-Funktion in client/src/ui/forms.rs
// ERSETZEN durch diese Version (fügt einen optionalen `class`-Prop hinzu,
// analog zu `InputTime`, das dieses Feature schon hat). Rückwärtskompatibel:
// der einzige bestehende Call-Site (overview.rs) übergibt keine `class` und
// verhält sich exakt wie zuvor.
// ─────────────────────────────────────────────────────────────────────────

#[component]
pub fn InputDate(
    value: String,
    oninput: EventHandler<FormEvent>,
    #[props(default)] class: Option<String>,
) -> Element {
    let custom_class = class.unwrap_or_default();
    rsx! {
        input {
            r#type: "date",
            class: "{NATIVE_INPUT} w-full block {custom_class}",
            value: "{value}",
            oninput: move |e| oninput.call(e),
        }
    }
}