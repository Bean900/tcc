use dioxus::prelude::*;

use crate::{
    config::Config,
    side::legal::{LegalPageShell, LegalSection},
};

#[component]
pub fn Privacy() -> Element {
    let config = use_context::<Signal<Option<Config>>>().read().clone();
    let c = if let Some(c) = config {
        c.legal
    } else {
        return rsx! {};
    };

    let last_updated = c
        .privacy_last_updated
        .clone()
        .unwrap_or_else(|| "Siehe Impressum".to_string());

    rsx! {
        LegalPageShell { title: "Datenschutzerklärung".to_string(),

            div { class: "text-xs text-zinc-400 -mt-2 mb-4",
                "Letzte Aktualisierung: {last_updated}"
            }

            LegalSection { title: "1. Verantwortlicher (Art. 13 Abs. 1 DSGVO)".to_string(),
                p {
                    "Verantwortlicher im Sinne der Datenschutz-Grundverordnung (DSGVO) ist:"
                }
                div { class: "mt-2 space-y-0.5 font-medium text-zinc-700",
                    if let Some(company) = c.company_name.as_deref().filter(|s| !s.is_empty()) {
                        p { "{company}" }
                    }
                    p { "{c.operator_name}" }
                    if !c.address_street.is_empty() {
                        p { "{c.address_street}" }
                        p { "{c.address_zip} {c.address_city}, {c.address_country}" }
                    }
                    if !c.contact_email.is_empty() {
                        p {
                            a {
                                href: "mailto:{c.contact_email}",
                                class: "text-amber-700 hover:underline",
                                "{c.contact_email}"
                            }
                        }
                    }
                }

                if c.privacy_officer_name.as_deref().map_or(false, |s| !s.is_empty()) {
                    div { class: "mt-3 pt-3 border-t border-zinc-100",
                        p { class: "font-medium text-zinc-700 mb-1",
                            "Datenschutzbeauftragter:"
                        }
                        if let Some(name) = c.privacy_officer_name.as_deref() {
                            p { "{name}" }
                        }
                        if let Some(email) = c.privacy_officer_email.as_deref() {
                            p {
                                a {
                                    href: "mailto:{email}",
                                    class: "text-amber-700 hover:underline",
                                    "{email}"
                                }
                            }
                        }
                    }
                }
            }

            LegalSection { title: "2. Verarbeitete Daten im Überblick".to_string(),
                p {
                    "Wir verarbeiten personenbezogene Daten nur, soweit dies für die \
                     Bereitstellung der Anwendung erforderlich ist. Nachfolgend erläutern \
                     wir, welche Daten zu welchem Zweck verarbeitet werden."
                }
                div { class: "mt-3 overflow-x-auto",
                    table { class: "w-full text-xs border-collapse",
                        thead {
                            tr { class: "bg-amber-50/60 border-b border-amber-100",
                                th { class: "text-left py-2 px-3 font-semibold text-zinc-600", "Datenkategorie" }
                                th { class: "text-left py-2 px-3 font-semibold text-zinc-600", "Zweck" }
                                th { class: "text-left py-2 px-3 font-semibold text-zinc-600", "Rechtsgrundlage" }
                            }
                        }
                        tbody {
                            DataRow {
                                category: "IP-Adressen (Server-Logs)".to_string(),
                                purpose: "Betrieb, Sicherheit, Fehlerdiagnose".to_string(),
                                basis: "Art. 6 Abs. 1 lit. f DSGVO (berechtigte Interessen)".to_string(),
                            }
                            DataRow {
                                category: "Zugangsdaten (Keycloak-Auth)".to_string(),
                                purpose: "Authentifizierung und Autorisierung".to_string(),
                                basis: "Art. 6 Abs. 1 lit. b DSGVO (Vertragserfüllung)".to_string(),
                            }
                            DataRow {
                                category: "Sitzungs-Token (localStorage)".to_string(),
                                purpose: "Aufrechterhaltung der Anmeldung".to_string(),
                                basis: "Art. 6 Abs. 1 lit. b DSGVO / § 25 Abs. 2 TTDSG".to_string(),
                            }
                            DataRow {
                                category: "Anwendungsdaten (Projektinhalte)".to_string(),
                                purpose: "Kerndienst: Speicherung Ihrer Daten".to_string(),
                                basis: "Art. 6 Abs. 1 lit. b DSGVO (Vertragserfüllung)".to_string(),
                            }
                        }
                    }
                }
            }

            LegalSection { title: "3. Hosting und Server-Logs".to_string(),
                p {
                    "Diese Anwendung wird gehostet bei: "
                    strong { class: "text-zinc-700", "{c.hosting_provider}" }
                    ". Serverstandort: "
                    strong { class: "text-zinc-700", "{c.server_location}" }
                    "."
                }
                p {
                    "Bei jedem Aufruf unserer Webanwendung erfasst der Server automatisch \
                     sogenannte Server-Log-Dateien. Diese enthalten u. a.:"
                }
                ul { class: "list-disc list-inside space-y-1 pl-1",
                    li { "IP-Adresse des anfragenden Geräts" }
                    li { "Datum und Uhrzeit der Anfrage" }
                    li { "Aufgerufene Seite / Ressource" }
                    li { "HTTP-Statuscode" }
                    li { "Übertragene Datenmenge" }
                    li { "Browser-Typ und Betriebssystem (User-Agent)" }
                }
                p {
                    "Diese Daten sind technisch notwendig für den sicheren Betrieb und \
                     die Fehlerdiagnose (Rechtsgrundlage: Art. 6 Abs. 1 lit. f DSGVO)."
                }
                p {
                    "Speicherdauer: Die Server-Logs werden nach spätestens "
                    strong { class: "text-zinc-700",
                        "{c.data_retention_logs_days} Tagen"
                    }
                    " automatisch gelöscht."
                }
            }

            LegalSection { title: "4. Authentifizierung (Keycloak)".to_string(),
                p {
                    "Für die Anmeldung an dieser Anwendung nutzen wir Keycloak, einen \
                     Open-Source-Authentifizierungsserver. Keycloak wird auf unserem \
                     eigenen Server in "
                    strong { class: "text-zinc-700", "{c.server_location}" }
                    " betrieben. Es findet keine Übermittlung von \
                     Anmeldedaten an Dritte statt."
                }
                p {
                    "Bei der Anmeldung werden verarbeitet:"
                }
                ul { class: "list-disc list-inside space-y-1 pl-1",
                    li { "Benutzername / E-Mail-Adresse" }
                    li { "Passwort (nur zum Zeitpunkt der Anmeldung, nicht dauerhaft gespeichert)" }
                    li { "Sitzungs-Token (im Browser-Speicher des Nutzers)" }
                    li { "IP-Adresse bei Login-Vorgängen (Sicherheitsprotokollierung)" }
                }
                p {
                    "Rechtsgrundlage: Art. 6 Abs. 1 lit. b DSGVO (Vertragserfüllung)."
                }
            }

            LegalSection { title: "5. Lokale Datenspeicherung (Browser)".to_string(),
                p {
                    "Die Anwendung speichert bestimmte Daten im lokalen Speicher Ihres \
                     Browsers (Web Storage / localStorage). Diese Daten verlassen Ihr \
                     Gerät nicht und werden nicht an Server übertragen, sofern Sie keine \
                     Cloud-Synchronisierung aktiviert haben."
                }
                div { class: "mt-2 space-y-2",
                    StorageRow {
                        key_name: "Authentifizierungs-Token".to_string(),
                        purpose: "Anmeldestatus und automatische Anmeldung".to_string(),
                        duration: "Bis zur Abmeldung oder Token-Ablauf".to_string(),
                    }
                    StorageRow {
                        key_name: "Projektdaten".to_string(),
                        purpose: "Lokale Zwischenspeicherung Ihrer Inhalte".to_string(),
                        duration: "Bis zur manuellen Löschung oder Abmeldung".to_string(),
                    }
                    StorageRow {
                        key_name: "Cookie-Einwilligung (cook_run_consent)".to_string(),
                        purpose: "Speicherung Ihrer Datenschutz-Präferenzen".to_string(),
                        duration: "12 Monate".to_string(),
                    }
                }
                p {
                    "Rechtsgrundlage: § 25 Abs. 2 Nr. 2 TTDSG \
                     (technisch notwendige Datenspeicherung)."
                }
            }

            if c.has_analytics() {
                LegalSection { title: "6. Analyse-Dienste".to_string(),
                    if c.matomo_url.is_some() {
                        div {
                            p { class: "font-medium text-zinc-700 mb-1", "Matomo" }
                            p {
                                "Wir setzen Matomo (ehemals Piwik) für die Analyse der \
                                 Nutzung unserer Anwendung ein. Matomo wird auf unserem \
                                 eigenen Server betrieben; Ihre Daten werden nicht an Dritte \
                                 weitergegeben."
                            }
                            p {
                                "Rechtsgrundlage: Art. 6 Abs. 1 lit. f DSGVO bei \
                                 vollständiger IP-Anonymisierung; andernfalls Art. 6 \
                                 Abs. 1 lit. a DSGVO (Einwilligung)."
                            }
                        }
                    }
                    if c.plausible_domain.is_some() {
                        div {
                            p { class: "font-medium text-zinc-700 mb-1", "Plausible Analytics" }
                            p {
                                "Wir nutzen Plausible Analytics, einen datenschutzfreundlichen \
                                 Analysedienst ohne Cookies und ohne Personenidentifikation. \
                                 Es werden keine personenbezogenen Daten erhoben."
                            }
                            p {
                                "Rechtsgrundlage: § 25 Abs. 2 TTDSG (kein Consent erforderlich)."
                            }
                        }
                    }
                    if c.google_analytics_id.is_some() {
                        div {
                            p { class: "font-medium text-zinc-700 mb-1", "Google Analytics" }
                            p {
                                "Wir verwenden Google Analytics 4 von Google LLC. \
                                 Die Datenverarbeitung erfolgt nur mit Ihrer ausdrücklichen \
                                 Einwilligung. Google kann Daten in die USA übermitteln; \
                                 es gelten Standardvertragsklauseln (SCCs) der EU-Kommission."
                            }
                            p {
                                "Rechtsgrundlage: Art. 6 Abs. 1 lit. a DSGVO (Einwilligung)."
                            }
                        }
                    }
                }
            }

            LegalSection { title: "7. Ihre Rechte als betroffene Person (Art. 15–22 DSGVO)".to_string(),
                p { "Sie haben das Recht auf:" }
                div { class: "mt-2 space-y-1.5",
                    RightRow { icon: "✓", title: "Auskunft (Art. 15 DSGVO)".to_string(),
                        text: "Auskunft über die zu Ihrer Person gespeicherten Daten.".to_string()
                    }
                    RightRow { icon: "✓", title: "Berichtigung (Art. 16 DSGVO)".to_string(),
                        text: "Berichtigung unrichtiger oder Ergänzung unvollständiger Daten.".to_string()
                    }
                    RightRow { icon: "✓", title: "Löschung (Art. 17 DSGVO)".to_string(),
                        text: "Löschung Ihrer Daten, soweit keine gesetzliche Aufbewahrungspflicht besteht.".to_string()
                    }
                    RightRow { icon: "✓", title: "Einschränkung (Art. 18 DSGVO)".to_string(),
                        text: "Einschränkung der Verarbeitung unter bestimmten Voraussetzungen.".to_string()
                    }
                    RightRow { icon: "✓", title: "Datenportabilität (Art. 20 DSGVO)".to_string(),
                        text: "Erhalt Ihrer Daten in einem strukturierten, maschinenlesbaren Format.".to_string()
                    }
                    RightRow { icon: "✓", title: "Widerspruch (Art. 21 DSGVO)".to_string(),
                        text: "Widerspruch gegen die Verarbeitung auf Basis von Art. 6 Abs. 1 lit. f DSGVO.".to_string()
                    }
                    RightRow { icon: "✓", title: "Widerruf von Einwilligungen (Art. 7 Abs. 3 DSGVO)".to_string(),
                        text: "Widerruf jederzeit, ohne Angabe von Gründen.".to_string()
                    }
                }
                p { class: "mt-3",
                    "Zur Geltendmachung Ihrer Rechte wenden Sie sich an: "
                    a {
                        href: "mailto:{c.contact_email}",
                        class: "text-amber-700 hover:underline",
                        "{c.contact_email}"
                    }
                }
            }

            LegalSection { title: "8. Beschwerderecht bei der Aufsichtsbehörde".to_string(),
                p {
                    "Sie haben das Recht, sich jederzeit bei der zuständigen \
                     Datenschutz-Aufsichtsbehörde über die Verarbeitung Ihrer Daten \
                     zu beschweren (Art. 77 DSGVO)."
                }
                if let Some(authority) = c.supervisory_authority.as_deref().filter(|s| !s.is_empty()) {
                    p { class: "mt-2",
                        "Zuständige Aufsichtsbehörde: "
                        strong { class: "text-zinc-700", "{authority}" }
                    }
                }
                p { class: "text-xs text-zinc-400 mt-2",
                    "Eine aktuelle Übersicht aller Datenschutz-Aufsichtsbehörden finden Sie \
                     unter: https://www.bfdi.bund.de"
                }
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
struct DataRowProps {
    category: String,
    purpose: String,
    basis: String,
}

#[component]
fn DataRow(props: DataRowProps) -> Element {
    rsx! {
        tr { class: "border-b border-zinc-100 last:border-0",
            td { class: "py-2 px-3 font-medium text-zinc-700 align-top", "{props.category}" }
            td { class: "py-2 px-3 text-zinc-600 align-top", "{props.purpose}" }
            td { class: "py-2 px-3 text-zinc-500 align-top", "{props.basis}" }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
struct StorageRowProps {
    key_name: String,
    purpose: String,
    duration: String,
}

#[component]
fn StorageRow(props: StorageRowProps) -> Element {
    rsx! {
        div { class: "flex flex-col gap-0.5 p-2.5 rounded-lg bg-zinc-50 border border-zinc-100",
            p { class: "text-xs font-semibold text-zinc-700", "{props.key_name}" }
            p { class: "text-xs text-zinc-500", "{props.purpose}" }
            p { class: "text-[11px] text-zinc-400",
                span { class: "font-medium", "Speicherdauer: " }
                "{props.duration}"
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
struct RightRowProps {
    icon: &'static str,
    title: String,
    text: String,
}

#[component]
fn RightRow(props: RightRowProps) -> Element {
    rsx! {
        div { class: "flex items-start gap-2.5",
            span { class: "shrink-0 text-amber-500 font-bold text-sm mt-0.5",
                "{props.icon}"
            }
            div {
                p { class: "font-medium text-zinc-700 text-xs", "{props.title}" }
                p { class: "text-zinc-500 text-xs", "{props.text}" }
            }
        }
    }
}
