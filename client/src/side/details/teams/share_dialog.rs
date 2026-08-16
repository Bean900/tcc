//! Dialog zur Konfiguration des öffentlichen Team-Anmeldelinks: QR-Code,
//! Link zum Kopieren, Pflichtfelder, maximale Teamanzahl und
//! Anmeldeschluss.

use crate::storage::{self, ShareTeamConfig, StorageManager};
use crate::ui::buttons::{AsyncAction, ConfirmButton};
use crate::ui::cards::StatBox;
use crate::ui::dialogs::Modal;
use crate::ui::forms::{Checkbox, CheckboxVariant, InputDate, InputNumber, InputTime, TextArea};
use crate::ui::tokens::{LBL, NATIVE_INPUT};
use crate::async_action;
use async_std::task::sleep;
use std::time::Duration;
use base64::engine::general_purpose;
use base64::Engine;
use chrono::{Local, Utc};
use dioxus::prelude::*;
use qrcode::render::svg;
use qrcode::QrCode;
use uuid::Uuid;
use web_sys::console;
use web_sys::wasm_bindgen::JsCast;

use super::model::{tab_cls, PopUpWindow};

#[component]
pub(super) fn ShareDialog(
    team_dialog_signal: Signal<PopUpWindow>,
    project_id: Uuid,
    share_config_option: Option<ShareTeamConfig>,
    number_of_teams: usize,
    on_share_config_changed: EventHandler<()>,
) -> Element {
    let share_config = share_config_option.clone().unwrap_or_default();
    let is_share_config_some = share_config_option.is_some();

    let mut share_config_signal = use_signal(|| share_config.clone());

    let mut share_config_date = use_signal(|| {
        share_config_signal
            .read()
            .registration_deadline
            .map(|dt| dt.with_timezone(&Local).date_naive())
    });
    let mut share_config_time = use_signal(|| {
        share_config_signal
            .read()
            .registration_deadline
            .map(|dt| dt.with_timezone(&Local).time())
    });

    // War zuvor ein `Signal<String>`, wird aber nie mutiert – ein
    // einmalig berechneter, einfacher `let`-Wert genügt (Phase 2, Punkt 7).
    let share_url: String = {
        let base_url = web_sys::window()
            .and_then(|w| w.location().origin().ok())
            .unwrap_or_else(|| "unknown".to_string());
        format!("{base_url}/cook-and-run/{project_id}/share")
    };

    let deadline_passed = share_config_signal
        .read()
        .registration_deadline
        .map_or(false, |deadline| deadline < Utc::now());

    let mut current_config_signal = use_signal(|| is_share_config_some && !deadline_passed);

    let max_teams_text = match share_config_signal.read().max_teams {
        Some(0) | None => "∞".to_string(),
        Some(max) => max.to_string(),
    };

    let max_teams_reached = match share_config_signal.read().max_teams {
        Some(0) | None => false,
        Some(max) => number_of_teams >= max as usize,
    };

    // War zuvor `use_signal(|| !max_teams_reached)` und damit nur beim
    // Mount berechnet → wurde nicht aktualisiert, wenn der Nutzer
    // "Max Teams" im Config-Tab ändert (Phase 1, Bug). Jetzt bei jedem
    // Render neu abgeleitet, analog zu `deadline_passed`/`max_teams_reached`.
    let is_active = !max_teams_reached;

    // use_memo bleibt hier bewusst erhalten: QR-Code-Rendering +
    // Base64-Encoding ist eine nicht-triviale Berechnung, die wir nur
    // einmal ausführen wollen (share_url ändert sich im Dialog-Lebenszyklus
    // nicht mehr) – korrekte Anwendung von use_memo für eine teure,
    // stabile Ableitung.
    let qr_data_uri = use_memo({
        let share_url = share_url.clone();
        move || {
            let code = match QrCode::new(share_url.as_bytes()) {
                Ok(c) => c,
                Err(_) => return String::new(),
            };
            let svg_xml = code
                .render::<svg::Color>()
                .min_dimensions(200, 200)
                .dark_color(svg::Color("#000000"))
                .light_color(svg::Color("#ffffff"))
                .build();
            let encoded = general_purpose::STANDARD.encode(svg_xml);
            format!("data:image/svg+xml;base64,{encoded}")
        }
    });

    // Kleines UX-Feedback für den (vormals funktionslosen) Copy-Button.
    let mut copied_signal = use_signal(|| false);

    // Analog zu EditTeamDialog: kurzer grüner "Gespeichert"-Glow nach
    // erfolgreichem Speichern (Modal::glow, siehe tokens::SAVE_GLOW_CSS).
    // Der Dialog bleibt hier bewusst offen (anders als beim Team-Edit) –
    // die Config ist etwas, das man üblicherweise mehrfach nachjustiert.
    let mut save_success_signal = use_signal(|| false);

    let mut storage_signal = use_context::<Signal<StorageManager>>();
    let save_update_share_config: AsyncAction = async_action!({
        if let Some(date) = *share_config_date.read() {
            let time = share_config_time
                .read()
                .unwrap_or_else(|| chrono::NaiveTime::from_hms_opt(23, 59, 59).unwrap_or_default());
            let local_datetime = date.and_time(time);

            let utc_datetime = local_datetime
                .and_local_timezone(chrono::Local)
                .single()
                .map(|local_dt| local_dt.with_timezone(&chrono::Utc));

            share_config_signal.write().registration_deadline = utc_datetime;
        } else {
            share_config_signal.write().registration_deadline = None;
        }

        let config = share_config_signal.read().clone();
        let create_config = config.to_create();
        let mut storage = storage_signal.write();

        let result = if !is_share_config_some {
            storage
                .create_cook_and_run_share_config(project_id, &create_config)
                .await
        } else {
            storage
                .update_cook_and_run_share_config(project_id, &create_config)
                .await
        };

        if let Err(e) = result {
            console::error_1(&format!("Error saving share config: {e}").into());
        } else {
            // `share_config_signal` enthält bereits den neuen Stand und
            // treibt die lokale Dialog-UI weiter direkt an. Zusätzlich
            // informieren wir explizit den Elternteil (TeamsContent),
            // damit dessen `share_config`-Resource neu geladen wird
            // (z. B. für den Badge/Button-Zustand im Hintergrund) –
            // statt uns auf implizites Storage-Signal-Tracking zu
            // verlassen (Phase 2, Punkt 5).
            on_share_config_changed.call(());
            team_dialog_signal.set(PopUpWindow::Share);
            save_success_signal.set(true);
            spawn(async move {
                sleep(Duration::from_millis(2000)).await;
                save_success_signal.set(false);
            });
        }
    });

    let mut toggle_required_field = move |field: storage::RequiredField| {
        let mut config = share_config_signal.write();
        if let Some(pos) = config.required_fields.iter().position(|f| *f == field) {
            config.required_fields.remove(pos);
        } else {
            config.required_fields.push(field);
        }
    };

    let is_success = *save_success_signal.read();

    rsx! {
        Modal {
            title: "Share Configuration".to_string(),
            on_close: move |_| team_dialog_signal.set(PopUpWindow::None),
            accent: true,
            scrollable: true,
            close_on_backdrop_click: false,
            glow: is_success,
            max_width: Some("max-w-4xl".to_string()),
            children: rsx! {
                    div { class: "flex border-b border-amber-100 mb-5",
                        button {
                            r#type: "button",
                            onclick: move |_| {
                                current_config_signal.set(true);
                            },
                            disabled: share_config_option.is_none() || deadline_passed,
                            class: if share_config_option.is_none() || deadline_passed { "px-4 py-2 text-sm font-medium text-zinc-300 cursor-not-allowed opacity-50" } else { tab_cls(*current_config_signal.read()) },
                            "Info"
                        }
                        button {
                            r#type: "button",
                            class: tab_cls(!*current_config_signal.read()),
                            onclick: move |_| {
                                current_config_signal.set(false);
                            },
                            "Config"
                        }
                    }

                    if *current_config_signal.read() {
                        div { class: "space-y-4",
                            div { class: "grid grid-cols-2 gap-4 mb-4",
                                StatBox {
                                    label: "Teams Registered".to_string(),
                                    value: number_of_teams.to_string(),
                                    alert: max_teams_reached,
                                }
                                StatBox {
                                    label: "Teams Allowed".to_string(),
                                    value: max_teams_text.clone(),
                                    alert: max_teams_reached,
                                }
                            }

                            if max_teams_reached {
                                div { class: "rounded-xl border border-amber-300 bg-amber-50 px-4 py-3 mb-4",
                                    p { class: "text-sm font-semibold text-amber-700",
                                        "Maximum team limit reached – no more teams can join via the share link."
                                    }
                                }
                            }

                            div { class: if is_active { "flex flex-col space-y-4" } else { "flex flex-col space-y-4 opacity-50 pointer-events-none" },
                                div { class: "flex flex-col items-center p-4 rounded-xl border border-amber-100 bg-amber-50/30",
                                    label { class: "{LBL} mb-3", "QR Code" }
                                    div { class: "bg-white p-3 rounded-xl border border-amber-100",
                                        img {
                                            src: "{qr_data_uri}",
                                            alt: "QR Code",
                                            class: "w-44 h-44",
                                        }
                                    }
                                    button {
                                        class: "mt-3 px-4 py-2 text-sm font-medium rounded-xl bg-[#D67229] hover:bg-[#C66741] text-white transition-colors duration-150",
                                        disabled: !is_active,
                                        onclick: move |_| {
                                            let qr_data = qr_data_uri.read().clone();
                                            if !qr_data.is_empty() {
                                                if let Some(window) = web_sys::window() {
                                                    let document = window.document().unwrap();
                                                    if let Ok(link) = document.create_element("a") {
                                                        let _ = link.set_attribute("href", &qr_data);
                                                        let _ = link.set_attribute("download", "qr-code.svg");
                                                        if let Some(body) = document.body() {
                                                            let _ = body.append_child(&link);
                                                            if let Some(elem) = link.dyn_ref::<web_sys::HtmlElement>() {
                                                                elem.click();
                                                            }
                                                            let _ = body.remove_child(&link);
                                                        }
                                                    }
                                                }
                                            }
                                        },
                                        "Download QR Code"
                                    }
                                }

                                div { class: "flex flex-col",
                                    label { class: "{LBL}", "Share Link" }
                                    div { class: "flex gap-2 mt-1",
                                        input {
                                            r#type: "text",
                                            readonly: true,
                                            disabled: !is_active,
                                            value: "{share_url}",
                                            class: "{NATIVE_INPUT} flex-1",
                                        }
                                        button {
                                            class: "px-4 py-2 text-sm font-medium rounded-xl bg-[#D67229] hover:bg-[#C66741] text-white transition-colors duration-150",
                                            disabled: !is_active,
                                            onclick: move |_| {
                                                let text = share_url.clone();
                                                spawn(async move {
                                                    if let Some(window) = web_sys::window() {
                                                        let clipboard = window.navigator().clipboard();
                                                        let promise = clipboard.write_text(&text);
                                                        match wasm_bindgen_futures::JsFuture::from(promise).await {
                                                            Ok(_) => {
                                                                copied_signal.set(true);
                                                                sleep(Duration::from_millis(1500)).await;
                                                                copied_signal.set(false);
                                                            }
                                                            Err(_) => {
                                                                console::error_1(&"Error copying share link to clipboard".into());
                                                            }
                                                        }
                                                    }
                                                });
                                            },
                                            if *copied_signal.read() { "Copied!" } else { "Copy" }
                                        }
                                    }
                                }
                            }
                        }
                    } else {
                        div { class: "flex flex-col space-y-5",
                            div {
                                label { class: "{LBL}", "Invitation Text" }
                                TextArea {
                                    placeholder: Some("Enter an invitation text...".to_string()),
                                    value: share_config_signal.read().invite_text.clone(),
                                    is_error: false,
                                    oninput: move |data: Event<FormData>| {
                                        share_config_signal.write().invite_text =
                                            data.value().to_string();
                                    },
                                }
                            }

                            div { class: "grid grid-cols-2 gap-3",
                                Checkbox {
                                    label: "Login required".to_string(),
                                    checked: share_config_signal.read().needs_login,
                                    variant: CheckboxVariant::Card,
                                    title: Some("Teams must be logged in to join".to_string()),
                                    onclick: move |_| {
                                        let current = share_config_signal.read().needs_login;
                                        share_config_signal.write().needs_login = !current;
                                    },
                                }
                                Checkbox {
                                    label: "Verification required".to_string(),
                                    checked: share_config_signal.read().default_needs_check,
                                    variant: CheckboxVariant::Card,
                                    title: Some(
                                        "New teams require verification before participation".to_string(),
                                    ),
                                    onclick: move |_| {
                                        let current = share_config_signal.read().default_needs_check;
                                        share_config_signal.write().default_needs_check = !current;
                                    },
                                }
                            }

                            div { class: "rounded-xl border border-amber-100 bg-amber-50/30 p-4",
                                label { class: "{LBL} mb-3", "Required Fields" }
                                div { class: "grid grid-cols-2 gap-2",
                                    Checkbox {
                                        label: "Email".to_string(),
                                        checked: share_config_signal.read().required_fields.contains(&storage::RequiredField::Mail),
                                        title: Some("Teams must provide an email address".to_string()),
                                        onclick: move |_| toggle_required_field(storage::RequiredField::Mail),
                                    }
                                    Checkbox {
                                        label: "Phone".to_string(),
                                        checked: share_config_signal.read().required_fields.contains(&storage::RequiredField::Phone),
                                        title: Some("Teams must provide a phone number".to_string()),
                                        onclick: move |_| toggle_required_field(storage::RequiredField::Phone),
                                    }
                                    Checkbox {
                                        label: "Number of Members".to_string(),
                                        checked: share_config_signal
                                            .read()
                                            .required_fields
                                            .contains(&storage::RequiredField::Members),
                                        title: Some("Teams must specify number of members".to_string()),
                                        onclick: move |_| toggle_required_field(storage::RequiredField::Members),
                                    }
                                    Checkbox {
                                        label: "Dietary Requirements".to_string(),
                                        checked: share_config_signal.read().required_fields.contains(&storage::RequiredField::Diets),
                                        title: Some("Teams can provide dietary requirements".to_string()),
                                        onclick: move |_| toggle_required_field(storage::RequiredField::Diets),
                                    }
                                }
                            }

                            div { class: "grid grid-cols-2 gap-4",
                                div { class: "cursor-help group relative",
                                    label { class: "{LBL}", "Max Teams" }
                                    InputNumber {
                                        placeholder: Some("0 = unlimited".to_string()),
                                        value: share_config_signal
                                            .read()
                                            .max_teams
                                            .map_or_else(|| "0".to_string(), |v| v.to_string()),
                                        is_error: false,
                                        oninput: move |_e: Event<FormData>| {
                                            let input_value = _e.value().trim().to_string();
                                            if input_value.is_empty() || input_value == "0" {
                                                share_config_signal.write().max_teams = None;
                                            } else if let Ok(num) = input_value.parse::<u32>() {
                                                share_config_signal.write().max_teams = Some(num);
                                            }
                                        },
                                    }
                                }
                                div { class: "cursor-help group relative",
                                    label { class: "{LBL}", "Valid Until" }
                                    div { class: "flex gap-2",
                                        InputDate {
                                            value: share_config_date.read().as_ref().map(|v| v.to_string()).unwrap_or_default(),
                                            class: Some("flex-1".to_string()),
                                            oninput: move |e: Event<FormData>| {
                                                let date_str = e.value();
                                                if let Ok(date) = date_str.parse::<chrono::NaiveDate>() {
                                                    share_config_date.set(Some(date));
                                                } else {
                                                    share_config_date.set(None);
                                                }
                                            },
                                        }
                                        InputTime {
                                            value: share_config_time.read().as_ref().map(|v| v.to_string()).unwrap_or_default(),
                                            class: Some("flex-1".to_string()),
                                            oninput: move |e: Event<FormData>| {
                                                let time_str = e.value();
                                                if let Ok(time) = time_str.parse::<chrono::NaiveTime>() {
                                                    share_config_time.set(Some(time));
                                                } else {
                                                    share_config_time.set(None);
                                                }
                                            },
                                        }
                                    }
                                    if deadline_passed {
                                        div { class: "mt-2 rounded-xl border border-amber-300 bg-amber-50 px-3 py-2",
                                            p { class: "text-xs font-semibold text-amber-700",
                                                "The registration deadline has passed."
                                            }
                                        }
                                    }
                                }
                            }

                            div { class: "flex justify-center mt-2 pt-2",
                                ConfirmButton {
                                    text: if share_config_option.is_none() { "Create & Activate".to_string() } else { "Update".to_string() },
                                    action: save_update_share_config.clone(),
                                }
                            }
                        }
                    }
            },
        }
    }
}
