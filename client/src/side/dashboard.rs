use std::rc::Rc;

use dioxus::prelude::*;
use uuid::Uuid;

#[component]
pub fn Dashboard() -> Element {
    let projects = vec![
        (
            Uuid::new_v4(),
            "Dinner Challenge",
            "2025-04-01",
            "2025-05-10",
            true,
        ),
        (
            Uuid::new_v4(),
            "Sommerlauf 2023",
            "2023-06-01",
            "2023-06-15",
            true,
        ),
        (
            Uuid::new_v4(),
            "Winterlauf 2023",
            "2023-11-01",
            "2023-11-20",
            false,
        ),
    ];

    let mut create_project_signal: Signal<Element> = use_signal(|| rsx!());
    let create_dialog = rsx! {
        CreateProjectDialog { create_project_signal: create_project_signal.clone() }
    };

    rsx! {
        div { class: "grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4 gap-6 p-6",

            // Bestehende Projekte
            {
                projects
                    .iter()
                    .map(|(id, name, created, updated, uploaded)| {
                        rsx! {
                            DashboardCard {
                                id: id.clone(),
                                name,
                                created,
                                updated,
                                uploaded: *uploaded,
                            }
                        }
                    })
            }

            a {
                // href: format!("/cook-and-run/{}/new", Uuid::new_v4()),
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
            class: "relative bg-white shadow-md rounded-xl p-6 h-32 hover:shadow-lg transition-all cursor-pointer hover:scale-105",

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
    rsx! {
        div { class: "backdrop-blur fixed inset-0 flex h-screen w-screen justify-center",
            div { class: "bg-white rounded-xl shadow-lg p-6 h-48 relative transform -translate-y-1/2 -translate-x-1/2 top-1/2 left-1/2 absolute",

                // Close button
                button {
                    class: "absolute top-3 right-3 text-gray-400 hover:text-gray-600",
                    onclick: move |_| {
                        create_project_signal.set(rsx! {});
                    },
                    svg {
                        class: "w-6 h-6",
                        fill: "currentColor",
                        xmlns: "http://www.w3.org/2000/svg",
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
                }

                // Create button
                button {
                    class: "w-full bg-blue-500 text-white rounded-lg py-2 hover:bg-blue-600 transition-all",
                    onclick: move |_| {
                        create_project_signal.set(rsx! {});
                    },
                    "Create"
                }
            }
        }
    }
}
