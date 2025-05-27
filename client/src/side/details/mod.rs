mod overview;
mod teams;

use overview::{Overview, OverviewProps};
use teams::{TeamProps, Teams, TeamsProps};

use crate::{
    side::{BlueButton, CloseButton, GreenButton, Input, InputError, RedButton, RedHollowButton},
    storage::{CookAndRunData, LocalStorage, StorageR, StorageW},
    Route,
};
use chrono::{DateTime, Utc};
use dioxus::prelude::*;
use std::sync::{Arc, Mutex};
use uuid::Uuid;
use web_sys::console;

fn get_cook_and_run_data(id: Uuid) -> Result<CookAndRunData, String> {
    let storage = use_context::<Arc<Mutex<LocalStorage>>>();
    let storage = storage.lock().expect("Expected storage lock");
    let cook_and_run = storage.select_cook_and_run(id);
    cook_and_run
}

#[derive(PartialEq, Clone)]
enum MenuPage {
    Overview,
    Teams,
    StartEnd,
    Courses,
    Calculation,
}

#[component]
pub fn ProjectDetailPage(id: Uuid) -> Element {
    let cook_and_run = get_cook_and_run_data(id);
    if cook_and_run.is_err() {
        console::error_1(
            &format!(
                "Error loading cook and run data: {}",
                cook_and_run.err().expect("Expected error")
            )
            .into(),
        );
        return rsx! {
            div { "Error loading data" }
        };
    }

    let cook_and_run = cook_and_run.expect("Expected cook and run data");

    let mut current_page = use_signal(|| MenuPage::Overview);
    rsx! {
        div { class: "flex h-screen w-full",
            // Sidebar
            {get_side_bar(current_page)}
            // Main Content
            div { class: "flex justify-center w-full",
                div { class: "py-4",
                    match current_page() {
                        MenuPage::Overview => {
                            Overview(
                                &OverviewProps {
                                    id: cook_and_run.id,
                                    name: cook_and_run.name,
                                    uploaded: false,
                                },
                            )
                        }
                        MenuPage::Teams => {
                            Teams(
                                &TeamsProps {
                                    project_id: cook_and_run.id,
                                    team_list: cook_and_run
                                        .contact_list
                                        .iter()
                                        .map(|contact| {
                                            TeamProps {
                                                id: contact.id,
                                                name: contact.team_name.clone(),
                                                address: contact.address.clone(),
                                                allergies: contact.allergies.clone(),
                                            }
                                        })
                                        .collect(),
                                },
                            )
                        }
                        MenuPage::StartEnd => todo!(),
                        MenuPage::Courses => todo!(),
                        MenuPage::Calculation => rsx! {
                            section { class: "mb-8",
                                h2 { class: "text-2xl font-bold mb-4", "Berechnung" }
                                div { class: "flex space-x-4 items-center",
                                    button { class: "bg-purple-500 text-white px-4 py-2 rounded hover:bg-purple-600",
                                        "Berechnung starten"
                                    }
                                    a { href: "#", class: "text-blue-500 underline", "Laufzettel anzeigen" }
                                }
                            }
                        },
                    }
                }
            }
        }
    }
}

fn get_side_bar(mut current_page: Signal<MenuPage>) -> Element {
    rsx!(
        nav { class: "w-64 bg-gray-100 p-4 border-r border-gray-300",

            h2 { class: "text-xl font-bold mb-4", "Menü" }
            ul { class: "space-y-2",
                li {
                    button {
                        class: "block text-left text-gray-700 hover:text-blue-500 w-full",
                        onclick: move |_| current_page.set(MenuPage::Overview),
                        "Overview"
                    }
                }
                li {
                    button {
                        class: "block text-left text-gray-700 hover:text-blue-500 w-full",
                        onclick: move |_| current_page.set(MenuPage::Teams),
                        "Teams"
                    }
                }
                li {
                    button {
                        class: "block text-left text-gray-700 hover:text-blue-500 w-full",
                        onclick: move |_| current_page.set(MenuPage::StartEnd),
                        "Start and end point"
                    }
                }
                li {
                    button {
                        class: "block text-left text-gray-700 hover:text-blue-500 w-full",
                        onclick: move |_| current_page.set(MenuPage::Courses),
                        "Courses"
                    }
                }
                li {
                    button {
                        class: "block text-left text-gray-700 hover:text-blue-500 w-full",
                        onclick: move |_| current_page.set(MenuPage::Calculation),
                        "Calculation"
                    }
                }
            
            }
        }
    )
}

struct StartEndProps {
    start: Option<DateTime<Utc>>,
    end: Option<DateTime<Utc>>,
    start_address: Option<String>,
    end_address: Option<String>,
    is_start: bool,
    is_end: bool,
}

#[component]
fn StartEnd(props: &StartEndProps) -> Element {
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
