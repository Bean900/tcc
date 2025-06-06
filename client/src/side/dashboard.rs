use std::sync::{Arc, Mutex};

use dioxus::prelude::*;
use uuid::Uuid;
use web_sys::console;

use crate::{
    side::{CloseButton, GreenButton, Input, InputError},
    storage::{LocalStorage, StorageR, StorageW},
    Route,
};

#[component]
pub fn Dashboard() -> Element {
    let storage = use_context::<Arc<Mutex<LocalStorage>>>();
    let storage = storage.lock().expect("Expected storage lock");
    let cook_and_run_list = storage.select_all_cook_and_run_minimal();

    if cook_and_run_list.is_err() {
        console::error_1(
            &format!(
                "Error loading cook and run data: {}",
                cook_and_run_list.err().expect("Expected error")
            )
            .into(),
        );
        return rsx! {
            div { "Error loading data" }
        };
    }
    let cook_and_run_list = cook_and_run_list.expect("Expected cook and run data");

    let mut create_project_signal: Signal<Element> = use_signal(|| rsx!());
    let create_dialog = rsx! {
        CreateProjectDialog { create_project_signal: create_project_signal.clone() }
    };

    rsx! {
        div { class: "grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4 gap-6 p-6",

            // Bestehende Projekte
            {
                cook_and_run_list
                    .iter()
                    .map(|cook_and_run| {
                        rsx! {
                            DashboardCard {
                                id: cook_and_run.id,
                                name: cook_and_run.name.clone(),
                                created: cook_and_run.created.format("%Y-%m-%d %H:%M").to_string(),
                                updated: cook_and_run.edited.format("%Y-%m-%d %H:%M").to_string(),
                                uploaded: false,
                            }
                        }
                    })
            }

            a {
                class: "border-4 border-dashed border-gray-300 rounded-xl p-6 h-36 flex items-center justify-center text-gray-400 hover:bg-gray-50 hover:text-blue-500 hover:scale-105 transition-all duration-200 cursor-pointer",
                onclick: move |_| {
                    create_project_signal.set(create_dialog.clone());
                },
                div {

                    div { class: "text-5xl font-bold", "+" }
                }
            }
        }
        {create_project_signal}
    }
}

#[derive(PartialEq, Props, Clone)]
struct DashboardCardProps {
    id: Uuid,
    name: String,
    created: String,
    updated: String,
    uploaded: bool,
}

#[component]
fn DashboardCard(props: DashboardCardProps) -> Element {
    rsx! {
        a {
            href: format!("/cook-and-run/{}", props.id),
            class: "relative bg-white shadow-md rounded-xl p-6 h-36 hover:shadow-lg transition-all cursor-pointer hover:scale-105",

            div { key: {props.id},

                // Wolken-Icon oben rechts
                div { class: "absolute top-3 right-3 text-gray-400",
                    if props.uploaded {
                        svg {
                            class: "w-6 h-6 text-green-500",
                            fill: "currentColor",
                            xmlns: "http://www.w3.org/2000/svg",
                            //view_box: "0 0 20 20",
                            path { d: "M16.88 9.94a5 5 0 00-9.72-1.47A4 4 0 006 17h9a4 4 0 001.88-7.06z" }
                        }
                    } else {
                        svg {
                            class: "w-6 h-6 text-gray-300",
                            fill: "currentColor",
                            xmlns: "http://www.w3.org/2000/svg",
                            // view_box: "0 0 20 20",
                            path { d: "M16.88 9.94a5 5 0 00-9.72-1.47A4 4 0 006 17h9a4 4 0 001.88-7.06z" }
                        }
                    }
                }

                // Inhalt
                h2 { class: "text-2xl font-semibold text-gray-800 mb-2", "{props.name}" }
                // Erstellt am
                div { class: "flex items-center text-sm text-gray-500 mt-2",
                    svg {
                        class: "w-4 h-4 mr-2 text-gray-400",
                        fill: "currentColor",
                        view_box: "0 0 20 20",
                        xmlns: "http://www.w3.org/2000/svg",
                        path { d: "M6 2a1 1 0 00-1 1v1H4a2 2 0 00-2 2v1h16V6a2 2 0 00-2-2h-1V3a1 1 0 10-2 0v1H7V3a1 1 0 00-1-1zM2 9v7a2 2 0 002 2h12a2 2 0 002-2V9H2z" }
                    }
                    span { "{props.created}" }
                }

                // Zuletzt bearbeitet
                div { class: "flex items-center text-sm text-gray-400",
                    svg {
                        class: "w-4 h-4 mr-2 text-gray-300",
                        fill: "currentColor",
                        view_box: "0 0 20 20",
                        xmlns: "http://www.w3.org/2000/svg",
                        path { d: "M17.414 2.586a2 2 0 010 2.828l-8.586 8.586a2 2 0 01-.879.515l-4 1a1 1 0 01-1.213-1.213l1-4a2 2 0 01.515-.879l8.586-8.586a2 2 0 012.828 0zM15 5l-1-1L6 12l-.5 2 .5.5 2-.5L15 5z" }
                    }
                    span { "{props.updated}" }
                }
            }
        }
    }
}

#[component]
fn CreateProjectDialog(create_project_signal: Signal<Element>) -> Element {
    let mut project_name_signal = use_signal(|| "".to_string());
    let mut error_signal = use_signal(|| "".to_string());

    rsx! {
        div { class: "backdrop-blur fixed inset-0 flex h-screen w-screen justify-center items-center",
            div { class: "relative bg-white shadow-md rounded-xl p-6 w-72 hover:shadow-lg transition-all cursor-pointer ",


                // Close button
                CloseButton {
                    onclick: move |_| {
                        create_project_signal.set(rsx! {});
                    },
                }

                // Title
                h2 { class: "text-2xl font-semibold text-gray-800 mb-4", "Create Project" }

                // Input field
                Input {
                    place_holer: Some("Project Name".to_string()),
                    value: project_name_signal.clone(),
                    is_error: !error_signal.read().is_empty(),
                    oninput: move |e: Event<FormData>| {
                        let value = e.value().to_string();
                        project_name_signal.set(value.clone());
                        if value.trim().is_empty() {
                            error_signal.set("Project name cannot be empty!".to_string());
                        } else {
                            error_signal.set("".to_string());
                        }
                    },
                }
                // Error message
                InputError { error: error_signal.read() }

                div { class: "flex justify-center",

                    GreenButton {
                        text: "Create".to_string(),
                        error_signal: error_signal.clone(),
                        onclick: move |_| {
                            if project_name_signal.read().trim().is_empty() {
                                error_signal.set("Project name cannot be empty!".to_string());
                                return;
                            }
                            let project_id = Uuid::new_v4();
                            let storage = use_context::<Arc<Mutex<LocalStorage>>>();
                            let mut storage = storage.lock().expect("Expected storage lock");
                            let project_name = project_name_signal.read().to_string();
                            let result = storage.create_cook_and_run(project_id, project_name);
                            if result.is_err() {
                                console::error_1(
                                    &format!(
                                        "Error creating project: {}",
                                        result.err().expect("Expected error"),
                                    )
                                        .into(),
                                );
                            }
                            create_project_signal.set(rsx! {});
                            use_navigator()
                                .push(Route::ProjectDetailPage {
                                    id: project_id,
                                });
                        },
                    }
                }
            }
        }
    }
}
