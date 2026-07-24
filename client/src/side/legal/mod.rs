pub mod consent;
pub mod impressum;
pub mod privacy;

pub use consent::{CookieBanner, CookieSettings};
pub use impressum::Impressum;
pub use privacy::Privacy;

use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct LegalPageShellProps {
    pub title: String,
    pub children: Element,
}

#[component]
pub fn LegalPageShell(props: LegalPageShellProps) -> Element {
    rsx! {
        div { class: "w-full min-h-screen flex flex-col bg-[#F8EFE1]",
            div { class: "flex-1 w-full max-w-3xl mx-auto px-6 py-10 space-y-2",

                div { class: "mb-6",
                    div { class: "flex items-center gap-2.5 mb-1",
                        div { class: "w-1.5 h-7 rounded-full bg-amber-400/70" }
                        h1 { class: "text-2xl font-bold text-zinc-900",
                            "{props.title}"
                        }
                    }
                    div { class: "h-px bg-amber-200/60 mt-4" }
                }

                {props.children}
            }
        }
    }
}

// ─────────────────────────────────────────────
//  LegalSection
//
//  Abschnitt innerhalb einer Legal-Seite.
//  Nutzung:
//    LegalSection { title: "Kontakt".to_string(),
//        p { "Inhalt..." }
//    }
// ─────────────────────────────────────────────

#[derive(Props, Clone, PartialEq)]
pub struct LegalSectionProps {
    pub title: String,
    pub children: Element,
}

#[component]
pub fn LegalSection(props: LegalSectionProps) -> Element {
    rsx! {
        section {
            class: "bg-white rounded-2xl border border-amber-100 shadow-sm overflow-hidden",

            // Abschnitts-Header (gleicher Stil wie Card-Headers in der App)
            div { class: "px-5 py-3 bg-amber-50/70 border-b border-amber-100",
                h2 { class: "text-sm font-semibold text-zinc-700",
                    "{props.title}"
                }
            }

            // Inhalt
            div { class: "px-5 py-4 text-sm text-zinc-600 leading-relaxed space-y-3",
                {props.children}
            }
        }
    }
}
