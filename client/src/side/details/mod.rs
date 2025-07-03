mod address;
mod calculate;
mod courses;
mod overview;
mod share_team;
mod startend;
mod teams;

use courses::CoursesParam;
use overview::Overview;
use startend::{StartEnd, StartEndParam};
use teams::{Teams, TeamsProps};

use crate::storage::{CookAndRunData, LocalStorage, StorageR};
use dioxus::{html::label, prelude::*};
use std::sync::{Arc, Mutex};
use uuid::Uuid;
use web_sys::console;

pub use share_team::ShareTeam;

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
pub fn ProjectOverviewPage(cook_and_run_id: Uuid) -> Element {
    rsx!(
        ProjectDetailPage { cook_and_run_id, menu: MenuPage::Overview }
    )
}

#[component]
pub fn ProjectTeamsPage(cook_and_run_id: Uuid) -> Element {
    rsx!(
        ProjectDetailPage { cook_and_run_id, menu: MenuPage::Teams }
    )
}
#[component]
pub fn ProjectStartEndPage(cook_and_run_id: Uuid) -> Element {
    rsx!(
        ProjectDetailPage { cook_and_run_id, menu: MenuPage::StartEnd }
    )
}
#[component]
pub fn ProjectCoursesPage(cook_and_run_id: Uuid) -> Element {
    rsx!(
        ProjectDetailPage { cook_and_run_id, menu: MenuPage::Courses }
    )
}
#[component]
pub fn ProjectCalculationPage(cook_and_run_id: Uuid) -> Element {
    rsx!(
        ProjectDetailPage { cook_and_run_id, menu: MenuPage::Calculation }
    )
}

#[component]
fn ProjectDetailPage(cook_and_run_id: Uuid, menu: MenuPage) -> Element {
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

    let current_page = use_signal(|| menu.clone());

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
        nav { class: "w-64 h-full bg-[#F8EFE1] p-6 shadow-md rounded-r-2xl flex flex-col",

            h2 { class: "text-xl font-semibold text-[#70513E] mb-6 tracking-wide",
                "Menu"
            }

            ul { class: "space-y-3",

                SidebarButton {
                    label: "Overview",
                    page: MenuPage::Overview,
                    current_page,
                }
                SidebarButton { label: "Teams", page: MenuPage::Teams, current_page }
                SidebarButton {
                    label: "Start and end point",
                    page: MenuPage::StartEnd,
                    current_page,
                }
                SidebarButton {
                    label: "Courses",
                    page: MenuPage::Courses,
                    current_page,
                }
                SidebarButton {
                    label: "Calculation",
                    page: MenuPage::Calculation,
                    current_page,
                }
            }
        }
    )
}

#[component]
fn SidebarButton(label: String, page: MenuPage, current_page: Signal<MenuPage>) -> Element {
    let is_active = current_page.read().eq(&page);
    let base = "block text-left w-full px-4 py-2 rounded-lg transition-colors duration-200";
    let active = "bg-[#D67229] text-white";
    let inactive = "text-[#70513E] hover:text-[#C66741] hover:bg-[#F1E7D7]";

    rsx! {
        li {
            button {
                class: format_args!("{} {}", base, if is_active { active } else { inactive }),
                onclick: move |_| current_page.set(page.clone()),
                "{label}"
            }
        }
    }
}
