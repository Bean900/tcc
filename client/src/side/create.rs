use std::sync::{Arc, Mutex};

use dioxus::prelude::*;
use uuid::Uuid;

use crate::{
    storage::{LocalStorage, StorageW},
    Route,
};

#[component]
pub fn CreateProject(id: Uuid) -> Element {
    let db = use_context::<Arc<Mutex<LocalStorage>>>();

    let mut project_name = use_signal(|| String::new());

    rsx! {
        section { class: "flex justify-center items-center min-h-screen",
            div { class: "relative bg-white shadow-md rounded-xl p-6 h-12 hover:shadow-lg transition-all",

                h2 { class: "text-2xl font-bold mb-4", "Create Project" }
                input {
                    class: "border p-2 rounded w-full mb-4",
                    r#type: "text",
                    placeholder: "Project name",
                    oninput: move |e| project_name.set(e.value()),
                }
                div { class: "flex flex-wrap gap-4 mb-4",
                    button {
                        class: "bg-green-500 text-white px-4 py-2 rounded hover:bg-green-600",
                        onclick: move |_| {
                            let mut db = db.lock().unwrap();
                            let _ = db.create_cook_and_run(id, project_name.read().clone());
                            use_navigator().push(Route::ProjectDetailPage { id });
                        },
                        "Create"
                    }
                }
            }
        }
    }
}
