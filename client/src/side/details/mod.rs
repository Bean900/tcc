mod courses;
mod overview;
mod startend;
mod teams;

mod address;

use courses::CoursesParam;
use overview::{Overview, OverviewProps};
use startend::{StartEnd, StartEndParam};
use teams::{Teams, TeamsProps};

use crate::storage::{CookAndRunData, LocalStorage, StorageR};
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

    let overview_props = OverviewProps {
        id: id,
        name: cook_and_run.name,
        uploaded: false,
    };

    let team_props = TeamsProps {
        project_id: id,
        team_list: cook_and_run.contact_list,
    };

    let start_end_param =
        StartEndParam::new(id, &cook_and_run.start_point, &cook_and_run.end_point);

    let courses_param = CoursesParam::new(id, cook_and_run.course_list);

    let current_page = use_signal(|| MenuPage::Overview);
    rsx! {
        div { class: "flex h-screen w-full",
            // Sidebar
            {get_side_bar(current_page)}
            // Main Content
            div { class: "flex justify-center w-full",
                div { class: "py-4",
                    match current_page() {
                        MenuPage::Overview => Overview(&overview_props),
                        MenuPage::Teams => Teams(&team_props),
                        MenuPage::StartEnd => rsx! {
                            StartEnd { param: start_end_param }
                        },
                        MenuPage::Courses => rsx! {
                            courses::Courses { param: courses_param }
                        },
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
