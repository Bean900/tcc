use async_std::task::sleep;
use chrono::NaiveTime;
use dioxus::prelude::*;
use std::time::Duration;
use uuid::Uuid;
use web_sys::console;

use crate::{
    async_action,
    side::{
        details::{address::Address, ErrorPage, LoadingPage},
        AsyncAction, ConfirmButton, EndSVG, Headline1, Input, InputError, InputTime, StartSVG,
    },
    storage::{AddressData, MeetingPointData, StorageManager},
};

use super::address::AddressParam;

// ─────────────────────────────────────────────
//  CSS: Keyframe-Animation für den grünen Glow
// ─────────────────────────────────────────────

const SAVE_GLOW_CSS: &str = r#"
@keyframes save-glow {
    0%   {
        border-color: #d1fae5;
        box-shadow: 0 0 0 0px rgba(34, 197, 94, 0),
                    0 1px 3px 0 rgba(0, 0, 0, 0.06);
    }
    20%  {
        border-color: #22c55e;
        box-shadow: 0 0 0 5px rgba(34, 197, 94, 0.22),
                    0 1px 3px 0 rgba(0, 0, 0, 0.06);
    }
    55%  {
        border-color: #16a34a;
        box-shadow: 0 0 0 5px rgba(34, 197, 94, 0.10),
                    0 1px 3px 0 rgba(0, 0, 0, 0.06);
    }
    100% {
        border-color: #bbf7d0;
        box-shadow: 0 0 0 0px rgba(34, 197, 94, 0),
                    0 1px 3px 0 rgba(0, 0, 0, 0.06);
    }
}
.save-glow-card {
    animation: save-glow 2s ease-in-out forwards;
}
.save-glow-card .save-glow-header {
    background-color: rgba(240, 253, 244, 0.70) !important;
    border-bottom-color: #bbf7d0 !important;
    transition: background-color 0.4s ease, border-color 0.4s ease;
}
.save-glow-card .save-glow-accent {
    background-color: rgba(34, 197, 94, 0.75) !important;
    transition: background-color 0.4s ease;
}
"#;

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
        None => rsx!(LoadingPage {}),
        Some(Err(e)) => rsx!(ErrorPage {
            error_text:
                "Could not load project. You may need to log in or the servers may be offline."
                    .to_string(),
            error_details: e.clone(),
        }),
        Some(Ok(point_data)) => rsx!(StartEndContent {
            cook_and_run_id,
            start_point: point_data.0.clone(),
            end_point: point_data.1.clone(),
        }),
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

        section { class: "px-8 py-6 space-y-8",

            // ── Page header ───────────────────────────────────────
            Headline1 { headline: "Start & End Point".to_string() }

            // ── Two-column card grid ──────────────────────────────
            div { class: "grid grid-cols-1 md:grid-cols-2 gap-6",

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
                    on_name_input: move |event: Event<FormData>| {
                        has_unsaved_changes.set(true);
                        let name = check_name(&event.value(), start_name_error_signal.clone());
                        start_name_signal.set(name);
                    },
                    on_time_input: move |event: Event<FormData>| {
                        if let Some(t) = check_time(&event.value()) {
                            has_unsaved_changes.set(true);
                            start_time_signal.set(t);
                        }
                    },
                    on_toggle: move |_| {
                        has_unsaved_changes.set(true);
                        let checkbox_state = !*start_has_point_signal.read();
                        start_has_point_signal.set(checkbox_state);
                    },
                    svg_icon: rsx!(StartSVG {}),
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
                    on_name_input: move |event: Event<FormData>| {
                        has_unsaved_changes.set(true);
                        let name = check_name(&event.value(), end_name_error_signal.clone());
                        end_name_signal.set(name);
                    },
                    on_time_input: move |event: Event<FormData>| {
                        if let Some(t) = check_time(&event.value()) {
                            has_unsaved_changes.set(true);
                            end_time_signal.set(t);
                        }
                    },
                    on_toggle: move |_| {
                        has_unsaved_changes.set(true);
                        let checkbox_state = !*end_has_point_signal.read();
                        end_has_point_signal.set(checkbox_state);
                    },
                    svg_icon: rsx!(EndSVG {}),
                }
            }

            // ── Save error banner ─────────────────────────────────
            if !save_response_error_signal.read().is_empty() {
                div { class: "rounded-2xl border border-red-200 bg-red-50 px-5 py-4 text-sm text-red-700",
                    "{save_response_error_signal.read()}"
                }
            }

            // ── Save button (disabled solange keine Änderungen) ───
            div { class: "flex justify-end pt-1",
                div { class: "{save_btn_wrapper_class}",
                    ConfirmButton { action: on_save, text: "Save".to_string() }
                }
            }
        }
    }
}

// ─────────────────────────────────────────────
//  Reusable point card
// ─────────────────────────────────────────────

#[component]
fn PointCard(
    title: &'static str,
    has_point_signal: Signal<bool>,
    toggle_label: &'static str,
    name_signal: Signal<String>,
    name_error_signal: Signal<String>,
    time_signal: Signal<NaiveTime>,
    address_param: AddressParam,
    on_name_input: EventHandler<Event<FormData>>,
    on_time_input: EventHandler<Event<FormData>>,
    on_toggle: EventHandler<MouseData>,
    svg_icon: Element,
    is_success: bool,
) -> Element {
    let disabled_cls = if *has_point_signal.read() {
        ""
    } else {
        "opacity-40 pointer-events-none"
    };

    // Karte: im Erfolgsfall grüner Glow via @keyframes
    let card_class = if is_success {
        "bg-white rounded-2xl border overflow-hidden save-glow-card"
    } else {
        "bg-white rounded-2xl border border-amber-100 shadow-sm overflow-hidden"
    };

    let header_class = if is_success {
        "px-5 py-3.5 border-b flex items-center gap-2.5 save-glow-header"
    } else {
        "px-5 py-3.5 bg-amber-50/70 border-b border-amber-100 flex items-center gap-2.5"
    };

    let accent_class = if is_success {
        "w-1.5 h-5 rounded-full save-glow-accent"
    } else {
        "w-1.5 h-5 rounded-full bg-amber-400/70"
    };

    const LBL: &str =
        "block text-[11px] font-semibold tracking-[0.12em] uppercase text-amber-700/70 mb-1.5";

    rsx! {
        div { class: "{card_class}",

            // Card header
            div { class: "{header_class}",
                div { class: "{accent_class}" }
                div { class: "flex items-center gap-2",
                    {svg_icon}
                    span { class: "text-sm font-semibold text-zinc-800", "{title}" }
                }
            }

            // Card body
            div { class: "px-5 py-5 space-y-4",

                // Enable toggle
                label { class: "flex items-center gap-2.5 cursor-pointer group w-fit",
                    input {
                        r#type: "checkbox",
                        checked: has_point_signal,
                        class: "accent-[#D67229] w-4 h-4 rounded cursor-pointer",
                        onclick: move |_| {
                            let toggle = *has_point_signal.read();
                            has_point_signal.set(!toggle);
                        },
                    }
                    span { class: "text-[13px] text-zinc-600 group-hover:text-zinc-800 transition-colors",
                        "{toggle_label}"
                    }
                }

                // Name + Time – dimmed when disabled
                div { class: "space-y-3 {disabled_cls} transition-opacity duration-150",
                    div {
                        label { class: "{LBL}", "Name" }
                        Input {
                            value: name_signal,
                            place_holer: "{title}",
                            is_error: !name_error_signal.read().is_empty(),
                            oninput: move |e| on_name_input.call(e),
                        }
                        InputError { error: name_error_signal.read() }
                    }
                    div {
                        label { class: "{LBL}", "Time" }
                        InputTime {
                            value: time_signal,
                            is_error: false,
                            oninput: move |e| on_time_input.call(e),
                        }
                    }
                }

                // Address – dimmed when disabled
                div { class: "{disabled_cls} transition-opacity duration-150",
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
