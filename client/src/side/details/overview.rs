use std::sync::{Arc, Mutex};

use chrono::NaiveDate;
use dioxus::prelude::*;
use uuid::Uuid;
use web_sys::wasm_bindgen::{JsCast, JsValue};
use web_sys::{console, js_sys, Blob, HtmlAnchorElement, Url};

use crate::side::{InputDate, InputMultirow};
use crate::storage::{CookAndRunData, LocalStorage, StorageR};

use crate::{
    side::{BlueButton, CloseButton, GreenButton, Input, InputError, RedButton, RedHollowButton},
    storage::StorageW,
    Route,
};

fn delete_cook_and_run_project(id: Uuid) -> Result<(), String> {
    let storage = use_context::<Arc<Mutex<LocalStorage>>>();
    let mut storage = storage.lock().expect("Expected storage lock");
    let result = storage.delete_cook_and_run(id);
    result
}

fn update_meta_of_cook_and_run(
    id: Uuid,
    new_name: String,
    new_plan_text: String,
    occur: NaiveDate,
) -> Result<(), String> {
    let new_plan_text = new_plan_text.trim();
    let plan_text = if new_plan_text.is_empty() {
        None
    } else {
        Some(new_plan_text.to_string())
    };

    let storage = use_context::<Arc<Mutex<LocalStorage>>>();
    let mut storage = storage.lock().expect("Expected storage lock");
    let result = storage.update_meta_of_cook_and_run(id, new_name, plan_text, occur);
    result
}

fn select_cook_and_run_json(id: Uuid) -> Result<String, String> {
    let storage = use_context::<Arc<Mutex<LocalStorage>>>();
    let storage = storage.lock().expect("Expected storage lock");
    storage.select_cook_and_run_json(id)
}

pub fn download_file(filename: &str, contents: &str) {
    // Create a Blob from the string content
    let array = js_sys::Array::new();
    array.push(&JsValue::from_str(contents));
    let blob = Blob::new_with_str_sequence(&array).unwrap();

    // Create a URL for the Blob
    let url = Url::create_object_url_with_blob(&blob).unwrap();

    // Create a temporary anchor element
    let document = web_sys::window().unwrap().document().unwrap();
    let a = document
        .create_element("a")
        .unwrap()
        .dyn_into::<HtmlAnchorElement>()
        .unwrap();

    a.set_href(&url);
    a.set_download(filename);

    // Append it to the body and trigger click
    document.body().unwrap().append_child(&a).unwrap();
    a.click();

    // Clean up
    document.body().unwrap().remove_child(&a).unwrap();
    Url::revoke_object_url(&url).unwrap();
}

#[component]
pub(crate) fn Overview(props: CookAndRunData) -> Element {
    let mut delete_dialog_signal: Signal<Element> = use_signal(|| rsx!());

    let mut error_message = use_signal(|| "".to_string());

    let delete_dialog = rsx! {
        DeleteProjectDialog {
            project_id: props.id,
            delete_project_signal: delete_dialog_signal.clone(),
        }
    };

    let mut name_signal = use_signal(|| props.name.clone());

    let project_id = props.id.clone();
    let on_name_input = {
        move |evt: FormEvent| {
            let current_name = evt.value();
            name_signal.set(current_name.clone());
            if current_name.is_empty() {
                error_message.set("Project name can not be empty!".to_string());
            } else {
                error_message.set("".to_string());
            }
        }
    };

    let mut plan_text_signal = use_signal(|| props.plan_text.unwrap_or("".to_string()));
    let on_plan_text_input = {
        move |evt: FormEvent| {
            let text = evt.value();
            plan_text_signal.set(text.clone());
        }
    };

    let mut occur_signal = use_signal(|| props.occur);

    let on_save = move |_| {
        let current_name = name_signal.read().clone();
        if current_name.is_empty() {
            error_message.set("Project name can not be empty!".to_string());
        } else {
            let result = update_meta_of_cook_and_run(
                project_id,
                current_name,
                plan_text_signal.read().clone(),
                occur_signal.read().clone(),
            );
            if result.is_err() {
                console::error_1(
                    &format!(
                        "Error saving project name: {}",
                        result.err().expect("Expected error"),
                    )
                    .into(),
                );
                error_message.set(
                    "Technical error while saving project name. Please try again later."
                        .to_string(),
                );
            } else {
                error_message.set("".to_string());
            }
        }
    };

    rsx! {
        section {
            h2 { class: "text-2xl font-bold mb-4", "Overview" }

            "Project Name"
            Input {
                place_holer: Some("Project Name".to_string()),
                value: name_signal.read(),
                is_error: !error_message.read().is_empty(),
                oninput: on_name_input,
            }
            InputError { error: error_message.read() }


            InputMultirow {
                place_holer: "A bit of text that appears at the end on the participants' progress sheets."
                    .to_string(),
                value: plan_text_signal.read(),
                oninput: on_plan_text_input,
            }
            InputDate {
                value: occur_signal.read().to_string(),
                oninput: move |e: FormEvent| {
                    let date = NaiveDate::parse_from_str(&e.value(), "%Y-%m-%d");
                    if date.is_err() {
                        console::error_1(
                            &format!(
                                "Date format is not correct: {}",
                                date.expect_err("Expect error"),
                            )
                                .into(),
                        );
                        return;
                    }
                    occur_signal.set(date.expect("Expect date!"));
                },
            }


            div { class: "flex flex-wrap gap-4 items-center mt-4",
                GreenButton {
                    onclick: on_save,
                    text: "Save".to_string(),
                    error_signal: error_message.clone(),
                }

                if props.is_in_cloud {
                    button { class: "bg-red-500 text-white px-4 py-2 rounded hover:bg-red-600 cursor-pointer",
                        "Remove from Cloud"
                    }
                } else {
                    button { class: "bg-blue-500 text-white px-4 py-2 rounded hover:bg-blue-600 cursor-pointer",
                        "Upload to Cloud"
                    }
                }

                BlueButton {
                    onclick: move |_| {
                        let result = select_cook_and_run_json(props.id);
                        if result.is_err() {
                            console::error_1(
                                &format!(
                                    "Error selecting project JSON: {}",
                                    result.err().expect("Expected error"),
                                )
                                    .into(),
                            );
                            return;
                        }
                        let json_content = result.expect("Expected JSON content");
                        download_file(&format!("{}.tcc", props.name), &json_content);
                    },
                    text: "Download".to_string(),
                }

                div { class: "ml-auto",
                    RedHollowButton {
                        onclick: move |_| {
                            delete_dialog_signal.set(delete_dialog.clone());
                        },
                        text: "Delete Project".to_string(),
                    }
                }
            }


            div { class: "bg-yellow-100 border border-yellow-300 text-yellow-800 p-4 rounded max-w-xl mt-6",
                h3 { class: "font-bold mb-2", "Cloud Info" }
                p { "Projects stored only locally ensure your data stays on your machine." }
                p {
                    "Uploading to the cloud enables live access to route sheets, syncing across devices, and backups."
                }
                p { class: "mt-2 font-semibold",
                    "Note: Cloud functionality requires you to be logged in."
                }
            }
        }

        {delete_dialog_signal}
    }
}

#[component]
fn DeleteProjectDialog(delete_project_signal: Signal<Element>, project_id: Uuid) -> Element {
    rsx! {
        div { class: "backdrop-blur fixed inset-0 flex h-screen w-screen justify-center items-center",
            div { class: "relative bg-white shadow-md rounded-xl p-6 hover:shadow-lg transition-all cursor-pointer ",

                // Title
                h2 { class: "text-2xl font-semibold text-red-600 mb-4", "Delete Project" }

                p { class: "text-red-600 font-semibold mb-4",
                    "Deleting this project will permanently and irreversibly remove it. This action can not be undone."
                }

                // Close button
                CloseButton {
                    onclick: move |_| {
                        delete_project_signal.set(rsx! {});
                    },
                }

                // Delete confirmation
                RedButton {
                    text: "Delete Project".to_string(),
                    onclick: move |_| {
                        let result = delete_cook_and_run_project(project_id);
                        if result.is_err() {
                            console::error_1(
                                &format!(
                                    "Error deleting project: {}",
                                    result.err().expect("Expected error"),
                                )
                                    .into(),
                            );
                        } else {
                            use_navigator().push(Route::Dashboard {});
                        }
                    },
                }
            }
        }
    }
}
