use dioxus::prelude::*;

use crate::{
    config::Config,
    side::legal::{LegalPageShell, LegalSection},
};

#[component]
pub fn Impressum() -> Element {
    let config = use_context::<Signal<Option<Config>>>().read().clone();
    let c = if let Some(c) = config {
        c.legal
    } else {
        return rsx! {};
    };

    rsx! {
        LegalPageShell { title: "Impressum".to_string(),

            LegalSection { title: "Angaben gemäß § 5 TMG".to_string(),
                div { class: "space-y-1",
                    // Firmenname (falls abweichend)
                    if let Some(company) = c.company_name.as_deref().filter(|s| !s.is_empty()) {
                        p { class: "font-semibold text-zinc-800", "{company}" }
                    }
                    // Operator-Name
                    p { class: if c.company_name.is_some() { "text-zinc-600" } else { "font-semibold text-zinc-800" },
                        "{c.operator_name}"
                    }
                    // Adresse
                    if !c.address_street.is_empty() {
                        p { "{c.address_street}" }
                        p { "{c.address_zip} {c.address_city}, {c.address_country}" }
                    }
                }
            }

            LegalSection { title: "Kontakt".to_string(),
                div { class: "space-y-1.5",
                    if !c.contact_email.is_empty() {
                        div { class: "flex items-center gap-2",
                            ContactIcon { kind: "email" }
                            a {
                                href: "mailto:{c.contact_email}",
                                class: "hover:text-amber-700 transition-colors duration-150",
                                "{c.contact_email}"
                            }
                        }
                    }
                    if let Some(phone) = c.contact_phone.as_deref().filter(|s| !s.is_empty()) {
                        div { class: "flex items-center gap-2",
                            ContactIcon { kind: "phone" }
                            a {
                                href: "tel:{phone}",
                                class: "hover:text-amber-700 transition-colors duration-150",
                                "{phone}"
                            }
                        }
                    }
                }
            }

            if c.vat_id.as_deref().map_or(false, |s| !s.is_empty())
                || c.commercial_register_number.as_deref().map_or(false, |s| !s.is_empty())
            {
                LegalSection { title: "Weitere Angaben".to_string(),
                    div { class: "space-y-1",
                        if let Some(vat) = c.vat_id.as_deref().filter(|s| !s.is_empty()) {
                            p {
                                span { class: "font-medium text-zinc-700", "Umsatzsteuer-ID: " }
                                "{vat}"
                            }
                        }
                        if let Some(reg_nr) = c.commercial_register_number.as_deref().filter(|s| !s.is_empty()) {
                            p {
                                span { class: "font-medium text-zinc-700", "Handelsregisternummer: " }
                                "{reg_nr}"
                            }
                        }
                        if let Some(reg_court) = c.commercial_register_court.as_deref().filter(|s| !s.is_empty()) {
                            p {
                                span { class: "font-medium text-zinc-700", "Registergericht: " }
                                "{reg_court}"
                            }
                        }
                    }
                }
            }

            if let Some(editorial) = c.editorial_responsible.as_deref().filter(|s| !s.is_empty()) {
                LegalSection { title: "Verantwortlicher gem. § 18 Abs. 2 MStV".to_string(),
                    p { "{editorial}" }
                }
            }

            LegalSection { title: "Online-Streitbeilegung".to_string(),
                p {
                    "Die Europäische Kommission stellt eine Plattform zur \
                     Online-Streitbeilegung (OS) bereit: "
                    a {
                        href: "https://ec.europa.eu/consumers/odr/",
                        target: "_blank",
                        rel: "noopener noreferrer",
                        class: "text-amber-700 hover:text-amber-800 hover:underline \
                                transition-colors duration-150",
                        "https://ec.europa.eu/consumers/odr/"
                    }
                    ". Unsere E-Mail-Adresse finden Sie oben im Impressum."
                }
                p {
                    "Wir sind nicht bereit und nicht verpflichtet, an \
                     Streitbeilegungsverfahren vor einer \
                     Verbraucherschlichtungsstelle teilzunehmen \
                     (§ 36 Abs. 1 Nr. 1 VSBG)."
                }
            }

            LegalSection { title: "Haftungsausschluss".to_string(),
                p { class: "font-medium text-zinc-700", "Haftung für Inhalte" }
                p {
                    "Die Inhalte dieser Seiten wurden mit größter Sorgfalt erstellt. \
                     Für die Richtigkeit, Vollständigkeit und Aktualität der Inhalte \
                     können wir jedoch keine Gewähr übernehmen. Als Diensteanbieter \
                     sind wir gemäß § 7 Abs. 1 TMG für eigene Inhalte auf diesen \
                     Seiten nach den allgemeinen Gesetzen verantwortlich. \
                     Nach §§ 8 bis 10 TMG sind wir als Diensteanbieter jedoch nicht \
                     verpflichtet, übermittelte oder gespeicherte fremde Informationen \
                     zu überwachen oder nach Umständen zu forschen, die auf eine \
                     rechtswidrige Tätigkeit hinweisen."
                }
                p { class: "font-medium text-zinc-700 mt-2", "Haftung für Links" }
                p {
                    "Unser Angebot enthält Links zu externen Webseiten Dritter, auf \
                     deren Inhalte wir keinen Einfluss haben. Deshalb können wir für \
                     diese fremden Inhalte auch keine Gewähr übernehmen. Für die Inhalte \
                     der verlinkten Seiten ist stets der jeweilige Anbieter oder Betreiber \
                     der Seiten verantwortlich."
                }
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
struct ContactIconProps {
    kind: &'static str,
}

#[component]
fn ContactIcon(props: ContactIconProps) -> Element {
    rsx! {
        span { class: "shrink-0 text-amber-500",
            if props.kind == "email" {
                svg {
                    class: "w-4 h-4",
                    fill: "none", xmlns: "http://www.w3.org/2000/svg",
                    view_box: "0 0 24 24", stroke: "currentColor", stroke_width: "2",
                    path { stroke_linecap: "round", stroke_linejoin: "round",
                        d: "M3 8l7.89 5.26a2 2 0 002.22 0L21 8M5 19h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v10a2 2 0 002 2z"
                    }
                }
            } else {
                svg {
                    class: "w-4 h-4",
                    fill: "none", xmlns: "http://www.w3.org/2000/svg",
                    view_box: "0 0 24 24", stroke: "currentColor", stroke_width: "2",
                    path { stroke_linecap: "round", stroke_linejoin: "round",
                        d: "M3 5a2 2 0 012-2h3.28a1 1 0 01.948.684l1.498 4.493a1 1 0 01-.502 \
                           1.21l-2.257 1.13a11.042 11.042 0 005.516 5.516l1.13-2.257a1 1 0 \
                           011.21-.502l4.493 1.498a1 1 0 01.684.949V19a2 2 0 01-2 2h-1C9.716 \
                           21 3 14.284 3 6V5z"
                    }
                }
            }
        }
    }
}
