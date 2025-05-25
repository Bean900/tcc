use std::sync::{Arc, Mutex};

use dioxus::prelude::*;
use uuid::Uuid;
use web_sys::console;

use crate::{
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
                class: "border-4 border-dashed border-gray-300 rounded-xl p-6 h-32 flex items-center justify-center text-gray-400 hover:bg-gray-50 hover:text-blue-500 hover:scale-105 transition-all duration-200 cursor-pointer",
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
    let disabled_button = "w-full bg-gray-300 text-gray-500 rounded-lg py-2 cursor-not-allowed";
    let enabled_button =
        "w-full bg-blue-500 text-white rounded-lg py-2 hover:bg-blue-600 transition-all cursor-pointer";
    let mut project_name_signal = use_signal(|| "".to_string());

    let mut is_creating_signal = use_signal(|| false);
    rsx! {
        div { class: "backdrop-blur fixed inset-0 flex h-screen w-screen justify-center items-center",
            div { class: "relative bg-white shadow-md rounded-xl p-6 h-48 hover:shadow-lg transition-all cursor-pointer ",


                // Close button
                button {
                    class: "hover:text-gray-600 absolute top-3 right-3 cursor-pointer",
                    onclick: move |_| {
                        create_project_signal.set(rsx! {});
                    },
                    svg {
                        class: "w-6 h-6",
                        stroke: "currentColor",
                        xmlns: "http://www.w3.org/2000/svg",
                        view_box: "0 0 24 24",
                        path { d: "M6 18L18 6M6 6l12 12" }
                    }
                }

                // Title
                h2 { class: "text-2xl font-semibold text-gray-800 mb-4", "Create Project" }

                // Input field
                input {
                    class: "w-full border border-gray-300 rounded-lg p-2 mb-4 focus:outline-none focus:ring-2 focus:ring-blue-500",
                    r#type: "text",
                    placeholder: "Project Name",
                    oninput: move |e| {
                        project_name_signal.set(e.value().trim().to_string());
                    },
                }

                // Create button
                if *is_creating_signal.read() {
                    div {
                        role: "status",
                        class: "flex justify-center items-center h-12",
                        svg {
                            class: "w-8 h-8 text-gray-200 animate-spin dark:text-gray-600 fill-blue-600",
                            view_box: "0 0 100 101",
                            fill: "none",
                            xmlns: "http://www.w3.org/2000/svg",
                            path {
                                d: "M100 50.5908C100 78.2051 77.6142 100.591 50 100.591C22.3858 100.591 0 78.2051 0 50.5908C0 22.9766 22.3858 0.59082 50 0.59082C77.6142 0.59082 100 22.9766 100 50.5908ZM9.08144 50.5908C9.08144 73.1895 27.4013 91.5094 50 91.5094C72.5987 91.5094 90.9186 73.1895 90.9186 50.5908C90.9186 27.9921 72.5987 9.67226 50 9.67226C27.4013 9.67226 9.08144 27.9921 9.08144 50.5908Z",
                                fill: "currentColor",
                            }
                            path {
                                d: "M93.9676 39.0409C96.393 38.4038 97.8624 35.9116 97.0079 33.5539C95.2932 28.8227 92.871 24.3692 89.8167 20.348C85.8452 15.1192 80.8826 10.7238 75.2124 7.41289C69.5422 4.10194 63.2754 1.94025 56.7698 1.05124C51.7666 0.367541 46.6976 0.446843 41.7345 1.27873C39.2613 1.69328 37.813 4.19778 38.4501 6.62326C39.0873 9.04874 41.5694 10.4717 44.0505 10.1071C47.8511 9.54855 51.7191 9.52689 55.5402 10.0491C60.8642 10.7766 65.9928 12.5457 70.6331 15.2552C75.2735 17.9648 79.3347 21.5619 82.5849 25.841C84.9175 28.9121 86.7997 32.2913 88.1811 35.8758C89.083 38.2158 91.5421 39.6781 93.9676 39.0409Z",
                                fill: "currentFill",
                            }
                        }
                    }
                } else {
                    button {
                        class: if project_name_signal.read().is_empty() { disabled_button } else { enabled_button },
                        disabled: project_name_signal.read().is_empty(),
                        onclick: move |_| {
                            is_creating_signal.set(true);
                            spawn(async move {
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
                                is_creating_signal.set(false);
                                create_project_signal.set(rsx! {});
                                use_navigator()
                                    .push(Route::ProjectDetailPage {
                                        id: project_id,
                                    });
                            });
                        },

                        "Create"
                    }
                }
            }
        }
    }
}
