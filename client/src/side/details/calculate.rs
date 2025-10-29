use dioxus::prelude::*;
use uuid::Uuid;
use web_sys::console;

use crate::{
    calculator::Calculator,
    error,
    side::{AddressSVG, Headline1, Headline2, SecondaryButton},
    storage::{TeamData, PlanData, StorageManager},
    Route,
};

fn save_plan(cook_and_run_id: Uuid, plan: &PlanData) -> Result<(), String> {
    let mut storage = StorageManager::get_lock()?;
    let result = storage.update_plan_of_cook_and_run(cook_and_run_id, &plan);
    result
}

#[component]
pub fn Calculate(id: Uuid) -> Element {
    let storage = match StorageManager::get_lock() {
        Ok(storage) => storage,
        Err(e) => {
            console::error_1(&format!("Error while getting storage lock: {}", e).into());
            return error(
                "Unexpected Error",
                "An unexpected error has occurred. Please team your system administrator.",
            );
        }
    };

    let cook_and_run = match storage.select_cook_and_run(id) {
        Ok(cook_and_run) => cook_and_run,
        Err(e) => {
            console::error_1(&format!("Error while loading cook and run: {}", e,).into());
            return error(
                "Not Found",
                "The requested cook and run could not be found.",
            );
        }
    };

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
        return rsx!("Error while creating calculator. Are all fields set?");
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
                            if let Err(e) = save_plan(id, &result) {
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
                            .team_list
                            .iter()
                            .map(|team| {
                                let team_id = team.id.clone();
                                rsx! {
                                    a {
                                        key: {team_id},
                                        onclick: move |_| {
                                            let cook_and_run_id = id;
                                            use_navigator()
                                                .push(Route::RunSchedule {
                                                    cook_and_run_id,
                                                    team_id,
                                                });
                                        },
                                        class: "bg-white relative shadow-lg rounded-xl p-6 hover:shadow-xl transition-all cursor-pointer hover:scale-105",
                                        div { class: "flex flex-col items-start", {TeamCard(team.clone())} }
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
fn TeamCard(props: TeamData) -> Element {
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
