use dioxus::prelude::*;

use crate::{config::Config, state::ConsentState, Route};

#[component]
pub fn CookieBanner() -> Element {
    let config = use_context::<Signal<Option<Config>>>().read().clone();
    let c = if let Some(c) = config {
        c.legal
    } else {
        return rsx! {};
    };

    let mut consent = use_context::<Signal<ConsentState>>();

    let should_show = {
        let s = consent.read();
        c.cookie_consent_enabled && !s.consent_given
    };

    if !should_show {
        return rsx! {};
    }

    let version = c.consent_version.clone();
    let has_analytics = c.has_analytics();

    rsx! {
        // ── Bottom-Bar ───────────────────────────────────────────────────
        div {
            class: "fixed bottom-0 left-0 right-0 z-[90] \
                    bg-[#FDFAF6]/97 backdrop-blur-sm \
                    border-t border-amber-200/60 \
                    shadow-[0_-4px_24px_rgba(0,0,0,0.07)]",

            div {
                class: "max-w-5xl mx-auto px-6 py-4 \
                        flex flex-col sm:flex-row \
                        items-start sm:items-center gap-4",

                // ── Links: Icon + Text ───────────────────────────────────
                div { class: "flex items-start gap-3 flex-1 min-w-0",
                    span {
                        class: "shrink-0 text-lg mt-0.5",
                        "🍪"
                    }
                    div {
                        p { class: "text-sm font-semibold text-zinc-800 mb-0.5",
                            "Cookie-Einstellungen"
                        }
                        p { class: "text-xs text-zinc-500 leading-relaxed",
                            if has_analytics {
                                "Neben technisch notwendigen Cookies nutzen wir optionale \
                                 Analyse-Cookies, um die App zu verbessern. \
                                 Sie können frei wählen."
                            } else {
                                "Diese App verwendet ausschließlich technisch notwendige Cookies \
                                 für Authentifizierung und Sitzungsverwaltung."
                            }
                        }
                    }
                }

                // ── Rechts: Buttons ──────────────────────────────────────
                div { class: "flex items-center gap-2 shrink-0 flex-wrap",

                    // Einstellungen-Link (dezent)
                    Link {
                        to: Route::CookieSettings {},
                        class: "px-3 py-1.5 text-xs text-zinc-500 \
                                hover:text-amber-700 underline underline-offset-2 \
                                transition-colors duration-150",
                        "Einstellungen"
                    }

                    // ── Trennlinie ───────────────────────────────────────
                    span { class: "h-5 w-px bg-amber-200/80 hidden sm:block" }

                    // Nur Notwendige (gleichwertig prominent – DSGVO-Anforderung)
                    button {
                        class: "px-4 py-2 rounded-xl text-sm font-medium \
                                bg-zinc-100 text-zinc-700 \
                                hover:bg-zinc-200 \
                                transition-colors duration-150 cursor-pointer",
                        onclick: {
                            let v = version.clone();
                            move |_| {
                                consent.set(ConsentState::accept_necessary(&v));
                            }
                        },
                        "Nur Notwendige"
                    }

                    // Akzeptieren
                    button {
                        class: "px-4 py-2 rounded-xl text-sm font-medium \
                                bg-[#D67229] text-white \
                                hover:bg-[#C66741] \
                                transition-colors duration-150 cursor-pointer",
                        onclick: {
                            let v = version.clone();
                            move |_| {
                                consent.set(ConsentState::accept_all(&v));
                            }
                        },
                        "Akzeptieren"
                    }
                }
            }
        }
    }
}
