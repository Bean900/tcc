use dioxus::prelude::*;

use crate::{
    config::Config,
    side::legal::{LegalPageShell, LegalSection},
    state::ConsentState,
};

#[component]
pub fn CookieSettings() -> Element {
    let config = use_context::<Signal<Option<Config>>>().read().clone();
    let c = if let Some(c) = config {
        c.legal
    } else {
        return rsx! {};
    };

    let mut consent = use_context::<Signal<ConsentState>>();

    let version = c.consent_version.clone();
    let consent_enabled = c.cookie_consent_enabled;
    let has_analytics = c.has_analytics();
    let current_analytics = consent.read().analytics_allowed;
    let consent_given = consent.read().consent_given;

    rsx! {
        LegalPageShell { title: "Cookie-Einstellungen".to_string(),
            if consent_given && consent_enabled {
                div {
                    class: "flex items-center gap-2.5 px-4 py-3 \
                            rounded-xl bg-amber-50 border border-amber-200/70 text-sm",
                    svg {
                        class: "w-4 h-4 shrink-0 text-amber-600",
                        fill: "none", xmlns: "http://www.w3.org/2000/svg",
                        view_box: "0 0 24 24", stroke: "currentColor", stroke_width: "2",
                        path { stroke_linecap: "round", stroke_linejoin: "round",
                            d: "M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z"
                        }
                    }
                    span { class: "text-amber-800 text-xs",
                        "Ihre Einstellungen wurden gespeichert."
                    }
                }
            }

            LegalSection { title: "Über Cookies".to_string(),
                p {
                    "Diese Anwendung verwendet Technologien zur Datenspeicherung im Browser \
                     (Cookies und Web Storage). Nachfolgend finden Sie eine Übersicht der \
                     verwendeten Kategorien und die Möglichkeit, Ihre Einstellungen anzupassen."
                }
            }

            LegalSection { title: "Notwendige Cookies".to_string(),
                div { class: "flex items-start justify-between gap-4",
                    div { class: "flex-1 space-y-2",
                        p {
                            "Technisch notwendige Cookies ermöglichen grundlegende Funktionen \
                             der Anwendung. Ohne diese Cookies kann die App nicht korrekt funktionieren."
                        }
                        div { class: "space-y-1.5",
                            CookieEntry {
                                name: "Authentifizierungs-Token".to_string(),
                                storage: "localStorage".to_string(),
                                purpose: "Speichert Ihre Anmeldedaten für die aktuelle Sitzung".to_string(),
                                duration: "Sitzungsdauer / bis zu 24 Stunden".to_string(),
                            }
                            CookieEntry {
                                name: "App-Daten (Projektdaten)".to_string(),
                                storage: "localStorage".to_string(),
                                purpose: "Zwischenspeicherung von Projektdaten zur Offline-Nutzung".to_string(),
                                duration: "Bis zur Abmeldung oder manuellen Löschung".to_string(),
                            }
                            if consent_enabled {
                                CookieEntry {
                                    name: "Cookie-Einwilligung".to_string(),
                                    storage: "localStorage (cook_run_consent)".to_string(),
                                    purpose: "Speichert Ihre Cookie-Präferenzen".to_string(),
                                    duration: "12 Monate".to_string(),
                                }
                            }
                        }
                    }
                    if consent_enabled {
                        div { class: "shrink-0 flex items-center gap-2 mt-0.5",
                            span { class: "text-xs text-zinc-400", "Immer aktiv" }
                            div {
                                class: "w-10 h-5 rounded-full bg-amber-300/60 \
                                        flex items-center px-0.5 cursor-not-allowed",
                                title: "Notwendige Cookies können nicht deaktiviert werden",
                                div { class: "w-4 h-4 rounded-full bg-amber-600/70 ml-auto" }
                            }
                        }
                    }
                }
            }

            if has_analytics && consent_enabled {
                LegalSection { title: "Analyse-Cookies".to_string(),
                    div { class: "flex items-start justify-between gap-4",
                        div { class: "flex-1 space-y-2",
                            p {
                                "Analyse-Cookies helfen uns zu verstehen, wie die Anwendung \
                                 genutzt wird, und ermöglichen Verbesserungen. \
                                 Diese Cookies sind optional."
                            }
                        }
                        // Toggle (aktiv)
                        div { class: "shrink-0 flex items-center gap-2 mt-0.5",
                            span { class: "text-xs text-zinc-500",
                                if current_analytics { "Aktiv" } else { "Inaktiv" }
                            }
                            button {
                                class: {
                                    let base = "w-10 h-5 rounded-full flex items-center \
                                                px-0.5 transition-colors duration-200 cursor-pointer";
                                    if current_analytics {
                                        format!("{base} bg-amber-500")
                                    } else {
                                        format!("{base} bg-zinc-300")
                                    }
                                },
                                onclick: {
                                    let v = version.clone();
                                    move |_| {
                                        let current = consent.read().clone();
                                        let updated = ConsentState {
                                            consent_given: true,
                                            timestamp: ConsentState::accept_all(&v).timestamp,
                                            version: v.clone(),
                                            analytics_allowed: !current.analytics_allowed,
                                        };
                                        updated.save();
                                        consent.set(updated);
                                    }
                                },
                                div {
                                    class: {
                                        let base = "w-4 h-4 rounded-full bg-white \
                                                    shadow-sm transition-all duration-200";
                                        if current_analytics {
                                            format!("{base} ml-auto")
                                        } else {
                                            format!("{base}")
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            if !consent_enabled {
                LegalSection { title: "Hinweis".to_string(),
                    div {
                        class: "flex items-start gap-2.5 p-3 rounded-xl \
                                bg-amber-50/60 border border-amber-200/50",
                        svg {
                            class: "w-4 h-4 shrink-0 mt-0.5 text-amber-600",
                            fill: "none", xmlns: "http://www.w3.org/2000/svg",
                            view_box: "0 0 24 24", stroke: "currentColor", stroke_width: "2",
                            path { stroke_linecap: "round", stroke_linejoin: "round",
                                d: "M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"
                            }
                        }
                        p { class: "text-xs text-amber-800 leading-relaxed",
                            "Diese Anwendung verwendet ausschließlich technisch notwendige \
                             Datenspeicherung. Eine Einwilligungsverwaltung ist daher nicht \
                             erforderlich (§ 25 Abs. 2 TTDSG)."
                        }
                    }
                }
            }

            if consent_enabled {
                div { class: "pt-2 flex flex-col sm:flex-row items-start sm:items-center \
                              justify-between gap-3 border-t border-amber-100",
                    p { class: "text-xs text-zinc-400 leading-relaxed",
                        "Sie können Ihre Einwilligung jederzeit mit Wirkung für die Zukunft \
                         widerrufen. Dies berührt nicht die Rechtmäßigkeit der bis zum \
                         Widerruf erfolgten Verarbeitung."
                    }
                    button {
                        class: "shrink-0 px-4 py-2 rounded-xl text-sm font-medium \
                                border border-red-300 text-red-600 \
                                hover:bg-red-50 transition-colors duration-150 cursor-pointer",
                        onclick: {
                            let v = version.clone();
                            move |_| {
                                consent.set(ConsentState::reset(&v));
                            }
                        },
                        "Einwilligung widerrufen"
                    }
                }
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
struct CookieEntryProps {
    name: String,
    storage: String,
    purpose: String,
    duration: String,
}

#[component]
fn CookieEntry(props: CookieEntryProps) -> Element {
    rsx! {
        div { class: "flex flex-col gap-0.5 p-2.5 rounded-lg bg-zinc-50 border border-zinc-100",
            div { class: "flex items-center gap-2",
                span { class: "text-xs font-semibold text-zinc-700", "{props.name}" }
                span {
                    class: "text-[10px] px-1.5 py-0.5 rounded-full \
                            bg-amber-100 text-amber-700 font-medium",
                    "{props.storage}"
                }
            }
            p { class: "text-xs text-zinc-500", "{props.purpose}" }
            p { class: "text-[11px] text-zinc-400",
                span { class: "font-medium", "Speicherdauer: " }
                "{props.duration}"
            }
        }
    }
}
