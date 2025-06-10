mod address;
mod calculate;
mod courses;
mod overview;
mod startend;
mod teams;

use courses::CoursesParam;
use overview::Overview;
use startend::{StartEnd, StartEndParam};
use teams::{Teams, TeamsProps};

use crate::storage::{CookAndRunData, LocalStorage, StorageR};
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
pub fn ProjectDetailPage(cook_and_run_id: Uuid) -> Element {
    let cook_and_run = get_cook_and_run_data(cook_and_run_id);
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
    let cook_and_run_overview = cook_and_run.clone();

    let team_props = TeamsProps {
        project_id: cook_and_run_id,
        team_list: cook_and_run.contact_list,
    };

    let start_end_param = StartEndParam::new(
        cook_and_run_id,
        &cook_and_run.start_point,
        &cook_and_run.end_point,
    );

    let courses_param = CoursesParam::new(
        cook_and_run_id,
        cook_and_run.course_list,
        cook_and_run.course_with_more_hosts,
    );

    let current_page = use_signal(|| MenuPage::Overview);

    rsx! {
        div { class: "flex h-screen w-full",
            // Sidebar
            {get_side_bar(current_page)}
            // Main Content
            div { class: "flex justify-center w-full",
                div { class: "py-4",
                    match current_page() {
                        MenuPage::Overview => Overview(cook_and_run_overview),
                        MenuPage::Teams => Teams(&team_props),
                        MenuPage::StartEnd => rsx! {
                            StartEnd { param: start_end_param }
                        },
                        MenuPage::Courses => rsx! {
                            courses::Courses { param: courses_param }
                        },
                        MenuPage::Calculation => rsx! {
                            calculate::Calculate { id: cook_and_run.id }
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
