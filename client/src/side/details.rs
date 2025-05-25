use std::sync::{Arc, Mutex};

use chrono::{DateTime, Utc};
use dioxus::{html::g::r, prelude::*};
use uuid::Uuid;
use web_sys::console;

use crate::storage::{LocalStorage, StorageR, StorageW};

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
    let storage = use_context::<Arc<Mutex<LocalStorage>>>();
    let storage = storage.lock().expect("Expected storage lock");
    let cook_and_run = storage.select_cook_and_run(id);

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
                                    name: cook_and_run.name,
                                    uploaded: false,
                                },
                            )
                        }
                        MenuPage::Teams => {
                            Teams(
                                &TeamsProps {
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

struct OverviewProps {
    name: String,
    uploaded: bool,
}

#[component]
fn Overview(props: &OverviewProps) -> Element {
    //let storage = use_context::<Arc<Mutex<LocalStorage>>>();
    // let storage = storage.lock().expect("Expected storage lock");

    let mut delete_dialog_signal: Signal<Element> = use_signal(|| rsx!());
    rsx! {
        section {
            h2 { class: "text-2xl font-bold mb-4", "Overview" }
            input {
                class: "border p-2 rounded w-full mb-4",
                r#type: "text",
                placeholder: "Project name",
                value: if !props.name.is_empty() { props.name.clone() },
            }
            div { class: "flex flex-wrap gap-4 mb-4",
                div { class: "px-4",
                    if props.uploaded {
                        button { class: "bg-red-500 text-white px-4 py-2 rounded hover:bg-red-600 cursor-pointer",
                            "Remove from Cloud"
                        }
                    } else {
                        button { class: "bg-blue-500 text-white px-4 py-2 rounded hover:bg-blue-600 cursor-pointer",
                            "Upload to Cloud"
                        }
                    }
                }

                div { class: "ml-auto px-4",
                    button {
                        class: "border border-red-500 text-red-500 px-4 py-2 rounded hover:bg-red-100 cursor-pointer",
                        onclick: move |_| { delete_dialog_signal.set(rsx! {
                            {delete_project_dialog(delete_dialog_signal, Uuid::new_v4())}
                        }) },
                        "Delete Project"
                    }
                }
                div { class: "bg-yellow-100 border border-yellow-300 text-yellow-800 p-4 rounded max-w-xl",
                    h3 { class: "font-bold mb-2", "Cloud Info" }
                    p { "Projects stored only locally ensure your data stays on your machine." }
                    p {
                        "Uploading to the cloud enables live access to route sheets, syncing across devices, and backups."
                    }
                    p { class: "mt-2 font-semibold",
                        "Note: Cloud functionality requires you to be logged in."
                    }
                }
            }
        }
        {delete_dialog_signal}
    }
}

fn delete_project_dialog(mut delete_project_signal: Signal<Element>, project_id: Uuid) -> Element {
    rsx! {
        div { class: "backdrop-blur fixed inset-0 flex h-screen w-screen justify-center items-center",
            div { class: "relative bg-white shadow-md rounded-xl p-6 h-54 hover:shadow-lg transition-all cursor-pointer ",

                // Title
                h2 { class: "text-2xl font-semibold text-red-800 mb-4", "Delete Project" }

                p { class: "text-red-600 font-semibold mb-4", "Warning:" }
                p { class: "text-red-600 font-semibold mb-4",
                    "Deleting this project will permanently and irreversibly remove it. This action cannot be undone."
                }

                // Delete button
                button {
                    class: "absolute top-3 right-3 cursor-pointer",
                    onclick: move |_| {
                        delete_project_signal.set(rsx! {});
                    },
                    svg {
                        class: "w-6 h-6",
                        stroke: "currentColor",
                        xmlns: "http://www.w3.org/2000/svg",
                        view_box: "0 0 24 24",
                        path { d: "M6 18L18 6M6 6l12 12" }
                    }
                }

                button {
                    class: "bg-red-500 text-white px-4 py-2 rounded hover:bg-red-600 cursor-pointer",
                    onclick: move |_| {
                        delete_project_signal.set(rsx! {});
                    },
                    "Delete Project"
                }
            }
        }
    }
}

struct TeamsProps {
    team_list: Vec<TeamProps>,
}

#[component]
fn Teams(props: &TeamsProps) -> Element {
    {
        rsx! {
            section {
                h2 { class: "text-2xl font-bold mb-4", "Teams" }

                // Buttons
                div { class: "flex space-x-4 mb-4",
                    button { class: "bg-blue-500 text-white px-4 py-2 rounded hover:bg-blue-600",
                        "+ Add Team"
                    }
                    button { class: "bg-green-500 text-white px-4 py-2 rounded hover:bg-green-600",
                        "Import from Excel"
                    }
                }

                // Scrollable grid
                div { class: "grid grid-cols-1 md:grid-cols-2 xl:grid-cols-3 gap-4 max-h-[calc(100vh-16rem)] overflow-y-auto pr-2",
                    //add here
                    {props.team_list.iter().map(|team| { Team(team) })}
                }
            }
        }
    }
}

struct TeamProps {
    id: Uuid,
    name: String,
    address: String,
    allergies: Vec<String>,
}

#[component]
fn Team(props: &TeamProps) -> Element {
    rsx! {
        div { key: {props.id}, class: "border rounded bg-white p-4 shadow",
            h3 { class: "text-lg font-semibold mb-1", "{props.name}" }
            p { class: "text-sm text-gray-600 mb-1", "📍 {props.address}" }
            if !props.allergies.is_empty() {
                p { class: "text-sm text-red-600", "⚠ Allergies: {props.allergies.join(\", \")}" }
            }
        }
    }
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
