use chrono::{NaiveTime, Utc};
use dioxus::prelude::*;
use uuid::Uuid;

use crate::{
    side::{details::address::Address, Input, InputTime},
    storage::{AddressData, MeetingPointData},
};

use super::address::AddressParam;

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
        }
    }
    fn to_data(&self) -> (Option<MeetingPointData>, Option<MeetingPointData>) {
        todo!()
    }
}

#[component]
pub fn StartEnd(param: StartEndParam) -> Element {
    rsx! {
        section {
            h2 { class: "text-2xl font-bold mb-6", "Start & End Point" }

            div { class: "grid grid-cols-1 md:grid-cols-2 gap-6",

                // Startpunkt
                div { class: "bg-white shadow rounded-xl p-4 border w-100 h-160",

                    h3 { class: "text-lg font-semibold mb-2 flex items-center space-x-2",
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

                    label { class: "inline-flex items-center mb-3 space-x-2",
                        input {
                            r#type: "checkbox",
                            checked: param.is_start,
                            class: "text-blue-600 rounded",
                            onclick: move |_| {},
                        }
                        span { "Use as start" }
                    }

                    div { class: "mb-3",
                        InputTime { value: param.start_signal, oninput: |event| {} }
                    }

                    Address { param: param.start_address }
                }

                // Zielpunkt
                div { class: "bg-white shadow rounded-xl p-4 border w-100 h-160",

                    h3 { class: "text-lg font-semibold mb-2 flex items-center space-x-2",
                        svg {
                            class: "w-5 h-5 text-green-600",
                            xmlns: "http://www.w3.org/2000/svg",
                            fill: "none",
                            view_box: "0 0 24 24",
                            stroke_width: "1.5",
                            stroke: "currentColor",

                            path {
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                d: "M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z",
                            }
                        }
                        span { "End Point" }
                    }

                    label { class: "inline-flex items-center mb-3 space-x-2",
                        input {
                            r#type: "checkbox",
                            checked: param.is_end,
                            class: "text-blue-600 rounded",
                            onclick: move |_| {},
                        }
                        span { "Use as destination" }
                    }

                    div { class: "mb-3",
                        InputTime { value: param.end_signal, oninput: |event| {} }
                    }

                    Address { param: param.end_address }
                }
            }
        }
    }
}
