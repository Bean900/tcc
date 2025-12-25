mod address;
//mod calculate;
//mod courses;
pub mod overview;
//mod share_team;
pub mod startend;
pub mod teams;

use dioxus::prelude::*;
use uuid::Uuid;

use crate::Route;

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
                    target_route: Route::StartEnd { cook_and_run_id },
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
    rsx! {
        div { class: "flex items-center justify-center h-screen w-full",
            div { class: "text-center",
                div { class: "inline-block",
                    div { class: "animate-spin rounded-full h-12 w-12 border-4 border-[#F1E7D7] border-t-[#D67229]" }
                }
                p { class: "mt-4 text-[#70513E] text-lg font-semibold", "Loading..." }
            }
        }
    }
}

#[component]
fn ErrorPage(error_text: String) -> Element {
    rsx! {
        div { class: "flex items-center justify-center h-screen w-full",
            div { class: "text-center",
                h2 { class: "text-2xl font-semibold text-red-600", "Oops!" }
                p { class: "mt-4 text-lg text-[#70513E]", "Error: {error_text}" }
            }
        }
    }
}
