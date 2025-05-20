use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct ProjectDetailPageProps {}

#[derive(PartialEq, Clone)]
enum MenuPage {
    Overview,
    Teams,
    StartEnd,
    Courses,
    Calculation,
}

#[component]
pub fn ProjectDetailPage(props: ProjectDetailPageProps) -> Element {
    let mut current_page = use_signal(|| MenuPage::Overview);
    rsx! {
        div { class: "flex h-screen",
            // Sidebar
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

            // Main Content
            div { class: "ml-64 p-6 overflow-auto",
                match current_page() {
                    MenuPage::Overview => rsx! {
                        section { class: "mb-8",
                            h2 { class: "text-2xl font-bold mb-4", "Übersicht" }
                            input {
                                class: "border p-2 rounded w-full",
                                r#type: "text",
                                placeholder: "Projektname",
                            }
                        }
                    },
                    MenuPage::Teams => rsx! {
                        section { class: "mb-8",
                            h2 { class: "text-2xl font-bold mb-4", "Teilnehmer" }
                            div { class: "flex space-x-4 mb-4",
                                button { class: "bg-blue-500 text-white px-4 py-2 rounded hover:bg-blue-600",
                                    "+ Teilnehmer hinzufügen"
                                }
                                button { class: "bg-green-500 text-white px-4 py-2 rounded hover:bg-green-600",
                                    "Aus Excel importieren"
                                }
                            }
                            div { class: "text-gray-600 italic", "(Teilnehmerliste folgt hier...)" }
                        }
                    },
                    MenuPage::StartEnd => rsx! {
                        section { class: "mb-8",
                            h2 { class: "text-2xl font-bold mb-4", "Start- & Zielpunkt" }
                            div { class: "grid grid-cols-2 gap-4",
                                input {
                                    class: "border p-2 rounded w-full",
                                    r#type: "text",
                                    placeholder: "Startadresse",
                                }
                                input { class: "border p-2 rounded w-full", r#type: "time" }
                                input {
                                    class: "border p-2 rounded w-full",
                                    r#type: "text",
                                    placeholder: "Zieladresse",
                                }
                                input { class: "border p-2 rounded w-full", r#type: "time" }
                            }
                        }
                    },
                    MenuPage::Courses => rsx! {
                        section { class: "mb-8",
                            h2 { class: "text-2xl font-bold mb-4", "Gerichte" }
                            div { class: "space-y-2",
                                div { class: "border p-4 rounded bg-gray-50", "Gang 1: Vorspeise - 18:00 Uhr" }
                                div { class: "border p-4 rounded bg-gray-50", "Gang 2: Hauptgang - 19:00 Uhr" }
                                div { class: "border p-4 rounded bg-gray-50", "Gang 3: Nachspeise - 20:00 Uhr" }
                                button { class: "mt-4 bg-blue-500 text-white px-4 py-2 rounded hover:bg-blue-600",
                                    "+ Gang hinzufügen"
                                }
                            }
                        }
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
