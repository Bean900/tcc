use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use dioxus::prelude::*;
use uuid::Uuid;
use web_sys::wasm_bindgen::{JsCast, JsValue};
use web_sys::{console, js_sys, Blob, HtmlAnchorElement, Url};

use crate::side::details::{ErrorPage, LoadingPage};
use crate::side::{Headline1, InputDate};
use crate::storage::{CookAndRunMetaData, CookAndRunMetaUpdate, StorageManager};
use crate::AuthState;

use crate::{
    side::{CloseButton, ConfirmButton, Input, InputError, SecondaryButton, WarnButton},
    Route,
};

async fn delete_cook_and_run_project(id: Uuid) -> Result<(), String> {
    let mut storage = use_context::<Signal<StorageManager>>();
    let mut storage = storage.write();
    let result = storage.delete_cook_and_run(id).await;
    result
}

async fn update_meta_of_cook_and_run(
    id: Uuid,
    new_name: String,
    occur: NaiveDateTime,
) -> Result<(), String> {
    let mut storage = use_context::<Signal<StorageManager>>();
    let mut storage = storage.write();

    let result = storage
        .update_meta_of_cook_and_run(
            id,
            &CookAndRunMetaUpdate {
                name: new_name,
                occur,
            },
        )
        .await;
    result
}

async fn upload_cook_and_run(id: Uuid) -> Result<Uuid, String> {
    let mut storage: Signal<StorageManager> = use_context::<Signal<StorageManager>>();
    let mut storage = storage.write();

    storage.upload_to_cloud(id).await
}

async fn download_cook_and_run(id: Uuid) -> Result<Uuid, String> {
    let mut storage: Signal<StorageManager> = use_context::<Signal<StorageManager>>();
    let mut storage = storage.write();

    storage.download_from_cloud(id).await
}

async fn export_file(id: Uuid) -> Result<(), String> {
    let storage = use_context::<Signal<StorageManager>>();
    let storage = storage.read().clone();

    let cook_and_run = storage.select_cook_and_run(id).await?;

    let cook_and_run_json = serde_json::to_string(&cook_and_run)
        .map_err(|e| format!("Could not parse Cook and Run: {}", e.to_string()))?;
    // Create a Blob from the string content
    let array = js_sys::Array::new();
    array.push(&JsValue::from_str(&cook_and_run_json));
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
    a.set_download(&format!("{}.tcc", cook_and_run.name));

    // Append it to the body and trigger click
    document.body().unwrap().append_child(&a).unwrap();
    a.click();

    // Clean up
    document.body().unwrap().remove_child(&a).unwrap();
    Url::revoke_object_url(&url).unwrap();
    Ok(())
}

#[component]
pub fn Overview(cook_and_run_id: Uuid) -> Element {
    let storage = use_context::<Signal<StorageManager>>();
    let cook_and_run: Resource<Result<CookAndRunMetaData, String>> = use_resource(move || {
        let storage = storage.clone();
        async move {
            let storage = storage.read().clone();
            let cook_and_run = storage.select_cook_and_run_meta(cook_and_run_id).await?;
            Ok(cook_and_run)
        }
    });

    match &*cook_and_run.read_unchecked() {
        None => rsx!(
            LoadingPage {}
        ),
        Some(Err(e)) => rsx!(
            ErrorPage { error_text: e }
        ),
        Some(Ok(cook_and_run_meta)) => rsx!(
            OverviewContent { cook_and_run_meta: cook_and_run_meta.clone() }
        ),
    }
}

#[component]
pub fn OverviewContent(cook_and_run_meta: CookAndRunMetaData) -> Element {
    let auth = use_context::<Signal<AuthState>>();

    let mut delete_dialog_signal = use_signal(|| false);

    let mut error_name_signal = use_signal(|| "".to_string());

    let mut error_signal = use_signal(|| "".to_string());

    let mut name_signal = use_signal(|| cook_and_run_meta.name.clone());

    let on_name_input = {
        move |evt: FormEvent| {
            let current_name = evt.value();
            name_signal.set(current_name.clone());
            if current_name.is_empty() {
                error_name_signal.set("Project name can not be empty!".to_string());
            } else {
                error_name_signal.set("".to_string());
            }
        }
    };
    let mut occur_signal = use_signal(|| cook_and_run_meta.occur);

    let on_save = move |_| {
        let current_name = name_signal.read().clone();
        console::log_1(&format!("Start saving project: {}", current_name).into());
        async move {
            if current_name.is_empty() {
                error_name_signal.set("Project name can not be empty!".to_string());
                console::log_1(&format!("Project name can not be empty!").into());
            } else {
                console::log_1(&format!("Writing data to disk.").into());
                let result = update_meta_of_cook_and_run(
                    cook_and_run_meta.id,
                    current_name,
                    occur_signal.read().clone(),
                )
                .await;

                if let Err(e) = result {
                    console::error_1(&format!("Error saving project name: {}", e,).into());
                    error_signal.set("Saving failed! Try again later.".to_string());
                } else {
                    console::log_1(&format!("Project is saved!").into());
                    error_signal.set("".to_string());
                }
            }
        }
    };

    let error_login_signal = use_signal(|| match auth.read().clone() {
        AuthState::LoggedIn(_) => "".to_string(),
        _ => "Not loged in!".to_string(),
    });

    rsx! {
        section {
            Headline1 { headline: "Overview" }

            label { class: "block font-semibold text-[#3B3B3B]", "Project Name" }
            Input {
                place_holer: Some("Project Name".to_string()),
                value: name_signal.read(),
                is_error: !error_name_signal.read().is_empty(),
                oninput: on_name_input,
            }
            InputError { error: error_name_signal.read() }

            label { class: "block font-semibold text-[#3B3B3B]", "Occuring" }
            InputDate {
                value: occur_signal.read().format("%Y-%m-%d"),
                oninput: move |e: FormEvent| {
                    let date = NaiveDate::parse_from_str(&e.value(), "%Y-%m-%d");
                    let date = match date {
                        Ok(d) => d,
                        Err(e) => {
                            console::error_1(&format!("Date format is not correct: {}", e).into());
                            return;
                        }
                    };
                    occur_signal
                        .set(
                            date
                                .and_time(
                                    NaiveTime::from_hms_opt(0, 0, 0)
                                        .expect("Expect time to be valid!"),
                                ),
                        );
                },
            }

            InputError { error: error_signal.read() }

            div { class: "flex flex-wrap gap-4 items-center mt-4",
                ConfirmButton {
                    onclick: on_save,
                    text: "Save".to_string(),
                    error_signal: error_name_signal.clone(),
                }

                if cook_and_run_meta.is_in_cloud {
                    SecondaryButton {
                        onclick: move |_| {
                            async move {
                                let result = download_cook_and_run(cook_and_run_meta.id).await;
                                match result {
                                    Ok(cook_and_run_id) => {
                                        use_navigator().push(Route::Overview { cook_and_run_id });
                                    }
                                    Err(e) => {
                                        console::error_1(
                                            &format!("Error while downloading project: {}", e).into(),
                                        );
                                    }
                                };
                            }
                        },
                        text: "Download".to_string(),
                        error_signal: error_login_signal.clone(),
                    }
                } else {
                    SecondaryButton {
                        onclick: move |_| {
                            async move {
                                let result = upload_cook_and_run(cook_and_run_meta.id).await;
                                match result {
                                    Ok(cook_and_run_id) => {
                                        use_navigator().push(Route::Overview { cook_and_run_id });
                                    }
                                    Err(e) => {
                                        console::error_1(
                                            &format!("Error while downloading project: {}", e).into(),
                                        );
                                    }
                                };
                            }
                        },
                        text: "Upload".to_string(),
                        error_signal: error_login_signal.clone(),
                    }
                }


                SecondaryButton {
                    onclick: move |_| {
                        async move {
                            let result = export_file(cook_and_run_meta.id).await;
                            if let Err(e) = result {
                                console::error_1(&format!("Error while exporting file: {}", e).into());
                            }
                        }
                    },
                    text: "Export".to_string(),
                }

                div { class: "ml-auto",
                    WarnButton {
                        onclick: move |_| {
                            delete_dialog_signal.set(true);
                        },
                        text: "Delete Project".to_string(),
                    }
                }
            }

            if cook_and_run_meta.is_in_cloud {
                // Cloud-Projekt
                div { class: "bg-yellow-100 border border-yellow-300 text-yellow-800 p-4 rounded max-w-xl mt-6",
                    h3 { class: "font-bold mb-2", "Cloud Info" }
                    p { "This project is stored in the cloud." }
                    p { "Your data is synced across devices, and backups are handled automatically." }
                    p { class: "mt-2 font-semibold",
                        "You can't work offline. To transforme this projact to an offline project, use the download button."
                    }
                }
            } else {
                // Lokales Projekt
                div { class: "bg-yellow-100 border border-yellow-300 text-yellow-800 p-4 rounded max-w-xl mt-6",
                    h3 { class: "font-bold mb-2", "Local Project Info" }
                    p { "Projects stored only locally ensure your data stays on your machine." }
                    p {
                        "You can upload a project to the cloud to enable syncing, backups, and online collaboration."
                    }
                    p { class: "mt-2 font-semibold",
                        "Note: Cloud functionality requires you to be logged in."
                    }
                }
            }
        }

        if *delete_dialog_signal.read() {
            DeleteProjectDialog {
                delete_project_signal: delete_dialog_signal.clone(),
                project_id: cook_and_run_meta.id,
            }
        }
    }
}

#[component]
fn DeleteProjectDialog(delete_project_signal: Signal<bool>, project_id: Uuid) -> Element {
    let mut delete_loading_signal = use_signal(|| false);
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
                        delete_project_signal.set(false);
                    },
                }

                // Delete confirmation
                WarnButton {
                    text: "Delete Project".to_string(),
                    onclick: move |_| {
                        delete_loading_signal.set(true);
                        async move {
                            let result = delete_cook_and_run_project(project_id).await;
                            delete_loading_signal.set(false);
                            if let Err(e) = result {
                                console::error_1(&format!("Error deleting project: {}", e).into());
                            } else {
                                use_navigator().push(Route::Dashboard {});
                            }
                        }
                    },
                }
            }
        }
    }
}
