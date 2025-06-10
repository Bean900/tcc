use std::sync::{Arc, Mutex};

use dioxus::prelude::*;
use uuid::Uuid;
use web_sys::console;

use crate::{
    side::AddressSVG,
    storage::{ContactData, LocalStorage, StorageR},
    Route,
};

#[component]
pub fn Calculate(id: Uuid) -> Element {
    let storage = use_context::<Arc<Mutex<LocalStorage>>>();
    let storage: std::sync::MutexGuard<'_, LocalStorage> =
        storage.lock().expect("Expected storage lock");
    let cook_and_run = storage.select_cook_and_run(id);

    if cook_and_run.is_err() {
        console::error_1(
            &format!(
                "Error while loading cook and run: {}",
                cook_and_run.expect_err("Expect error"),
            )
            .into(),
        );
        return rsx!( "Error while loading cook and run" );
    }
    let cook_and_run = cook_and_run.expect("Expect cook and run");

    rsx! {
        section {
            h2 { class: "text-3xl font-bold mb-6 text-gray-900", "Calculate" }
            button { class: "bg-blue-500 text-white font-semibold py-2 px-4 rounded hover:bg-blue-600 transition-all",
                "Calculate"
            }
            hr { class: "my-6 border-gray-300" }
            h2 { class: "text-3xl font-bold mb-6 text-gray-900", "Run Schedule" }
            div { class: "grid grid-cols-1 md:grid-cols-2 xl:grid-cols-3 gap-6 p-6 max-h-[calc(100vh-16rem)] overflow-y-auto pr-2",

                {
                    cook_and_run
                        .contact_list
                        .iter()
                        .map(|contact| {
                            let contact_id = contact.id.clone();
                            rsx! {
                                a {
                                    key: {contact_id},
                                    onclick: move |_| {
                                        let cook_and_run_id = id;
                                        use_navigator()
                                            .push(Route::RunSchedule {
                                                cook_and_run_id,
                                                contact_id,
                                            });
                                    },
                                    class: "bg-white relative shadow-lg rounded-xl p-6 hover:shadow-xl transition-all cursor-pointer hover:scale-105",
                                    div { class: "flex flex-col items-start", {ContactCard(contact.clone())} }
                                }
                            }
                        })
                }
            }
        }
    }
}
#[component]
fn ContactCard(props: ContactData) -> Element {
    rsx! {
        div {
            // Name
            h2 { class: "text-2xl font-semibold text-gray-800 mb-2", "{props.team_name}" }
            // Address
            div { class: "flex items-center space-x-2 mb-1",
                AddressSVG {}
                p { class: "text-sm text-gray-600 inline-flex items-center",
                    "{props.address.address}"
                }
            }
            // Needs Check Indicator
            if props.needs_check {
                div { class: "absolute top-2 right-2 bg-red-500 text-white text-xs font-bold rounded-full px-2 py-1",
                    "!"
                }
            }
        }
    }
}
