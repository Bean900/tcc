// ─────────────────────────────────────────────
//  share_register.rs
//
//  Öffentliche Team-Registrierungsseite, die über
//  den Share-Link  /cook-and-run/{id}/share
//  erreichbar ist.
//
//  Layout-Strategie (Tailwind):
//    Mobile   < sm  : Vollbild, kein Card-Rand, gestackte Spalten, voller Button
//    Tablet   sm–md : Zentrierte Card mit Seitenabstand, noch gestackt
//    Desktop  ≥ md  : Zweispaltig (Felder | Adresse), max-w-2xl, vertikal mittig
// ─────────────────────────────────────────────

use chrono::{Local, TimeZone, Utc};
use dioxus::prelude::*;
use uuid::Uuid;
use web_sys::console;

use crate::async_action;
use crate::keycloak::AuthState;
use crate::side::details::address::{Address, AddressParam};
use crate::side::details::{ErrorPage, LoadingPage};
use crate::side::AsyncAction;
use crate::side::{ConfirmButton, Input, InputError, InputNumber, InputPhoneNumber};
use crate::storage::{RequiredField, ShareTeamConfig, StorageManager, TeamCreate};

// ─────────────────────────────────────────────
//  Design-Tokens  (identisch mit teams.rs)
// ─────────────────────────────────────────────

const LBL: &str =
    "block text-[11px] font-semibold tracking-[0.12em] uppercase text-amber-700/70 mb-1.5";

// ─────────────────────────────────────────────
//  Hilfs-Funktionen (übernommen aus teams.rs)
// ─────────────────────────────────────────────

fn map_string(value: String) -> Option<String> {
    let value = value.trim();
    if value.is_empty() {
        None
    } else {
        Some(value.to_string())
    }
}

fn map_u8(value: String) -> Option<u8> {
    let value = value.trim();
    if value.is_empty() {
        None
    } else {
        value.parse::<u8>().ok()
    }
}

// ─────────────────────────────────────────────
//  Sprache
// ─────────────────────────────────────────────

#[derive(Clone, PartialEq, Debug)]
enum Lang {
    De,
    En,
}

struct Txt {
    page_title: &'static str,
    form_title: &'static str,
    lbl_name: &'static str,
    name_ph: &'static str,
    lbl_email: &'static str,
    email_ph: &'static str,
    lbl_phone: &'static str,
    phone_ph: &'static str,
    lbl_members: &'static str,
    members_ph: &'static str,
    lbl_diets: &'static str,
    diets_ph: &'static str,
    lbl_address: &'static str,
    deadline_until: &'static str,
    btn_submit: &'static str,
    err_name_empty: &'static str,
    err_email_invalid: &'static str,
    err_members_invalid: &'static str,
    err_members_zero: &'static str,
    err_phone_empty: &'static str,
    success_title: &'static str,
    success_body: &'static str,
    deadline_passed_title: &'static str,
    deadline_passed_body: &'static str,
    login_required_title: &'static str,
    login_required_body: &'static str,
    max_teams_title: &'static str,
    max_teams_body: &'static str,
    inactive_title: &'static str,
    inactive_body: &'static str,
}

fn txt(lang: &Lang) -> Txt {
    match lang {
        Lang::De => Txt {
            page_title:             "Team registrieren",
            form_title:             "Team-Daten",
            lbl_name:               "Teamname",
            name_ph:                "z.B. Die Chili-Chasers",
            lbl_email:              "E-Mail",
            email_ph:               "z.B. chili@chasers.de",
            lbl_phone:              "Telefon",
            phone_ph:               "z.B. +49 1234 56789",
            lbl_members:            "Anzahl Mitglieder",
            members_ph:             "z.B. 2",
            lbl_diets:              "Ernährungsbesonderheiten",
            diets_ph:               "z.B. vegetarisch, Nussallergie, halal …",
            lbl_address:            "Adresse",
            deadline_until:         "Anmeldung möglich bis",
            btn_submit:             "Team erstellen",
            err_name_empty:         "Teamname darf nicht leer sein!",
            err_email_invalid:      "Bitte eine gültige E-Mail-Adresse eingeben!",
            err_members_invalid:    "Bitte eine gültige Zahl eingeben!",
            err_members_zero:       "Anzahl Mitglieder muss größer als 0 sein!",
            err_phone_empty:       "Telefonnummer darf nicht leer sein!",
            success_title:          "Team erfolgreich registriert!",
            success_body:           "Dein Team wurde angelegt. Wir freuen uns auf dich!",
            deadline_passed_title:  "Anmeldung geschlossen",
            deadline_passed_body:   "Der Anmeldeschluss ist abgelaufen. Es können keine weiteren Teams registriert werden.",
            login_required_title:   "Anmeldung erforderlich",
            login_required_body:    "Für die Registrierung ist ein Login notwendig. Bitte melde dich an, um ein Team anzulegen.",
            max_teams_title:        "Maximale Teamanzahl erreicht",
            max_teams_body:         "Leider können keine weiteren Teams registriert werden – die maximale Anzahl wurde erreicht.",
            inactive_title:         "Link nicht aktiv",
            inactive_body:          "Dieser Registrierungslink ist aktuell nicht aktiv.",
        },
        Lang::En => Txt {
            page_title:             "Register team",
            form_title:             "Team data",
            lbl_name:               "Team name",
            name_ph:                "e.g. The Chili Chasers",
            lbl_email:              "Email",
            email_ph:               "e.g. chili@chasers.de",
            lbl_phone:              "Phone number",
            phone_ph:               "e.g. +49 1234 56789",
            lbl_members:            "Number of members",
            members_ph:             "e.g. 2",
            lbl_diets:              "Dietary requirements",
            diets_ph:               "e.g. vegetarian, nut allergy, halal …",
            lbl_address:            "Address",
            deadline_until:         "Registration open until",
            btn_submit:             "Create team",
            err_name_empty:         "Team name cannot be empty!",
            err_email_invalid:      "Please enter a valid email address!",
            err_members_invalid:    "Please enter a valid number!",
            err_members_zero:       "Number of members must be greater than 0!",
            err_phone_empty:       "Phone number cannot be empty!",
            success_title:          "Team successfully registered!",
            success_body:           "Your team has been created. We look forward to seeing you!",
            deadline_passed_title:  "Registration closed",
            deadline_passed_body:   "The registration deadline has passed. No more teams can be registered.",
            login_required_title:   "Login required",
            login_required_body:    "A login is required to register. Please sign in to create a team.",
            max_teams_title:        "Maximum team limit reached",
            max_teams_body:         "No more teams can be registered – the maximum number of teams has been reached.",
            inactive_title:         "Link not active",
            inactive_body:          "This registration link is currently not active.",
        },
    }
}

// ─────────────────────────────────────────────
//  Validierung
// ─────────────────────────────────────────────

fn check_name(name: Signal<String>, mut err: Signal<String>, t: &Txt) -> bool {
    if name.read().trim().is_empty() {
        err.set(t.err_name_empty.to_string());
        false
    } else {
        err.set(String::new());
        true
    }
}

fn check_phone(phone: Signal<String>, mut err: Signal<String>, t: &Txt) -> bool {
    if phone.read().trim().is_empty() {
        err.set(t.err_phone_empty.to_string());
        false
    } else {
        err.set(String::new());
        true
    }
}

fn check_email(email: Signal<String>, mut err: Signal<String>, t: &Txt) -> bool {
    let v = email.read().trim().to_string();
    if v.is_empty() || (!v.contains('@') || !v.contains('.')) {
        err.set(t.err_email_invalid.to_string());
        false
    } else {
        err.set(String::new());
        true
    }
}

fn check_members(members: Signal<String>, mut err: Signal<String>, t: &Txt) -> bool {
    let v = members.read().trim().to_string();
    if v.is_empty() {
        err.set(t.err_members_invalid.to_string());
        return false;
    }
    match v.parse::<u32>() {
        Err(_) => {
            err.set(t.err_members_invalid.to_string());
            false
        }
        Ok(0) => {
            err.set(t.err_members_zero.to_string());
            false
        }
        Ok(_) => {
            err.set(String::new());
            true
        }
    }
}

// ─────────────────────────────────────────────
//  Root-Komponente
// ─────────────────────────────────────────────

/// Einstiegspunkt für  /cook-and-run/{id}/share
#[component]
pub fn ShareRegisterPage(cook_and_run_id: Uuid) -> Element {
    let storage = use_context::<Signal<StorageManager>>();

    let share_config: Resource<Result<Option<ShareTeamConfig>, String>> =
        use_resource(move || async move {
            storage
                .read()
                .clone()
                .select_cook_and_run_share_config(cook_and_run_id)
                .await
        });

    match &*share_config.read_unchecked() {
        None => rsx!(LoadingPage {}),
        Some(Err(e)) => rsx!(ErrorPage {
            error_text: "Could not load registration page.".to_string(),
            error_details: e.clone(),
        }),
        // Kein Config → Link inaktiv
        Some(Ok(None)) => {
            let lang_signal = use_signal(|| Lang::De);
            rsx! {
                PageLayout {
                    header: rsx! { PageHeader { title: txt(&lang_signal.read()).inactive_title, lang_signal:lang_signal } },
                    content: rsx! {
                        BlockedView {
                            icon:  "lock",
                            title: txt(&lang_signal.read()).inactive_title.to_string(),
                            body:  txt(&lang_signal.read()).inactive_body.to_string(),
                        }
                    },
                }
            }
        }
        Some(Ok(Some(config))) => rsx!(RegisterPageShell {
            cook_and_run_id,
            share_config: config.clone(),
        }),
    }
}

// ─────────────────────────────────────────────
//  Layout-Wrapper
//
//  Mobile  < sm : Rand-loser Vollbild-Stack
//                 (Header klebt am oberen Bildschirmrand)
//  ≥ sm        : Weißes Papier, zentriert, max-w-2xl,
//                vertikal eingebettet in amber-Hintergrund
// ─────────────────────────────────────────────

#[component]
fn PageLayout(header: Element, content: Element) -> Element {
    rsx! {
        div { class: "min-h-screen w-full \
                      flex flex-col",

            // Zentrierungsspalte
            div { class: "w-full sm:max-w-2xl sm:mx-auto \
                          flex flex-col flex-1 \
                          sm:py-10 sm:px-4 gap-4 sm:gap-5",

                {header}
                {content}

                // Atemraum am Seitenende auf Mobile
                div { class: "h-8 sm:hidden" }
            }
        }
    }
}

// ─────────────────────────────────────────────
//  Seiten-Header
//  – sticky auf Mobile (bleibt beim Scrollen oben)
//  – inline auf Desktop (fließt normal)
// ─────────────────────────────────────────────

#[component]
fn PageHeader(title: &'static str, lang_signal: Signal<Lang>) -> Element {
    rsx! {
        header {
            class: "sticky top-0 z-20 \
                    bg-white/95 backdrop-blur-sm border-b border-amber-100 \
                    sm:static sm:bg-transparent sm:backdrop-blur-none sm:border-b-0 \
                    px-4 py-3 sm:px-0 sm:py-0 \
                    flex items-center justify-between shrink-0",

            // Logo-Kreis + Titel
            div { class: "flex items-center gap-2.5",
                div { class: "w-8 h-8 sm:w-9 sm:h-9 rounded-xl \
                              bg-amber-100 flex items-center justify-center \
                              text-[#D67229] shrink-0",
                    svg {
                        class: "w-4 h-4 sm:w-5 sm:h-5",
                        view_box: "0 0 24 24", fill: "none",
                        stroke: "currentColor", stroke_width: "2",
                        path { d: "M17 21v-2a4 4 0 0 0-4-4H5a4 4 0 0 0-4 4v2" }
                        circle { cx: "9", cy: "7", r: "4" }
                        path { d: "M23 21v-2a4 4 0 0 0-3-3.87" }
                        path { d: "M16 3.13a4 4 0 0 1 0 7.75" }
                    }
                }
                span { class: "text-base sm:text-lg font-semibold text-zinc-800",
                    "{title}"
                }
            }

            // Sprach-Toggle
            div { class: "flex gap-1.5",
                {lang_btn("DE", *lang_signal.read() == Lang::De,
                    move |_| lang_signal.set(Lang::De))}
                {lang_btn("EN", *lang_signal.read() == Lang::En,
                    move |_| lang_signal.set(Lang::En))}
            }
        }
    }
}

#[component]
fn RegisterPageShell(cook_and_run_id: Uuid, share_config: ShareTeamConfig) -> Element {
    let auth_signal = use_context::<Signal<Option<AuthState>>>().read().clone();

    let needs_login = match auth_signal {
        Some(AuthState::LoggedIn(_, _, _)) => false,
        _ => share_config.needs_login,
    };

    let lang_signal: Signal<Lang> = use_signal(|| Lang::De);

    let deadline_passed = share_config
        .registration_deadline
        .map_or(false, |dl| dl < Utc::now().naive_utc());

    let max_teams_reached = share_config.max_teams.map_or(false, |max| false);

    let t_title = txt(&lang_signal.read()).page_title;

    rsx! {
        PageLayout {
            header: rsx! { PageHeader { title: t_title, lang_signal } },
            content: rsx! {
                if !share_config.invite_text.is_empty() {
                    div { class: "mx-4 sm:mx-0 \
                                  rounded-2xl border border-amber-200 bg-amber-50/70 \
                                  px-4 py-3.5 sm:px-5 sm:py-4",
                        p { class: "text-sm text-amber-900 leading-relaxed",
                            "{share_config.invite_text}"
                        }
                    }
                }

                if needs_login {
                    LoginRequiredView { lang: lang_signal.read().clone() }
                } else if deadline_passed {
                    DeadlinePassedView {
                        lang: lang_signal.read().clone(),
                        deadline: share_config.registration_deadline.unwrap(),
                    }
                } else if max_teams_reached {
                    BlockedView {
                        icon:  "users",
                        title: txt(&lang_signal.read()).max_teams_title.to_string(),
                        body:  txt(&lang_signal.read()).max_teams_body.to_string(),
                    }
                } else {
                    RegisterFormView {
                        cook_and_run_id,
                        share_config: share_config.clone(),
                        lang: lang_signal,
                    }
                }
            },
        }
    }
}

fn lang_btn(
    label: &'static str,
    active: bool,
    onclick: impl FnMut(MouseEvent) + 'static,
) -> Element {
    let cls = if active {
        "px-3 py-1.5 min-w-[40px] text-xs font-semibold rounded-lg \
         bg-[#D67229] text-white border border-[#D67229] \
         transition-colors duration-150 cursor-pointer"
    } else {
        "px-3 py-1.5 min-w-[40px] text-xs font-semibold rounded-lg \
         bg-transparent text-amber-700 border border-amber-300 \
         hover:border-[#D67229] hover:text-[#D67229] \
         active:bg-amber-50 \
         transition-colors duration-150 cursor-pointer"
    };
    rsx! {
        button { r#type: "button", class: "{cls}", onclick, "{label}" }
    }
}

#[component]
fn BlockedView(icon: &'static str, title: String, body: String) -> Element {
    rsx! {
        div { class: "bg-white \
                      border-y sm:border border-amber-100 \
                      sm:rounded-2xl sm:shadow-sm",

            div { class: "px-4 py-3.5 sm:px-6 sm:py-4 \
                          bg-amber-50/70 border-b border-amber-100 \
                          flex items-center gap-2.5",
                div { class: "w-1.5 h-5 rounded-full bg-amber-400/70 shrink-0" }
                span { class: "text-base font-semibold text-zinc-800", "{title}" }
            }

            div { class: "px-4 py-10 sm:px-6 sm:py-12 \
                          flex flex-col items-center gap-4 text-center",
                div { class: "w-14 h-14 rounded-full bg-amber-100 \
                              flex items-center justify-center text-[#D67229]",
                    {blocked_icon(icon)}
                }
                p { class: "text-sm text-zinc-500 max-w-xs leading-relaxed",
                    "{body}"
                }
            }
        }
    }
}

/// Deadline abgelaufen – mit Zeitstempel-Pill
#[component]
fn DeadlinePassedView(lang: Lang, deadline: chrono::NaiveDateTime) -> Element {
    let t = txt(&lang);
    let formatted = Local
        .from_local_datetime(&deadline)
        .single()
        .map(|dt| dt.format("%d.%m.%Y, %H:%M Uhr").to_string())
        .unwrap_or_else(|| deadline.format("%d.%m.%Y %H:%M").to_string());

    rsx! {
        div { class: "bg-white \
                      border-y sm:border border-amber-100 \
                      sm:rounded-2xl sm:shadow-sm",

            div { class: "px-4 py-3.5 sm:px-6 sm:py-4 \
                          bg-amber-50/70 border-b border-amber-100 \
                          flex items-center gap-2.5",
                div { class: "w-1.5 h-5 rounded-full bg-amber-400/70 shrink-0" }
                span { class: "text-base font-semibold text-zinc-800",
                    "{t.deadline_passed_title}"
                }
            }

            div { class: "px-4 py-10 sm:px-6 sm:py-12 \
                          flex flex-col items-center gap-4 text-center",
                div { class: "w-14 h-14 rounded-full bg-amber-100 \
                              flex items-center justify-center text-[#D67229]",
                    {blocked_icon("clock")}
                }
                p { class: "text-sm text-zinc-500 max-w-xs leading-relaxed",
                    "{t.deadline_passed_body}"
                }
                // Zeitstempel-Pill
                span { class: "inline-flex items-center gap-1.5 \
                               px-3.5 py-1.5 rounded-full \
                               border border-amber-200 bg-amber-50 \
                               text-xs font-semibold text-amber-700 mt-1",
                    svg {
                        class: "w-3.5 h-3.5 shrink-0",
                        view_box: "0 0 24 24", fill: "none",
                        stroke: "currentColor", stroke_width: "2",
                        circle { cx: "12", cy: "12", r: "10" }
                        polyline { points: "12 6 12 12 16 14" }
                    }
                    "{formatted}"
                }
            }
        }
    }
}

#[component]
fn LoginRequiredView(lang: Lang) -> Element {
    let t = txt(&lang);
    rsx! {
        BlockedView {
            icon:  "lock",
            title: t.login_required_title.to_string(),
            body:  t.login_required_body.to_string(),
        }
    }
}

// ─────────────────────────────────────────────
//  Registrierungsformular
//
//  Felder:
//    – Teamname + Adresse : immer sichtbar
//    – E-Mail, Telefon, Mitglieder, Diäten :
//      nur wenn in share_config.required_fields
//
//  Layout:
//    Mobile  < md : einspaltig, randlose Card
//    ≥ md       : zweispaltig (Team-Felder | Adresse)
// ─────────────────────────────────────────────

#[component]
fn RegisterFormView(
    cook_and_run_id: Uuid,
    share_config: ShareTeamConfig,
    lang: Signal<Lang>,
) -> Element {
    let mut name_signal = use_signal(|| String::new());
    let name_err = use_signal(|| String::new());
    let mut email_signal = use_signal(|| String::new());
    let email_err = use_signal(|| String::new());
    let mut phone_signal = use_signal(|| String::new());
    let phone_err = use_signal(|| String::new());
    let mut members_signal = use_signal(|| String::new());
    let members_err = use_signal(|| String::new());
    let mut diets_signal = use_signal(|| String::new());
    let address_param = AddressParam::default();
    let mut submitted = use_signal(|| false);

    // Sichtbarkeit optionaler Felder
    let show_email = share_config.required_fields.contains(&RequiredField::Mail);
    let show_phone = share_config.required_fields.contains(&RequiredField::Phone);
    let show_members = share_config
        .required_fields
        .contains(&RequiredField::Members);
    let show_diets = share_config.required_fields.contains(&RequiredField::Diets);

    // Deadline-Anzeige
    let deadline_str: Option<String> = share_config.registration_deadline.map(|dl| {
        Local
            .from_local_datetime(&dl)
            .single()
            .map(|dt| dt.format("%d.%m.%Y, %H:%M Uhr").to_string())
            .unwrap_or_else(|| dl.format("%d.%m.%Y %H:%M").to_string())
    });

    let default_needs_check = share_config.default_needs_check;

    if *submitted.read() {
        return rsx! { SuccessView { lang: lang.read().clone() } };
    }

    let t = txt(&lang.read());

    rsx! {
        div { class: "bg-white \
                      border-y sm:border border-amber-100 \
                      sm:rounded-2xl sm:shadow-sm",

            // ── Card-Header ────────────────────────────────────────
            div { class: "px-4 py-3.5 sm:px-6 sm:py-4 \
                          bg-amber-50/70 border-b border-amber-100 \
                          flex items-center gap-2.5",
                div { class: "w-1.5 h-5 rounded-full bg-amber-400/70 shrink-0" }
                span { class: "text-base font-semibold text-zinc-800",
                    "{t.form_title}"
                }
            }

            div { class: "px-4 py-4 sm:px-6 sm:py-5 space-y-4 sm:space-y-5",

                // ── Deadline-Banner ────────────────────────────────
                if let Some(dl) = deadline_str {
                    div { class: "flex items-center gap-2.5 \
                                  rounded-xl border border-amber-300 bg-amber-50 \
                                  px-3.5 py-2.5",
                        svg {
                            class: "w-4 h-4 text-amber-600 shrink-0",
                            view_box: "0 0 24 24", fill: "none",
                            stroke: "currentColor", stroke_width: "2",
                            circle { cx: "12", cy: "12", r: "10" }
                            polyline { points: "12 6 12 12 16 14" }
                        }
                        p { class: "text-xs font-semibold text-amber-700",
                            "{t.deadline_until}: {dl}"
                        }
                    }
                }

                // ── Formular-Spalten ───────────────────────────────
                // Mobile: gestackt  |  ≥ md: nebeneinander
                div { class: "flex flex-col md:flex-row md:gap-6",

                    // ── Linke Spalte: Team-Felder ──────────────────
                    // Trennlinie unten (Mobile) / rechts (Desktop)
                    div { class: "flex flex-col gap-3.5 \
                                  border-b border-amber-100 pb-5 mb-5 \
                                  md:border-b-0 md:pb-0 md:mb-0 \
                                  md:border-r md:pr-6 md:flex-1",

                        // Teamname (immer)
                        div {
                            label { class: "{LBL}", "{t.lbl_name}" }
                            Input {
                                place_holer: Some(t.name_ph.to_string()),
                                is_error: !name_err.read().is_empty(),
                                value: name_signal.clone(),
                                oninput: move |e: Event<FormData>| {
                                    name_signal.set(e.value());
                                    check_name(name_signal, name_err, &txt(&lang.read()));
                                },
                            }
                            InputError { error: name_err.read() }
                        }

                        // E-Mail
                        if show_email {
                            div {
                                label { class: "{LBL}", "{t.lbl_email}" }
                                Input {
                                    place_holer: Some(t.email_ph.to_string()),
                                    is_error: !email_err.read().is_empty(),
                                    value: email_signal.clone(),
                                    oninput: move |e: Event<FormData>| {
                                        email_signal.set(e.value());
                                        check_email(email_signal, email_err, &txt(&lang.read()));
                                    },
                                }
                                InputError { error: email_err.read() }
                            }
                        }

                        // Telefon
                        if show_phone {
                            div {
                                label { class: "{LBL}", "{t.lbl_phone}" }
                                InputPhoneNumber {
                                    place_holer: Some(t.phone_ph.to_string()),
                                    is_error: !phone_err.read().is_empty(),
                                    value: phone_signal.clone(),
                                    oninput: move |e: Event<FormData>| {
                                        phone_signal.set(e.value());
                                        check_phone(phone_signal, phone_err, &txt(&lang.read()));
                                    },
                                }
                                InputError { error: phone_err.read() }
                            }
                        }

                        // Mitgliederanzahl
                        if show_members {
                            div {
                                label { class: "{LBL}", "{t.lbl_members}" }
                                InputNumber {
                                    place_holer: Some(t.members_ph.to_string()),
                                    value: members_signal.clone(),
                                    is_error: !members_err.read().is_empty(),
                                    oninput: move |e: Event<FormData>| {
                                        members_signal.set(e.value());
                                        check_members(members_signal, members_err,
                                            &txt(&lang.read()));
                                    },
                                }
                                InputError { error: members_err.read() }
                            }
                        }

                        // Ernährungsbesonderheiten
                        if show_diets {
                            div {
                                label { class: "{LBL}", "{t.lbl_diets}" }
                                Input {
                                    place_holer: Some(t.diets_ph.to_string()),
                                    is_error: false,
                                    value: diets_signal.clone(),
                                    oninput: move |e: Event<FormData>| {
                                        diets_signal.set(e.value());
                                    },
                                }
                            }
                        }
                    }

                    // ── Rechte Spalte: Adresse (immer) ────────────
                    div { class: "flex flex-col gap-2 md:flex-1",
                        label { class: "{LBL}", "{t.lbl_address}" }
                        Address { param: address_param }
                    }
                }

                // ── Footer: Hinweis + Submit ───────────────────────
                // Button: volle Breite auf Mobile, auto ab sm
                div { class: "pt-1 border-t border-amber-100 \
                              flex flex-col sm:flex-row sm:items-center \
                              sm:justify-between gap-3",
                    div { class: "w-full sm:w-auto",
                        ConfirmButton {
                            text: t.btn_submit.to_string(),
                            action: async_action!({
                                let t = txt(&lang.read());

                                let name_ok    = check_name(name_signal, name_err, &t);
                                let phone_ok   = !show_phone
                                    || check_phone(phone_signal, phone_err, &t);
                                let email_ok   = !show_email
                                    || check_email(email_signal, email_err, &t);
                                let members_ok = !show_members
                                    || check_members(members_signal, members_err, &t);
                                let addr_ok    = address_param.check_address_data().is_ok();

                                if !name_ok || !phone_ok || !email_ok || !members_ok || !addr_ok {
                                    return;
                                }

                                let team = TeamCreate {
                                    name: name_signal.read().trim().to_string(),
                                    address: address_param
                                        .get_address_data()
                                        .expect("address validated above"),
                                    mail:    show_email  .then(|| map_string(email_signal.read().clone()))  .flatten(),
                                    phone:   show_phone  .then(|| map_string(phone_signal.read().clone()))  .flatten(),
                                    members: show_members.then(|| map_u8(members_signal.read().clone()))    .flatten(),
                                    diets:   show_diets  .then(|| map_string(diets_signal.read().clone()))  .flatten(),
                                    needs_check: default_needs_check,
                                };

                                let mut storage_signal = use_context::<Signal<StorageManager>>();
                                let result = storage_signal
                                    .write()
                                    .create_team_of_cook_and_run(
                                        cook_and_run_id,
                                        Uuid::new_v4(),
                                        &team,
                                    )
                                    .await;

                                match result {
                                    Ok(_)  => submitted.set(true),
                                    Err(e) => console::error_1(
                                        &format!("Register error: {e}").into()
                                    ),
                                }
                            }),
                        }
                    }
                }
            }
        }
    }
}

// ─────────────────────────────────────────────
//  Erfolgsansicht
// ─────────────────────────────────────────────

#[component]
fn SuccessView(lang: Lang) -> Element {
    let t = txt(&lang);
    rsx! {
        div { class: "bg-white \
                      border-y sm:border border-green-100 \
                      sm:rounded-2xl sm:shadow-sm",

            div { class: "px-4 py-3.5 sm:px-6 sm:py-4 \
                          bg-green-50/70 border-b border-green-100 \
                          flex items-center gap-2.5",
                div { class: "w-1.5 h-5 rounded-full bg-green-400/70 shrink-0" }
                span { class: "text-base font-semibold text-zinc-800",
                    "{t.success_title}"
                }
            }

            div { class: "px-4 py-12 sm:px-6 sm:py-14 \
                          flex flex-col items-center gap-4 text-center",
                div { class: "w-16 h-16 rounded-full bg-green-100 \
                              flex items-center justify-center",
                    svg {
                        class: "w-8 h-8 text-green-600",
                        view_box: "0 0 24 24", fill: "none",
                        stroke: "currentColor", stroke_width: "2.5",
                        polyline { points: "20 6 9 17 4 12" }
                    }
                }
                p { class: "text-sm text-zinc-500 max-w-xs leading-relaxed",
                    "{t.success_body}"
                }
            }
        }
    }
}

// ─────────────────────────────────────────────
//  SVG-Icons für BlockedView / DeadlineView
// ─────────────────────────────────────────────

fn blocked_icon(name: &str) -> Element {
    match name {
        "clock" => rsx! {
            svg {
                class: "w-7 h-7",
                view_box: "0 0 24 24", fill: "none",
                stroke: "currentColor", stroke_width: "2",
                circle { cx: "12", cy: "12", r: "10" }
                polyline { points: "12 6 12 12 16 14" }
            }
        },
        "users" => rsx! {
            svg {
                class: "w-7 h-7",
                view_box: "0 0 24 24", fill: "none",
                stroke: "currentColor", stroke_width: "2",
                path { d: "M17 21v-2a4 4 0 0 0-4-4H5a4 4 0 0 0-4 4v2" }
                circle { cx: "9", cy: "7", r: "4" }
                path { d: "M23 21v-2a4 4 0 0 0-3-3.87" }
                path { d: "M16 3.13a4 4 0 0 1 0 7.75" }
            }
        },
        _ => rsx! {  // "lock" + Fallback
            svg {
                class: "w-7 h-7",
                view_box: "0 0 24 24", fill: "none",
                stroke: "currentColor", stroke_width: "2",
                rect { x: "3", y: "11", width: "18", height: "11", rx: "2", ry: "2" }
                path { d: "M7 11V7a5 5 0 0 1 10 0v4" }
            }
        },
    }
}
