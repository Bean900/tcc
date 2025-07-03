use std::sync::{Arc, Mutex};

use dioxus::prelude::*;
use uuid::Uuid;
use web_sys::console;

use crate::{
    calculator::Calculator,
    side::{AddressSVG, Headline1, Headline2, SecondaryButton},
    storage::{ContactData, LocalStorage, PlanData, StorageR, StorageW},
    Route,
};

fn save_plan(cook_and_run_id: Uuid, plan: Option<PlanData>) -> Result<(), String> {
    let storage = use_context::<Arc<Mutex<LocalStorage>>>();
    let mut storage = storage.lock().expect("Expected storage lock");

    let result = storage.update_top_plan_in_cook_and_run(cook_and_run_id, plan);
    result
}

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
    let mut top_plan_signal = use_signal(|| cook_and_run.top_plan.clone());

    let calculator = Calculator::new(&cook_and_run);
    if calculator.is_err() {
        console::error_1(
            &format!(
                "Error while creating calculator: {}",
                calculator.expect_err("Expect error"),
            )
            .into(),
        );
        return rsx!( "Error while creating calculator. Are all fields set?" );
    }
    let calculator = calculator.expect("Expect calculator");

    rsx! {
        section {
            Headline1 { headline: "Calculation".to_string() }

            SecondaryButton {
                text: "Calculat",
                onclick: move |_| {
                    calculator.calculate();
                    calculator.stop();
                    match calculator.get_top_plan() {
                        Some(result) => {
                            if let Err(e) = save_plan(id, Some(result.clone())) {
                                console::error_1(&format!("Error saving plan: {}", e).into());
                            } else {
                                top_plan_signal.set(Some(result));
                                console::log_1(&"Plan saved successfully".into());
                            }
                        }
                        None => {
                            console::error_1(&format!("Calculation result not set!").into());
                        }
                    }
                },
            }

            div { class: "grid grid-cols-1 md:grid-cols-2 xl:grid-cols-3 gap-6 p-6 max-h-[calc(100vh-16rem)] overflow-y-auto pr-2",

                if top_plan_signal.read().is_some() {
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
}
#[component]
fn ContactCard(props: ContactData) -> Element {
    rsx! {
        div {
            // Name
            Headline2 { headline: props.team_name.clone() }
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
