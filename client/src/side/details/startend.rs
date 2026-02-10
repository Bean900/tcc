use chrono::NaiveTime;
use dioxus::prelude::*;
use uuid::Uuid;
use web_sys::console;

use crate::{
    async_action,
    side::{
        details::{address::Address, ErrorPage, LoadingPage},
        AsyncAction, ConfirmButton, EndSVG, Headline1, Headline2, Input, InputError, InputTime,
        StartSVG,
    },
    storage::{AddressData, MeetingPointData, StorageManager},
};

use super::address::AddressParam;

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
            .map_or(NaiveTime::from_hms_opt(0, 0, 0).unwrap(), |p| {
                p.time.clone()
            })
    });
    let mut end_time_signal = use_signal(|| {
        end_point
            .as_ref()
            .map_or(NaiveTime::from_hms_opt(0, 0, 0).unwrap(), |p| {
                p.time.clone()
            })
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
                &start_name_signal.read(),
                &end_name_signal.read(),
                *start_time_signal.read(),
                *end_time_signal.read(),
                start_adress_param.clone(),
                end_adress_param.clone(),
            )
            .await;
            if let Err(e) = &start_update {
                console::error_1(&format!("Failed to update start point: {}", e).into());
                save_response_error_signal.set("Failed to save start/end point data".to_string());
            }
            if let Err(e) = &end_update {
                console::error_1(&format!("Failed to update end point: {}", e).into());
                save_response_error_signal.set("Failed to save start/end point data".to_string());
            }
        }
    });

    rsx! {
        section {
            Headline1 { headline: "Start & End Point".to_string() }

            div { class: "grid grid-cols-1 md:grid-cols-2 gap-6",

                // Start Point
                div { class: "bg-[#fdfaf6] shadow rounded-xl p-4 border w-100 h-160",

                    h3 { class: "text-lg font-semibold mb-2 flex items-center justify-between",
                        div { class: "flex items-center space-x-2",
                            StartSVG {}
                            Headline2 { headline: "Start Point".to_string() }
                        }
                    
                    }

                    label { class: "inline-flex items-center space-x-2 text-[#3B3B3B] font-sans leading-relaxed text-base mb-4",
                        input {
                            r#type: "checkbox",
                            checked: start_has_point_signal,
                            class: "rounded",
                            onclick: move |_| {
                                let checkbox_state = !*start_has_point_signal.read();
                                start_has_point_signal.set(checkbox_state);
                            },
                        }
                        span { "Use start point" }
                    }

                    div {
                        class: {
                            format!(
                                "mb-3 {}",
                                if *start_has_point_signal.read() {
                                    ""
                                } else {
                                    "opacity-50 pointer-events-none"
                                },
                            )
                        },
                        Input {
                            value: start_name_signal,
                            place_holer: "Start".to_string(),
                            is_error: !start_name_error_signal.read().is_empty(),
                            oninput: move |event: Event<FormData>| {
                                let name = check_name(&event.value(), start_name_error_signal.clone());
                                start_name_signal.set(name);
                            },
                        }
                        InputError { error: start_name_error_signal.read() }
                        InputTime {
                            value: start_time_signal,
                            is_error: false,
                            oninput: move |event: Event<FormData>| {
                                let time = check_time(&event.value());
                                match time {
                                    Some(t) => start_time_signal.set(t),
                                    None => {}
                                }
                            },
                        }
                    }

                    div {
                        class: {
                            format!(
                                "{}",
                                if *start_has_point_signal.read() {
                                    ""
                                } else {
                                    "opacity-50 pointer-events-none"
                                },
                            )
                        },
                        Address { param: start_adress_param }
                    }
                }

                // End Point
                div { class: "bg-[#fdfaf6] shadow rounded-xl p-4 border w-100 h-160",

                    h3 { class: "text-lg font-semibold mb-2 flex items-center justify-between",
                        div { class: "flex items-center space-x-2",
                            EndSVG {}
                            Headline2 { headline: "End Point".to_string() }
                        }
                    
                    }

                    label { class: "inline-flex items-center space-x-2 text-[#3B3B3B] font-sans leading-relaxed text-base mb-4",
                        input {
                            r#type: "checkbox",
                            checked: end_has_point_signal,
                            class: "rounded",
                            onclick: move |_| {
                                let checkbox_state = !*end_has_point_signal.read();
                                end_has_point_signal.set(checkbox_state);
                            },
                        }
                        span { "Use end point" }
                    }

                    div {
                        class: {
                            format!(
                                "mb-3 {}",
                                if *end_has_point_signal.read() {
                                    ""
                                } else {
                                    "opacity-50 pointer-events-none"
                                },
                            )
                        },

                        Input {
                            value: end_name_signal,
                            place_holer: "End".to_string(),
                            is_error: !end_name_error_signal.read().is_empty(),
                            oninput: move |event: Event<FormData>| {
                                let name = check_name(&event.value(), end_name_error_signal.clone());
                                end_name_signal.set(name);
                            },
                        }
                        InputError { error: end_name_error_signal.read() }
                        InputTime {
                            value: end_time_signal,
                            is_error: false,
                            oninput: move |event: Event<FormData>| {
                                let time = check_time(&event.value());
                                match time {
                                    Some(t) => end_time_signal.set(t),
                                    None => {}
                                }
                            },
                        }
                    }

                    div {
                        class: {
                            format!(
                                "{}",
                                if *end_has_point_signal.read() {
                                    ""
                                } else {
                                    "opacity-50 pointer-events-none"
                                },
                            )
                        },
                        Address { param: end_adress_param }
                    }
                }
            }

            if !save_response_error_signal.read().is_empty() {
                div { class: "bg-red-50 border border-red-200 rounded-xl p-4 mt-6 text-red-700 font-sans",
                    "{save_response_error_signal.read()}"
                }
            }

            div { class: "flex justify-end w-full mt-8 pr-2",
                ConfirmButton { action: on_save, text: "Save".to_string() }
            }
        }
    }
}

fn check_name(name: &str, mut error_signal: Signal<String>) -> String {
    let trim_name = name.trim();
    if trim_name.is_empty() {
        error_signal.set("Name can not be empty!".to_string());
    } else {
        error_signal.set("".to_string());
    }
    trim_name.to_string()
}

fn check_time(time_str: &str) -> Option<NaiveTime> {
    let time = NaiveTime::parse_from_str(time_str, "%H:%M");
    match time {
        Ok(time) => Some(time),

        Err(e) => {
            console::error_1(&format!("Time format is not correct: {}", e,).into());
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
        address: address,
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
        let start_point_data =
            to_meeting_point_data(start_name, start_time, start_adress_param.clone());
        Some(start_point_data)
    } else {
        None
    };

    let end_point = if end_has_point {
        let end_point_data = to_meeting_point_data(end_name, end_time, end_adress_param.clone());
        Some(end_point_data)
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
    console::log_1(&"Finished saving point data.".into());
    (start_update, end_update)
}
