use std::sync::Arc;

use chrono::{DateTime, Utc};
use dioxus::prelude::*;
use uuid::Uuid;

use crate::storage::{LocalStorage, StorageW};

struct ProjectDetail {
    overview: OverviewProps,
    teams: TeamsProps,
    start_end: StartEndProps,
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
    let db = use_context::<Arc<LocalStorage>>();

    let props: ProjectDetail = todo!();
    let mut current_page = use_signal(|| MenuPage::Overview);
    rsx! {
        div { class: "flex h-screen",
            // Sidebar
            {get_side_bar(current_page)}
            // Main Content
            div { class: "ml-64 p-6 overflow-auto",
                match current_page() {
                    MenuPage::Overview => Overview(&props.overview),
                    MenuPage::Teams => Teams(&props.teams),
                    MenuPage::StartEnd => StartEnd(&props.start_end),
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
    id: Uuid,
    name: String,
}

#[component]
fn Overview(props: &OverviewProps) -> Element {
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
                button { class: "bg-green-500 text-white px-4 py-2 rounded hover:bg-green-600",
                    "Save"
                }
                button { class: "bg-blue-500 text-white px-4 py-2 rounded hover:bg-blue-600",
                    "Upload to Cloud"
                }
                button { class: "bg-red-500 text-white px-4 py-2 rounded hover:bg-red-600",
                    "Remove from Cloud"
                }
                button { class: "bg-red-500 text-white px-4 py-2 rounded hover:bg-red-600",
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
