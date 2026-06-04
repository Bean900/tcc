use dioxus::prelude::*;

use crate::{
    config::Config,
    side::legal::{LegalPageShell, LegalSection},
};

#[derive(Clone, Copy, PartialEq)]
enum Lang {
    De,
    En,
}

struct Tx {
    toggle_btn: &'static str,
    page_title: &'static str,

    tmg_title: &'static str,
    auth_reps_label: &'static str,

    contact_title: &'static str,

    further_title: &'static str,
    legal_form_label: &'static str,
    vat_label: &'static str,
    reg_nr_label: &'static str,
    reg_court_label: &'static str,

    supervisory_title: &'static str,

    editorial_title: &'static str,

    odr_title: &'static str,
    odr_pre: &'static str,
    odr_post: &'static str,
    odr_no_part: &'static str,

    disclaimer_title: &'static str,
    content_head: &'static str,
    content_body: &'static str,
    links_head: &'static str,
    links_body: &'static str,
}

static DE: Tx = Tx {
    toggle_btn: "EN",
    page_title: "Impressum",

    tmg_title: "Angaben gemäß § 5 TMG",
    auth_reps_label: "Vertreten durch:",

    contact_title: "Kontakt",

    further_title: "Weitere Angaben",
    legal_form_label: "Rechtsform:",
    vat_label: "Umsatzsteuer-ID:",
    reg_nr_label: "Handelsregisternummer:",
    reg_court_label: "Registergericht:",

    supervisory_title: "Aufsichtsbehörde",

    editorial_title: "Verantwortliche/r gem. § 18 Abs. 2 MStV",

    odr_title: "Online-Streitbeilegung",
    odr_pre: "Die Europäische Kommission stellt eine Plattform zur \
              Online-Streitbeilegung (OS) bereit: ",
    odr_post: ". Unsere E-Mail-Adresse finden Sie oben im Impressum.",
    odr_no_part: "Wir sind nicht bereit und nicht verpflichtet, an \
                  Streitbeilegungsverfahren vor einer Verbraucherschlichtungsstelle \
                  teilzunehmen (§ 36 Abs. 1 Nr. 1 VSBG).",

    disclaimer_title: "Haftungsausschluss",
    content_head: "Haftung für Inhalte",
    content_body: "Die Inhalte dieser Seiten wurden mit größter Sorgfalt erstellt. Für die \
                   Richtigkeit, Vollständigkeit und Aktualität der Inhalte können wir jedoch \
                   keine Gewähr übernehmen. Als Diensteanbieter sind wir gemäß § 7 Abs. 1 TMG \
                   für eigene Inhalte auf diesen Seiten nach den allgemeinen Gesetzen \
                   verantwortlich. Nach §§ 8 bis 10 TMG sind wir als Diensteanbieter jedoch \
                   nicht verpflichtet, übermittelte oder gespeicherte fremde Informationen zu \
                   überwachen oder nach Umständen zu forschen, die auf eine rechtswidrige \
                   Tätigkeit hinweisen.",
    links_head: "Haftung für Links",
    links_body: "Unser Angebot enthält Links zu externen Webseiten Dritter, auf deren Inhalte \
                 wir keinen Einfluss haben. Deshalb können wir für diese fremden Inhalte auch \
                 keine Gewähr übernehmen. Für die Inhalte der verlinkten Seiten ist stets der \
                 jeweilige Anbieter oder Betreiber der Seiten verantwortlich.",
};

static EN: Tx = Tx {
    toggle_btn: "DE",
    page_title: "Legal Notice",

    tmg_title: "Information pursuant to § 5 TMG",
    auth_reps_label: "Represented by:",

    contact_title: "Contact",

    further_title: "Further Details",
    legal_form_label: "Legal Form:",
    vat_label: "VAT ID:",
    reg_nr_label: "Commercial Register No.:",
    reg_court_label: "Register Court:",

    supervisory_title: "Supervisory Authority",

    editorial_title: "Responsible for Editorial Content (§ 18 Abs. 2 MStV)",

    odr_title: "Online Dispute Resolution",
    odr_pre: "The European Commission provides a platform for online dispute resolution (ODR): ",
    odr_post: ". Our e-mail address can be found above in the legal notice.",
    odr_no_part: "We are neither willing nor obligated to participate in dispute resolution \
                  proceedings before a consumer arbitration body (§ 36 Abs. 1 Nr. 1 VSBG).",

    disclaimer_title: "Disclaimer",
    content_head: "Liability for Content",
    content_body: "The contents of these pages have been prepared with the utmost care. \
                   However, we cannot guarantee the accuracy, completeness, or timeliness of \
                   the content. As a service provider, we are responsible for our own content \
                   on these pages pursuant to § 7 Abs. 1 TMG under general law. Under §§ 8–10 \
                   TMG, however, we are not obligated as a service provider to monitor \
                   transmitted or stored third-party information or to investigate circumstances \
                   that indicate illegal activity.",
    links_head: "Liability for Links",
    links_body: "Our website contains links to external third-party websites whose content we \
                 have no control over. Therefore, we cannot assume any liability for this \
                 third-party content. The respective provider or operator of linked pages is \
                 always responsible for their content.",
};

#[component]
pub fn Impressum() -> Element {
    let config = use_context::<Signal<Option<Config>>>().read().clone();
    let c = if let Some(c) = config {
        c.legal
    } else {
        return rsx! {};
    };

    let mut lang = use_signal(|| Lang::De);
    let current_lang = *lang.read();
    let t: &'static Tx = if current_lang == Lang::De { &DE } else { &EN };

    let has_further = c.legal_form.as_deref().map_or(false, |s| !s.is_empty())
        || c.vat_id.as_deref().map_or(false, |s| !s.is_empty())
        || c.commercial_register_number
            .as_deref()
            .map_or(false, |s| !s.is_empty());

    rsx! {
        LegalPageShell { title: t.page_title.to_string(),

            div { class: "flex justify-end mb-6",
                button {
                    r#type: "button",
                    class: "inline-flex items-center gap-1.5 px-4 py-1.5 rounded-full \
                            text-sm font-medium border border-zinc-200 bg-white text-zinc-500 \
                            shadow-sm hover:border-amber-400 hover:text-amber-700 \
                            transition-all duration-200 cursor-pointer select-none",
                    onclick: move |_| {
                        lang.set(if *lang.read() == Lang::De { Lang::En } else { Lang::De });
                    },
                    "{t.toggle_btn}"
                }
            }

            LegalSection { title: t.tmg_title.to_string(),
                div { class: "flex flex-col items-left text-left gap-0.5 text-zinc-700",

                    if let Some(company) = c.company_name.as_deref().filter(|s| !s.is_empty()) {
                        p { class: "text-lg font-bold text-zinc-800", "{company}" }
                    }
                    p {
                        class: if c.company_name.is_some() {
                            "text-zinc-600"
                        } else {
                            "text-lg font-bold text-zinc-800"
                        },
                        "{c.operator_name}"
                    }
                    if let Some(reps) = c.authorized_representatives.as_deref().filter(|s| !s.is_empty()) {
                        p { class: "mt-0.5 text-sm text-zinc-500",
                            span { class: "font-medium text-zinc-600", "{t.auth_reps_label} " }
                            "{reps}"
                        }
                    }
                    if !c.address_street.is_empty() {
                        div { class: "mt-3 space-y-0.5",
                            p { "{c.address_street}" }
                            p { "{c.address_zip} {c.address_city}" }
                            if !c.address_country.is_empty() {
                                p { "{c.address_country}" }
                            }
                        }
                    }
                }
            }

            LegalSection { title: t.contact_title.to_string(),
                div { class: "flex flex-col items-left gap-2",
                    if !c.contact_email.is_empty() {
                        div { class: "flex items-left gap-2 text-zinc-700",
                            ContactIcon { kind: "email" }
                            a {
                                href: "mailto:{c.contact_email}",
                                class: "hover:text-amber-700 transition-colors duration-150",
                                "{c.contact_email}"
                            }
                        }
                    }
                    if let Some(phone) = c.contact_phone.as_deref().filter(|s| !s.is_empty()) {
                        div { class: "flex items-left gap-2 text-zinc-700",
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

            if has_further {
                LegalSection { title: t.further_title.to_string(),
                    div { class: "flex flex-col items-left gap-1 text-left text-zinc-700",
                        if let Some(form) = c.legal_form.as_deref().filter(|s| !s.is_empty()) {
                            p {
                                span { class: "font-medium text-zinc-600", "{t.legal_form_label} " }
                                "{form}"
                            }
                        }
                        if let Some(vat) = c.vat_id.as_deref().filter(|s| !s.is_empty()) {
                            p {
                                span { class: "font-medium text-zinc-600", "{t.vat_label} " }
                                "{vat}"
                            }
                        }
                        if let Some(nr) = c.commercial_register_number.as_deref().filter(|s| !s.is_empty()) {
                            p {
                                span { class: "font-medium text-zinc-600", "{t.reg_nr_label} " }
                                "{nr}"
                            }
                        }
                        if let Some(court) = c.commercial_register_court.as_deref().filter(|s| !s.is_empty()) {
                            p {
                                span { class: "font-medium text-zinc-600", "{t.reg_court_label} " }
                                "{court}"
                            }
                        }
                    }
                }
            }

            if let Some(authority) = c.supervisory_authority.as_deref().filter(|s| !s.is_empty()) {
                LegalSection { title: t.supervisory_title.to_string(),
                    p { class: "text-left text-zinc-700", "{authority}" }
                }
            }

            if let Some(editorial) = c.editorial_responsible.as_deref().filter(|s| !s.is_empty()) {
                LegalSection { title: t.editorial_title.to_string(),
                    p { class: "text-left text-zinc-700", "{editorial}" }
                }
            }

            LegalSection { title: t.odr_title.to_string(),
                div { class: "space-y-3 text-left text-zinc-700",
                    p {
                        "{t.odr_pre}"
                        a {
                            href: "https://ec.europa.eu/consumers/odr/",
                            target: "_blank",
                            rel: "noopener noreferrer",
                            class: "text-amber-700 hover:text-amber-800 hover:underline \
                                    transition-colors duration-150",
                            "https://ec.europa.eu/consumers/odr/"
                        }
                        "{t.odr_post}"
                    }
                    p { "{t.odr_no_part}" }
                }
            }

            LegalSection { title: t.disclaimer_title.to_string(),
                div { class: "space-y-4 text-left text-zinc-600",
                    div {
                        p { class: "font-semibold text-zinc-700 mb-1.5", "{t.content_head}" }
                        p { "{t.content_body}" }
                    }
                    div {
                        p { class: "font-semibold text-zinc-700 mb-1.5", "{t.links_head}" }
                        p { "{t.links_body}" }
                    }
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
                    fill: "none",
                    xmlns: "http://www.w3.org/2000/svg",
                    view_box: "0 0 24 24",
                    stroke: "currentColor",
                    stroke_width: "2",
                    path {
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                        d: "M3 8l7.89 5.26a2 2 0 002.22 0L21 8M5 19h14a2 2 0 002-2V7a2 2 0 \
                           00-2-2H5a2 2 0 00-2 2v10a2 2 0 002 2z"
                    }
                }
            } else {
                svg {
                    class: "w-4 h-4",
                    fill: "none",
                    xmlns: "http://www.w3.org/2000/svg",
                    view_box: "0 0 24 24",
                    stroke: "currentColor",
                    stroke_width: "2",
                    path {
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
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
