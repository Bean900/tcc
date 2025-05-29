use chrono::{DateTime, Utc};
use dioxus::prelude::*;
pub struct StartEndProps {
    pub start: Option<DateTime<Utc>>,
    pub end: Option<DateTime<Utc>>,
    pub start_address: Option<String>,
    pub end_address: Option<String>,
    pub is_start: bool,
    pub is_end: bool,
}

#[component]
pub fn StartEnd(props: &StartEndProps) -> Element {
    rsx! {
        section {
            h2 { class: "text-2xl font-bold mb-4", "Start & End Point" }

            div { class: "space-y-6",

                // Start Point
                div { class: "border p-4 rounded bg-gray-50",
                    div { class: "flex items-center mb-2",
                        input {
                            r#type: "checkbox",
                            class: "mr-2",
                            checked: props.is_start,
                        }
                        span { class: "font-semibold", "Use Start Point" }
                    }
                    div { class: "grid grid-cols-1 md:grid-cols-2 gap-4",
                        input {
                            class: "border p-2 rounded w-full",
                            r#type: "text",
                            placeholder: "Start address",
                            value: if props.start_address.is_some() { props.start_address.clone() },
                        }
                        input {
                            class: "border p-2 rounded w-full",
                            r#type: "time",
                            value: if props.start.is_some() { props.start.expect("Expected start").format("%H:%M").to_string() },
                        }
                    }
                }

                // End Point
                div { class: "border p-4 rounded bg-gray-50",
                    div { class: "flex items-center mb-2",
                        input {
                            r#type: "checkbox",
                            class: "mr-2",
                            checked: props.is_end,
                        }
                        span { class: "font-semibold", "Use End Point" }
                    }
                    div { class: "grid grid-cols-1 md:grid-cols-2 gap-4",
                        input {
                            class: "border p-2 rounded w-full",
                            r#type: "text",
                            placeholder: "End address",
                            value: if props.end_address.is_some() { props.end_address.clone() },
                        }
                        input {
                            class: "border p-2 rounded w-full",
                            r#type: "time",
                            value: if props.end.is_some() { props.end.expect("Expected end").format("%H:%M").to_string() },
                        }
                    }
                }
            }
        }
    }
}
