use dioxus::prelude::*;
use uuid::Uuid;
use web_sys::{console, wasm_bindgen::JsCast, HtmlInputElement};

use crate::{
    side::{CloseButton, ConfirmButton, ErrorSVG, Input, InputError, SecondaryButton},
    storage::{CookAndRunCreate, CookAndRunData, StorageManager},
};

#[component]
pub fn Dashboard() -> Element {
    let cook_and_run_list = use_resource(move || async move {
        let storage = use_context::<Signal<StorageManager>>();
        let storage = storage.read();
        match storage.select_cook_and_run_meta_list().await {
            Ok(list) => return Ok(list),
            Err(err) => {
                console::error_1(&format!("Failed to load cook and run list: {}", err).into());
                return Err("Failed to load data!");
            }
        };
    });

    let mut create_project_signal = use_signal(|| false);

    rsx! {
        div { class: "grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4 gap-6 p-6",

            // Bestehende Projekte
            {
                match &*cook_and_run_list.read_unchecked() {
                    Some(Err(err)) => rsx! {
                        ErrorSVG {}
                        div { class: "col-span-full text-red-500", {err} }
                    },
                    Some(Ok(list)) => rsx! {
                        {
                            list.iter()
                                .map(|cook_and_run| {
                                    rsx! {
                                        DashboardCard {
                                            id: cook_and_run.id,
                                            name: cook_and_run.name.clone(),
                                            created: cook_and_run.created.format("%Y-%m-%d %H:%M").to_string(),
                                            updated: cook_and_run.edited.format("%Y-%m-%d %H:%M").to_string(),
                                            uploaded: cook_and_run.is_in_cloud,
                                        }
                                    }
                                })
                        }
                    },
                    None => rsx! {
                        LoadingCard {}
                    },
                }
            }

            a {
                class: "border-4 border-dashed border-gray-300 rounded-xl p-6 h-36 flex items-center justify-center text-gray-400 hover:bg-[#fdfaf6] hover:text-[#4F7445] hover:scale-105 transition-all duration-200 cursor-pointer",
                onclick: move |_| {
                    create_project_signal.set(true);
                },
                div {

                    div { class: "text-5xl font-bold", "+" }
                }
            }
        
        }
        if *create_project_signal.read() {
            CreateProjectDialog { create_project_signal: create_project_signal.clone() }
        }
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
            href: format!("/cook-and-run/{}/overview", props.id),
            class: "relative bg-[#fdfaf6] shadow-md rounded-xl p-6 h-36 hover:shadow-lg transition-all cursor-pointer hover:scale-105",

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
fn LoadingCard() -> Element {
    rsx! {
        div { class: "relative bg-[#fdfaf6] shadow-md rounded-xl p-6 h-36 hover:shadow-lg transition-all cursor-pointer hover:scale-105",


            span { class: "sr-only", "Loading..." }
            div { role: "status", class: "h-6 max-w-sm animate-pulse",
                div { class: "h-6 bg-gray-200 rounded-full dark:bg-gray-700 w-48 mb-4" }
            }

            div { class: "flex items-center text-sm text-gray-500 mt-4",
                svg {
                    class: "w-4 h-4 mr-2 text-gray-400",
                    fill: "currentColor",
                    view_box: "0 0 20 20",
                    xmlns: "http://www.w3.org/2000/svg",
                    path { d: "M6 2a1 1 0 00-1 1v1H4a2 2 0 00-2 2v1h16V6a2 2 0 00-2-2h-1V3a1 1 0 10-2 0v1H7V3a1 1 0 00-1-1zM2 9v7a2 2 0 002 2h12a2 2 0 002-2V9H2z" }
                }
                div { role: "status", class: "h-4 max-w-sm animate-pulse",
                    div { class: "h-4 bg-gray-200 rounded-full dark:bg-gray-700 w-48 mb-4" }
                }
            }

            div { class: "flex items-center text-sm text-gray-400 mt-4",
                svg {
                    class: "w-4 h-4 mr-2 text-gray-300",
                    fill: "currentColor",
                    view_box: "0 0 20 20",
                    xmlns: "http://www.w3.org/2000/svg",
                    path { d: "M17.414 2.586a2 2 0 010 2.828l-8.586 8.586a2 2 0 01-.879.515l-4 1a1 1 0 01-1.213-1.213l1-4a2 2 0 01.515-.879l8.586-8.586a2 2 0 012.828 0zM15 5l-1-1L6 12l-.5 2 .5.5 2-.5L15 5z" }
                }
                div { role: "status", class: "h-4 max-w-sm animate-pulse",
                    div { class: "h-4 bg-gray-200 rounded-full dark:bg-gray-700 w-48 mb-4" }
                }
            }
        
        }
    }
}

#[component]
fn CreateProjectDialog(create_project_signal: Signal<bool>) -> Element {
    let mut project_name_signal = use_signal(|| "".to_string());
    let mut error_name_signal = use_signal(|| "".to_string());
    let mut error_signal = use_signal(|| "".to_string());

    rsx! {
        div { class: "backdrop-blur fixed inset-0 flex h-screen w-screen justify-center items-center",
            div { class: "relative bg-[#fdfaf6] shadow-md rounded-xl p-6 w-72 hover:shadow-lg transition-all cursor-pointer ",


                // Close button
                CloseButton {
                    onclick: move |_| {
                        create_project_signal.set(false);
                    },
                }

                // Title
                h2 { class: "text-2xl font-semibold text-gray-800 mb-4", "Create Project" }

                // Input field

                Input {
                    place_holer: Some("Project Name".to_string()),
                    value: project_name_signal.clone(),
                    is_error: !error_name_signal.read().is_empty(),
                    oninput: move |e: Event<FormData>| {
                        let value = e.value().to_string();
                        project_name_signal.set(value.clone());
                        if value.trim().is_empty() {
                            error_name_signal.set("Project name cannot be empty!".to_string());
                        } else {
                            error_name_signal.set("".to_string());
                        }
                    },
                }

                // Input file
                input {
                    id: "project_upload",
                    r#type: "file",
                    accept: ".tcc",
                    hidden: true,
                    multiple: false,
                    onchange: move |evt| {
                        async move {
                            if let Some(file_engine) = evt.files() {
                                let files = file_engine.files();
                                for file_name in &files {
                                    if let Some(file) = file_engine.read_file_to_string(file_name).await
                                    {
                                        let mut storage = use_context::<Signal<StorageManager>>();
                                        let mut storage = storage.write();
                                        let cook_and_run = match CookAndRunData::from_json(&file) {
                                            Ok(c_a_r) => c_a_r,
                                            Err(e) => {
                                                console::error_1(
                                                    &format!("Error parsing project file: {}", e).into(),
                                                );
                                                error_signal.set("Error parsing project file!".to_string());
                                                continue;
                                            }
                                        };
                                        let result = storage.create_from_file(cook_and_run).await;
                                        if let Err(e) = result {
                                            console::error_1(
                                                &format!("Error creating project from file: {}", e).into(),
                                            );
                                            error_signal
                                                .set("Error creating project from file!".to_string());
                                        } else {
                                            create_project_signal.set(false);
                                            todo!();
                                        }
                                    }
                                }
                            }
                        }
                    },
                }

                // Error message
                InputError { error: error_signal.read() }

                div { class: "flex flex-wrap gap-4 items-center mt-4 justify-center",

                    SecondaryButton {
                        text: "Upload Project".to_string(),
                        onclick: move |_| {
                            if let Some(doc) = web_sys::window().and_then(|w| w.document()) {
                                if let Some(el) = doc.get_element_by_id("project_upload") {
                                    if let Ok(input) = el.dyn_into::<HtmlInputElement>() {
                                        input.click();
                                    }
                                }
                            }
                        },
                    }


                    ConfirmButton {
                        text: "Create".to_string(),
                        error_signal: error_signal.clone(),
                        onclick: move |_| async move {
                            if project_name_signal.read().trim().is_empty() {
                                error_name_signal.set("Project name cannot be empty!".to_string());
                                return;
                            }
                            let project_id = Uuid::new_v4();
                            let mut storage = use_context::<Signal<StorageManager>>();
                            let mut storage = storage.write();
                            let project_name = project_name_signal.read().to_string();
                            let cook_and_run = CookAndRunCreate {
                                name: project_name,
                            };
                            let result = storage.create_cook_and_run(project_id, &cook_and_run).await;
                            if let Err(e) = result {
                                console::error_1(&format!("Error creating project: {}", e).into());
                                error_signal.set("Creating project failed!".to_string());
                                return;
                            }
                            create_project_signal.set(false);
                        },
                    }
                }
            }
        }
    }
}
