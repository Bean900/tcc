use async_std::task::sleep;
use chrono::NaiveTime;
use dioxus::prelude::*;
use std::time::Duration;
use uuid::Uuid;
use web_sys::console;

use crate::{
    async_action, side::{
        AsyncAction, details::{ErrorPage, LoadingPage, address::Address}
    }, storage::{AddressData, MeetingPointData, StorageManager}, ui::{
        buttons::ConfirmButton, cards::{BaseCard, CardHeader}, forms::{Input, InputError, InputTime}, icons::{EndSVG, StartSVG}, tokens::SAVE_GLOW_CSS, typography::{FieldLabel, Headline1},
    },
};

use super::address::AddressParam;

// ─────────────────────────────────────────────
//  Root
// ─────────────────────────────────────────────

#[component]
pub fn StartEnd(cook_and_run_id: Uuid) -> Element {
    let storage = use_context::<Signal<StorageManager>>();
    let start_end: Resource<Result<(Option<MeetingPointData>, Option<MeetingPointData>), String>> =
        use_resource(move || {
            let storage = storage.clone();
            async move {
                let storage = storage.read().clone();
                let start_point = storage.select_cook_and_run_start_point(cook_and_run_id);
                let end_point = storage.select_cook_and_run_end_point(cook_and_run_id);
                let start_point = start_point.await?;
                let end_point = end_point.await?;
                Ok((start_point, end_point))
            }
        });

    match &*start_end.read_unchecked() {
        None => rsx!(
            LoadingPage {}
        ),
        Some(Err(e)) => rsx!(
            ErrorPage {
                error_text: "Could not load project. You may need to log in or the servers may be offline."
                    .to_string(),
                error_details: e.clone(),
            }
        ),
        Some(Ok(point_data)) => rsx!(
            StartEndContent {
                cook_and_run_id,
                start_point: point_data.0.clone(),
                end_point: point_data.1.clone(),
            }
        ),
    }
}

// ─────────────────────────────────────────────
//  Content
// ─────────────────────────────────────────────

#[component]
pub fn StartEndContent(
    cook_and_run_id: Uuid,
    start_point: Option<MeetingPointData>,
    end_point: Option<MeetingPointData>,
) -> Element {
    let mut start_has_point_signal = use_signal(|| start_point.is_some());
    let mut end_has_point_signal = use_signal(|| end_point.is_some());

    let mut start_name_signal = use_signal(|| {
        start_point
            .as_ref()
            .map_or("".to_string(), |p| p.name.clone())
    });
    let mut end_name_signal = use_signal(|| {
        end_point
            .as_ref()
            .map_or("".to_string(), |p| p.name.clone())
    });

    let start_name_error_signal = use_signal(|| "".to_string());
    let end_name_error_signal = use_signal(|| "".to_string());

    let mut start_time_signal = use_signal(|| {
        start_point
            .as_ref()
            .map_or(NaiveTime::from_hms_opt(0, 0, 0).unwrap(), |p| p.time)
    });
    let mut end_time_signal = use_signal(|| {
        end_point
            .as_ref()
            .map_or(NaiveTime::from_hms_opt(0, 0, 0).unwrap(), |p| p.time)
    });

    let start_adress_param = start_point
        .as_ref()
        .map(|point| AddressParam::new(&point.address))
        .unwrap_or_default();

    let end_adress_param = end_point
        .as_ref()
        .map(|point| AddressParam::new(&point.address))
        .unwrap_or_default();

    let mut save_response_error_signal = use_signal(|| "".to_string());

    // true  → grüner Glow auf den Cards für 2 s
    let mut save_success_signal = use_signal(|| false);

    // true  → ungespeicherte Änderungen vorhanden → Save-Button aktiv
    let mut has_unsaved_changes = use_signal(|| false);

    let on_save: AsyncAction = async_action!({
        save_response_error_signal.set("".to_string());
        let start_has_point = *start_has_point_signal.read();
        let end_has_point = *end_has_point_signal.read();
        if check_if_save_possible(
            start_has_point,
            &start_name_signal.read(),
            start_adress_param.clone(),
            end_has_point,
            &end_name_signal.read(),
            end_adress_param.clone(),
        ) {
            let storage_signal = use_context::<Signal<StorageManager>>();
            let (start_update, end_update) = save_point_data(
                storage_signal.clone(),
                cook_and_run_id,
                start_has_point,
                end_has_point,
                start_name_signal.read().trim(),
                end_name_signal.read().trim(),
                *start_time_signal.read(),
                *end_time_signal.read(),
                start_adress_param.clone(),
                end_adress_param.clone(),
            )
            .await;
            if let Err(e) = &start_update {
                console::error_1(&format!("Failed to update start point: {}", e).into());
                save_response_error_signal.set("Failed to save start/end point data".to_string());
            } else if let Err(e) = &end_update {
                console::error_1(&format!("Failed to update end point: {}", e).into());
                save_response_error_signal.set("Failed to save start/end point data".to_string());
            } else {
                // Erfolg: Änderungen gespeichert, Button deaktivieren, Glow starten
                has_unsaved_changes.set(false);
                save_success_signal.set(true);
                spawn(async move {
                    sleep(Duration::from_millis(2000)).await;
                    save_success_signal.set(false);
                });
            }
        } else {
            console::warn_1(&format!("Validation failed: Cannot save data!").into());
        }
    });

    let is_success = *save_success_signal.read();
    let can_save = *has_unsaved_changes.read();
    let save_btn_wrapper_class = if can_save {
        ""
    } else {
        "opacity-40 pointer-events-none cursor-not-allowed"
    };

    rsx! {
        // ── Keyframe-CSS einbinden ────────────────────────────────
        style { dangerous_inner_html: SAVE_GLOW_CSS }

        section { class: "max-w-6xl mx-auto px-4 sm:px-6 lg:px-8 py-8 space-y-8",

            // ── Page Header ───────────────────────────────────────
            Headline1 {
                headline: "Start & End Point".to_string(),
                subtitle: Some("Define where and when your cooking event begins and ends.".to_string()),
            }

            // ── Two-Column Card Grid ──────────────────────────────
            div { class: "grid grid-cols-1 lg:grid-cols-2 gap-6 items-start",

                // Start Point
                PointCard {
                    title: "Start Point",
                    has_point_signal: start_has_point_signal,
                    toggle_label: "Use start point",
                    name_signal: start_name_signal,
                    name_error_signal: start_name_error_signal,
                    time_signal: start_time_signal,
                    address_param: start_adress_param,
                    is_success,
                    on_name_input: move |event: FormEvent| {
                        has_unsaved_changes.set(true);
                        let name = check_name(&event.value(), start_name_error_signal.clone());
                        start_name_signal.set(name);
                    },
                    on_time_input: move |event: FormEvent| {
                        if let Some(t) = check_time(&event.value()) {
                            has_unsaved_changes.set(true);
                            start_time_signal.set(t);
                        }
                    },
                    svg_icon: rsx! {
                        StartSVG {}
                    },
                }

                // End Point
                PointCard {
                    title: "End Point",
                    has_point_signal: end_has_point_signal,
                    toggle_label: "Use end point",
                    name_signal: end_name_signal,
                    name_error_signal: end_name_error_signal,
                    time_signal: end_time_signal,
                    address_param: end_adress_param,
                    is_success,
                    on_name_input: move |event: FormEvent| {
                        has_unsaved_changes.set(true);
                        let name = check_name(&event.value(), end_name_error_signal.clone());
                        end_name_signal.set(name);
                    },
                    on_time_input: move |event: FormEvent| {
                        if let Some(t) = check_time(&event.value()) {
                            has_unsaved_changes.set(true);
                            end_time_signal.set(t);
                        }
                    },
                    svg_icon: rsx! {
                        EndSVG {}
                    },
                }
            }

            // ── Save Error Banner ─────────────────────────────────
            if !save_response_error_signal.read().is_empty() {
                div { class: "p-4 rounded-xl border border-red-200 bg-red-50 text-sm font-medium text-red-700",
                    "{save_response_error_signal.read()}"
                }
            }

            // ── Save Button ───────────────────────────────────────
            div { class: "flex justify-end pt-2",
                div { class: "{save_btn_wrapper_class}",
                    ConfirmButton {
                        text: if is_success { "Saved!".to_string() } else { "Save Changes".to_string() },
                        action: on_save,
                    }
                }
            }
        }
    }
}

// ─────────────────────────────────────────────
//  Reusable Point Card
// ─────────────────────────────────────────────

#[component]
fn PointCard(
    title: &'static str,
    mut has_point_signal: Signal<bool>,
    toggle_label: &'static str,
    name_signal: Signal<String>,
    name_error_signal: Signal<String>,
    time_signal: Signal<NaiveTime>,
    address_param: AddressParam,
    on_name_input: EventHandler<FormEvent>,
    on_time_input: EventHandler<FormEvent>,
    svg_icon: Element,
    is_success: bool,
) -> Element {
    let is_enabled = *has_point_signal.read();

    let disabled_cls = if is_enabled {
        ""
    } else {
        "opacity-40 pointer-events-none select-none"
    };

    let card_class = if is_success {
        "save-glow-card"
    } else {
        ""
    };

    let header_class = if is_success {
        "save-glow-header"
    } else {
        ""
    };

    rsx! {
        BaseCard { class: card_class.to_string(),
            CardHeader {
                title: title.to_string(),
                class: header_class.to_string(),
                action: rsx! {
                    div { class: "flex items-center gap-1.5 shrink-0 text-amber-700", {svg_icon} }
                },
            }

            div { class: "p-6 space-y-5",

                // Checkbox Toggle
                label { class: "inline-flex items-center gap-2.5 cursor-pointer group select-none",
                    input {
                        r#type: "checkbox",
                        checked: is_enabled,
                        class: "w-4 h-4 rounded text-amber-600 focus:ring-amber-500 border-amber-300 accent-amber-600 cursor-pointer",
                        oninput: move |_| {
                            let toggle = *has_point_signal.read();
                            has_point_signal.set(!toggle);
                        },
                    }
                    span { class: "text-xs font-semibold text-zinc-700 group-hover:text-amber-900 transition-colors",
                        "{toggle_label}"
                    }
                }

                // Name & Time Eingabefelder
                div { class: "space-y-4 {disabled_cls} transition-opacity duration-150",
                    div {
                        FieldLabel { text: "Name".to_string() }
                        Input {
                            value: name_signal.read().clone(),
                            place_holer: Some(title.to_string()),
                            is_error: !name_error_signal.read().is_empty(),
                            oninput: move |e| on_name_input.call(e),
                        }
                        InputError { error: name_error_signal.read().clone() }
                    }

                    div {
                        FieldLabel { text: "Time".to_string() }
                        InputTime {
                            value: time_signal.read().format("%H:%M").to_string(),
                            oninput: move |e| on_time_input.call(e),
                        }
                    }
                }

                // Adresse
                div { class: "{disabled_cls} transition-opacity duration-150 pt-2 border-t border-amber-100/60",
                    Address { param: address_param }
                }
            }
        }
    }
}

// ─────────────────────────────────────────────
//  Helpers
// ─────────────────────────────────────────────

fn check_name(name: &str, mut error_signal: Signal<String>) -> String {
    let trim_name = name.trim();
    if trim_name.is_empty() {
        error_signal.set("Name cannot be empty!".to_string());
    } else {
        error_signal.set("".to_string());
    }
    name.to_string()
}

fn check_time(time_str: &str) -> Option<NaiveTime> {
    if let Ok(t) = NaiveTime::parse_from_str(time_str, "%H:%M") {
        return Some(t);
    }
    match NaiveTime::parse_from_str(time_str, "%H:%M:%S") {
        Ok(t) => Some(t),
        Err(e) => {
            console::error_1(&format!("Time format is not correct: {}", e).into());
            None
        }
    }
}

fn check_if_save_possible(
    start_has_point: bool,
    start_name: &str,
    start_address: AddressParam,
    end_has_point: bool,
    end_name: &str,
    end_address: AddressParam,
) -> bool {
    console::log_1(&"Checking if save is possible...".into());
    if start_has_point
        && (start_name.trim().is_empty() || start_address.check_address_data().is_err())
    {
        return false;
    }
    if end_has_point && (end_name.trim().is_empty() || end_address.check_address_data().is_err()) {
        return false;
    }
    true
}

fn to_meeting_point_data(
    name: &str,
    time: NaiveTime,
    address_param: AddressParam,
) -> MeetingPointData {
    let address = address_param
        .get_address_data()
        .unwrap_or_else(|_| AddressData::default());
    MeetingPointData {
        name: name.to_string(),
        time,
        address,
    }
}

async fn save_point_data(
    mut storage_signal: Signal<StorageManager>,
    cook_and_run_id: Uuid,
    start_has_point: bool,
    end_has_point: bool,
    start_name: &str,
    end_name: &str,
    start_time: NaiveTime,
    end_time: NaiveTime,
    start_adress_param: AddressParam,
    end_adress_param: AddressParam,
) -> (Result<(), String>, Result<(), String>) {
    let start_point = if start_has_point {
        Some(to_meeting_point_data(
            start_name,
            start_time,
            start_adress_param,
        ))
    } else {
        None
    };

    let end_point = if end_has_point {
        Some(to_meeting_point_data(end_name, end_time, end_adress_param))
    } else {
        None
    };

    let mut storage = storage_signal.write().clone();
    let start_update = storage
        .update_start_point_in_cook_and_run(cook_and_run_id, &start_point)
        .await;
    console::log_1(&"Saved start point data.".into());
    let end_update = storage
        .update_end_point_in_cook_and_run(cook_and_run_id, &end_point)
        .await;
    console::log_1(&"Saved end point data.".into());
    console::log_1(&"Finished saving point data.".into());
    (start_update, end_update)
}