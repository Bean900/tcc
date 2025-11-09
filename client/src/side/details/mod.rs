mod address;
//mod calculate;
//mod courses;
pub mod overview;
//mod share_team;
//mod startend;
pub mod teams;

use dioxus::prelude::*;
use uuid::Uuid;
use web_sys::console;

use crate::Route;

/*
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
        team_list: cook_and_run.team_list,
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
}*/

#[component]
pub fn Menu(cook_and_run_id: Uuid) -> Element {
    let current_route = use_route::<Route>();

    rsx!(
        nav { class: "w-64 h-full bg-[#F8EFE1] p-6 shadow-md rounded-r-2xl flex flex-col",

            h2 { class: "text-xl font-semibold text-[#70513E] mb-6 tracking-wide",
                "Menu"
            }

            ul { class: "space-y-3",

                SidebarButton {
                    label: "Overview",
                    target_route: Route::Overview { cook_and_run_id },
                    current_route: current_route.clone(),
                }
                SidebarButton {
                    label: "Teams",
                    target_route: Route::Teams { cook_and_run_id },
                    current_route: current_route.clone(),
                }
                SidebarButton {
                    label: "Start & End",
                    target_route: Route::Overview { cook_and_run_id },
                    current_route: current_route.clone(),
                }
                SidebarButton {
                    label: "Courses",
                    target_route: Route::Overview { cook_and_run_id },
                    current_route: current_route.clone(),
                }
                SidebarButton {
                    label: "Calculation",
                    target_route: Route::Overview { cook_and_run_id },
                    current_route: current_route.clone(),
                }
            }
        }
        Outlet::<Route> {}
    )
}

#[component]
fn SidebarButton(label: String, target_route: Route, current_route: Route) -> Element {
    let base = "block text-left w-full px-4 py-2 rounded-lg transition-colors duration-200";
    let active = "bg-[#D67229] text-white";
    let inactive = "text-[#70513E] hover:text-[#C66741] hover:bg-[#F1E7D7]";
    let is_active = target_route.eq(&current_route);
    rsx! {
        li {
            button {
                class: format_args!("{} {}", base, if is_active { active } else { inactive }),
                onclick: move |_| {
                    let t = target_route.clone();
                    use_navigator().push(t);
                },
                "{label}"
            }
        }
    }
}

#[component]
fn LoadingPage() -> Element {
    rsx! { "Loading..." }
}

#[component]
fn ErrorPage(error_text: String) -> Element {
    rsx! { "Error: {error_text}" }
}
