use std::sync::{Arc, Mutex};

use chrono::NaiveTime;
use dioxus::{html::script::r#async, prelude::*};
use uuid::Uuid;
use web_sys::console;

use crate::{
    side::{debounce, details::address::Address, InputTime},
    storage::{LocalStorage, MeetingPointData, StorageW},
};

use super::address::AddressParam;

fn update_start_point_in_cook_and_run(
    id: Uuid,
    start_signal: Signal<Option<MeetingPointData>>,
    start_saving_signal: Signal<bool>,
    start_saving_error_signal: Signal<String>,
    start_data: Result<Option<MeetingPointData>, String>,
) {
    start_saving_error_signal.clone().set("".to_string());
    if start_data.is_err() {
        start_saving_error_signal
            .clone()
            .set(start_data.expect_err("Expect error"));
        return;
    }
    let start_data = start_data.expect("Expact data");
    start_signal.clone().set(start_data.clone());
    debounce(start_signal, start_saving_signal, move |data| {
        let storage = use_context::<Arc<Mutex<LocalStorage>>>();
        let mut storage = storage.lock().expect("Expected storage lock");
        let result = storage.update_start_point_in_cook_and_run(id, data);
        if result.is_err() {
            console::error_1(
                &format!(
                    "Error while saving data: {}",
                    result.expect_err("Expect error"),
                )
                .into(),
            );
            start_saving_error_signal
                .clone()
                .set("Error while saving data!".to_string());
        }
    });
}

fn update_end_point_in_cook_and_run(
    id: Uuid,
    end_signal: Signal<Option<MeetingPointData>>,
    end_saving_signal: Signal<bool>,
    end_saving_error_signal: Signal<String>,
    end_data: Result<Option<MeetingPointData>, String>,
) {
    end_saving_error_signal.clone().set("".to_string());
    if end_data.is_err() {
        end_saving_error_signal
            .clone()
            .set(end_data.expect_err("Expect error"));
        return;
    }
    let end_data = end_data.expect("Expact data");
    end_signal.clone().set(end_data.clone());
    debounce(end_signal, end_saving_signal, move |data| {
        let storage = use_context::<Arc<Mutex<LocalStorage>>>();
        let mut storage = storage.lock().expect("Expected storage lock");
        let result = storage.update_goal_point_in_cook_and_run(id, data);
        if result.is_err() {
            console::error_1(
                &format!(
                    "Error while saving data: {}",
                    result.expect_err("Expect error"),
                )
                .into(),
            );
            end_saving_error_signal
                .clone()
                .set("Error while saving data!".to_string());
        }
    });
}

#[derive(PartialEq, Clone, Copy)]
pub struct StartEndParam {
    project_id: Uuid,
    start_signal: Signal<NaiveTime>,
    start_error_signal: Signal<String>,
    end_signal: Signal<NaiveTime>,
    end_error_signal: Signal<String>,
    start_address: AddressParam,
    end_address: AddressParam,
    is_start: Signal<bool>,
    is_end: Signal<bool>,
    start_data_signal: Signal<Option<MeetingPointData>>,
    end_data_signal: Signal<Option<MeetingPointData>>,

    start_saving_signal: Signal<bool>,
    start_saving_error_signal: Signal<String>,
    end_saving_signal: Signal<bool>,
    end_saving_error_signal: Signal<String>,
}

impl StartEndParam {
    pub fn new(
        project_id: Uuid,
        start_point: &Option<MeetingPointData>,
        end_point: &Option<MeetingPointData>,
    ) -> Self {
        let (start_signal, start_address, is_start) = if let Some(start_point) = start_point {
            (
                use_signal(|| start_point.time),
                AddressParam::new(&start_point.address),
                use_signal(|| true),
            )
        } else {
            (
                use_signal(|| NaiveTime::from_hms_opt(00, 00, 00).expect("Expect time!")),
                AddressParam::default(),
                use_signal(|| false),
            )
        };
        let (end_signal, end_address, is_end) = if let Some(end_point) = end_point {
            (
                use_signal(|| end_point.time),
                AddressParam::new(&end_point.address),
                use_signal(|| true),
            )
        } else {
            (
                use_signal(|| NaiveTime::from_hms_opt(00, 00, 00).expect("Expect time!")),
                AddressParam::default(),
                use_signal(|| false),
            )
        };

        let start_error_signal = use_signal(|| "".to_string());
        let end_error_signal = use_signal(|| "".to_string());

        let start_data_signal = use_signal(|| start_point.clone());
        let end_data_signal = use_signal(|| end_point.clone());

        let start_saving_signal = use_signal(|| false);
        let start_saving_error_signal = use_signal(|| "".to_string());
        let end_saving_signal = use_signal(|| false);
        let end_saving_error_signal = use_signal(|| "".to_string());

        Self {
            project_id,
            start_signal,
            start_error_signal,
            end_signal,
            end_error_signal,
            start_address,
            end_address,
            is_start,
            is_end,
            start_data_signal,
            end_data_signal,

            start_saving_signal,
            start_saving_error_signal,
            end_saving_signal,
            end_saving_error_signal,
        }
    }
    fn to_start_data(&self) -> (Result<Option<MeetingPointData>, String>) {
        if !*self.is_start.read() {
            return Ok(None);
        }
        let address = self.start_address.get_address_data();
        if address.is_err() {
            return Err("Data is incomplete".to_string());
        }

        return Ok(Some(MeetingPointData {
            time: self.start_signal.read().clone(),
            address: address.expect("Expect address to be set!"),
        }));
    }

    fn to_end_data(&self) -> (Result<Option<MeetingPointData>, String>) {
        if !*self.is_end.read() {
            return Ok(None);
        }
        let address = self.end_address.get_address_data();
        if address.is_err() {
            return Err("Data is incomplete".to_string());
        }

        return Ok(Some(MeetingPointData {
            time: self.end_signal.read().clone(),
            address: address.expect("Expect address to be set!"),
        }));
    }
}

#[component]
pub fn StartEnd(param: StartEndParam) -> Element {
    {
        let (latitude_start_addr_signal, longitude_start_addr_signal, address_start_addr_signal) =
            param.start_address.get_data_signals();

        use_effect(move || {
            to_owned![
                latitude_start_addr_signal,
                longitude_start_addr_signal,
                address_start_addr_signal
            ];

            let data = param.to_start_data();
            update_start_point_in_cook_and_run(
                param.project_id,
                param.start_data_signal,
                param.start_saving_signal,
                param.start_saving_error_signal,
                data,
            );
        });
    }

    {
        let (latitude_end_addr_signal, longitude_end_addr_signal, address_end_addr_signal) =
            param.end_address.get_data_signals();

        use_effect(move || {
            to_owned![
                latitude_end_addr_signal,
                longitude_end_addr_signal,
                address_end_addr_signal
            ];

            let data = param.to_end_data();
            update_end_point_in_cook_and_run(
                param.project_id,
                param.end_data_signal,
                param.end_saving_signal,
                param.end_saving_error_signal,
                data,
            );
        });
    }

    rsx! {
        section {
            h2 { class: "text-2xl font-bold mb-6", "Start & End Point" }

            div { class: "grid grid-cols-1 md:grid-cols-2 gap-6",

                // Start Point
                div { class: "bg-white shadow rounded-xl p-4 border w-100 h-160",

                    h3 { class: "text-lg font-semibold mb-2 flex items-center justify-between",
                        div { class: "flex items-center space-x-2",
                            svg {
                                class: "w-5 h-5 text-blue-600",
                                xmlns: "http://www.w3.org/2000/svg",
                                fill: "none",
                                view_box: "0 0 24 24",
                                stroke_width: "1.5",
                                stroke: "currentColor",

                                path {
                                    stroke_linecap: "round",
                                    stroke_linejoin: "round",
                                    d: "M12 4.5v15m0-15l-3 3m3-3l3 3",
                                }
                            }
                            span { "Start Point" }
                        }

                        // Loading or Error Icon
                        div { class: "flex items-center space-x-2",

                            if !param.start_saving_error_signal.read().is_empty() {
                                div { title: param.start_saving_error_signal,
                                    svg {
                                        class: "w-5 h-5 text-red-600",
                                        xmlns: "http://www.w3.org/2000/svg",
                                        fill: "none",
                                        view_box: "0 0 24 24",
                                        stroke_width: "2",
                                        stroke: "currentColor",

                                        circle {
                                            cx: "12",
                                            cy: "12",
                                            r: "10",
                                            stroke: "currentColor",
                                            stroke_width: "2",
                                        }
                                        line {
                                            x1: "12",
                                            y1: "8",
                                            x2: "12",
                                            y2: "12",
                                            stroke: "currentColor",
                                            stroke_width: "2",
                                            stroke_linecap: "round",
                                        }
                                        line {
                                            x1: "12",
                                            y1: "16",
                                            x2: "12",
                                            y2: "16",
                                            stroke: "currentColor",
                                            stroke_width: "2",
                                            stroke_linecap: "round",
                                        }
                                    }
                                
                                }
                            } else if *param.start_saving_signal.read() {
                                div {
                                    class: "loader w-5 h-5 border-2 border-blue-600 border-t-transparent rounded-full animate-spin",
                                    title: "Saving in progress...",
                                }
                            }
                        }
                    }

                    label { class: "inline-flex items-center mb-3 space-x-2",
                        input {
                            r#type: "checkbox",
                            checked: param.is_start,
                            class: "text-blue-600 rounded",
                            onclick: move |_| {
                                let checkbox_state = !*param.is_start.read();
                                param.is_start.set(checkbox_state);
                            },
                        }
                        span { "Use start point" }
                    }

                    div {
                        class: {
                            format!(
                                "mb-3 {}",
                                if *param.is_start.read() { "" } else { "opacity-50 pointer-events-none" },
                            )
                        },
                        InputTime {
                            value: param.start_signal,
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
                                param.start_signal.set(time);
                            },
                        }
                    }

                    div {
                        class: {
                            format!(
                                "{}",
                                if *param.is_start.read() { "" } else { "opacity-50 pointer-events-none" },
                            )
                        },
                        Address { param: param.start_address }
                    }
                }

                // Goal Point
                div { class: "bg-white shadow rounded-xl p-4 border w-100 h-160",



                    h3 { class: "text-lg font-semibold mb-2 flex items-center justify-between",
                        div { class: "flex items-center space-x-2",
                            svg {
                                class: "w-5 h-5 text-blue-600",
                                xmlns: "http://www.w3.org/2000/svg",
                                fill: "none",
                                view_box: "0 0 24 24",
                                stroke_width: "1.5",
                                stroke: "currentColor",

                                path {
                                    stroke_linecap: "round",
                                    stroke_linejoin: "round",
                                    d: "M6 3v18m0-18c2.5 0 5 1.5 7.5 1.5S21 3 21 3v12s-2.5-1.5-5-1.5S6 15 6 15",
                                }
                            }
                            span { "End Point" }
                        }

                        // Loading or Error Icon
                        div { class: "flex items-center space-x-2",

                            if !param.end_saving_error_signal.read().is_empty() {
                                div { title: param.end_saving_error_signal,
                                    svg {
                                        class: "w-5 h-5 text-red-600",
                                        xmlns: "http://www.w3.org/2000/svg",
                                        fill: "none",
                                        view_box: "0 0 24 24",
                                        stroke_width: "2",
                                        stroke: "currentColor",

                                        circle {
                                            cx: "12",
                                            cy: "12",
                                            r: "10",
                                            stroke: "currentColor",
                                            stroke_width: "2",
                                        }
                                        line {
                                            x1: "12",
                                            y1: "8",
                                            x2: "12",
                                            y2: "12",
                                            stroke: "currentColor",
                                            stroke_width: "2",
                                            stroke_linecap: "round",
                                        }
                                        line {
                                            x1: "12",
                                            y1: "16",
                                            x2: "12",
                                            y2: "16",
                                            stroke: "currentColor",
                                            stroke_width: "2",
                                            stroke_linecap: "round",
                                        }
                                    }
                                
                                }
                            } else if *param.end_saving_signal.read() {
                                div {
                                    class: "loader w-5 h-5 border-2 border-blue-600 border-t-transparent rounded-full animate-spin",
                                    title: "Saving in progress...",
                                }
                            }
                        }
                    }


                    label { class: "inline-flex items-center mb-3 space-x-2",
                        input {
                            r#type: "checkbox",
                            checked: param.is_end,
                            class: "text-blue-600 rounded",
                            onclick: move |_| {
                                let checkbox_state = !*param.is_end.read();
                                param.is_end.set(checkbox_state);
                            },
                        }
                        span { "Use end point" }
                    }

                    div {
                        class: {
                            format!(
                                "mb-3 {}",
                                if *param.is_end.read() { "" } else { "opacity-50 pointer-events-none" },
                            )
                        },
                        InputTime {
                            value: param.end_signal,
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
                                param.end_signal.set(time);
                            },
                        }
                    }

                    div {
                        class: {
                            format!(
                                "{}",
                                if *param.is_end.read() { "" } else { "opacity-50 pointer-events-none" },
                            )
                        },
                        Address { param: param.end_address }
                    }
                }
            }
        }
    }
}
