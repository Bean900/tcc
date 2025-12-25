use chrono::NaiveTime;
use dioxus::prelude::*;
use uuid::Uuid;
use web_sys::console;

use crate::{
    side::{
        debounce,
        details::{address::Address, ErrorPage, LoadingPage},
        EndSVG, Headline1, Headline2, Input, InputError, InputTime, StartSVG,
    },
    storage::{MeetingPointData, StorageManager},
};

use super::address::AddressParam;

#[component]
pub fn StartEnd(cook_and_run_id: Uuid) -> Element {
    let storage = use_context::<Signal<StorageManager>>();
    let team_list: Resource<Result<(Option<MeetingPointData>, Option<MeetingPointData>), String>> =
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

    match &*team_list.read_unchecked() {
        None => rsx!(
            LoadingPage {}
        ),
        Some(Err(e)) => rsx!(
            ErrorPage { error_text: e }
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

    let mut start_name_error_signal = use_signal(|| "".to_string());
    let mut end_name_error_signal = use_signal(|| "".to_string());

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
                                let name = &event.value();
                                start_name_signal.set(name.clone());
                                if name.trim().is_empty() {
                                    start_name_error_signal.set("Name can not be empty!".to_string());
                                }
                            },
                        }
                        InputError { error: start_name_error_signal.read() }
                        InputTime {
                            value: start_time_signal,
                            is_error: false,
                            oninput: move |event: Event<FormData>| {
                                let time = NaiveTime::parse_from_str(&event.value(), "%H:%M");
                                if time.is_err() {
                                    console::error_1(
                                        &format!(
                                            "Time format is not correct: {}",
                                            time.expect_err("Expect error"),
                                        )
                                            .into(),
                                    );
                                    return;
                                }
                                let time = time.expect("Expect time");
                                start_time_signal.set(time);
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
                                let name = &event.value();
                                end_name_signal.set(name.clone());
                                if name.trim().is_empty() {
                                    end_name_error_signal.set("Name can not be empty!".to_string());
                                }
                            },
                        }
                        InputError { error: end_name_error_signal.read() }
                        InputTime {
                            value: end_time_signal,
                            is_error: false,
                            oninput: move |event: Event<FormData>| {
                                let time = NaiveTime::parse_from_str(&event.value(), "%H:%M");
                                if time.is_err() {
                                    console::error_1(
                                        &format!(
                                            "Time format is not correct: {}",
                                            time.expect_err("Expect error"),
                                        )
                                            .into(),
                                    );
                                    return;
                                }
                                let time = time.expect("Expect time");
                                end_time_signal.set(time);
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
        }
    }
}
