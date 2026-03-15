use std::vec;

use crate::{
    async_action,
    calculator::{Calculator, Course, DataMapper, Point, SchemaCalculator, Team},
    side::{
        details::{
            run_schedule::{run_schedule::RunSchedule, Schedule},
            ErrorPage, LoadingPage,
        },
        AsyncAction, ConfirmButton, Headline1, Input,
    },
    storage::{
        CourseData, Language, MeetingPointData, PlanConfigData, PlanData, StorageManager, TeamData,
    },
};
use chrono::NaiveDate;
use dioxus::prelude::*;
use reqwest::Error;
use uuid::Uuid;
use web_sys::console;

#[component]
pub fn Calculate(cook_and_run_id: Uuid) -> Element {
    let storage = use_context::<Signal<StorageManager>>();
    let plan_config_result: Resource<Result<PlanConfigData, String>> = use_resource(move || {
        let storage = storage.clone();
        async move {
            let storage = storage.read().clone();
            let result = storage
                .select_plan_config_of_cook_and_run(cook_and_run_id)
                .await;
            match result {
                Ok(Some(config)) => Ok(config),
                Ok(None) => Ok(PlanConfigData::default()),
                Err(e) => Err(e),
            }
        }
    });

    let course_list_result: Resource<
        Result<
            (
                Vec<CourseData>,
                Option<MeetingPointData>,
                Option<MeetingPointData>,
            ),
            String,
        >,
    > = use_resource(move || {
        let storage = storage.clone();
        async move {
            let storage = storage.read().clone();
            let course = storage
                .select_cook_and_run_course_list(cook_and_run_id)
                .await?;
            let start_point = storage
                .select_cook_and_run_start_point(cook_and_run_id)
                .await?;
            let end_point = storage
                .select_cook_and_run_end_point(cook_and_run_id)
                .await?;
            Ok((course, start_point, end_point))
        }
    });

    let plan_config= match &*plan_config_result.read_unchecked() {
        None => return rsx!(LoadingPage {}),
        Some(Err(e)) => return rsx!(ErrorPage {
            error_text:
                "Could not load plan configuration. You may need to log in or the servers may be offline."
                    .to_string(),
            error_details: e.clone(),
        }),
        Some(Ok(plan_config)) =>  plan_config.clone(),
    };

    let (course_list, start_point, end_point) = match &*course_list_result.read_unchecked() {
        None => return rsx!(LoadingPage {}),
        Some(Err(e)) => {
            return rsx!(ErrorPage {
            error_text:
                "Could not load course list. You may need to log in or the servers may be offline."
                    .to_string(),
            error_details: e.clone(),
        })
        }
        Some(Ok((course_list, start_point, end_point))) => {
            (course_list.clone(), start_point.clone(), end_point.clone())
        }
    };

    let plan_config_signal = use_signal(|| plan_config);

    let calculate_settings = rsx!(CalculateSettings {
        cook_and_run_id,
        plan_config_signal: plan_config_signal.clone(),
    });

    let calculate_plans = rsx!(CalculatePlans {
        cook_and_run_id,
        course_list: course_list.clone(),
        start_point: start_point.clone(),
        end_point: end_point.clone()
    });

    let calculate_preview = rsx!(CalculatePreview {
        cook_and_run_id,
        plan_config: plan_config_signal.clone(),
        course_list,
        start_point,
        end_point,
    });

    rsx!(
        div { class: "p-6",
            Headline1 { headline: "Calculation" }
            // Here we would add the actual calculation UI components, such as settings, plans, and preview.
            // For now, we can just display a placeholder.
            div { class: "mt-4 text-gray-500",
                "This is where the calculation settings, plans, and preview will be displayed."
            }
            div { class: "grid grid-cols-2 gap-6",
                {calculate_settings}
                {calculate_preview}
            }
            {calculate_plans}

        }
    )
}

#[component]
fn CalculateSettings(cook_and_run_id: Uuid, plan_config_signal: Signal<PlanConfigData>) -> Element {
    rsx!(
        div { class: "p-6 space-y-4",
            div { class: "bg-white rounded-lg shadow p-4",
            div { class: "text-lg font-semibold mb-4", "Plan Configuration" }

            div { class: "space-y-4",
                div { class: "flex flex-col",
                label { class: "text-sm font-medium text-gray-700 mb-1", "Title" }
                Input {
                                    place_holer: "Course name",
                                    value: "{plan_config_signal.read().title.clone()}",
                                    oninput: move |e: Event<FormData>| {
                                        plan_config_signal.write().title = e.value() .to_string();
                                    },
                                }
                }

                div { class: "flex flex-col",
                label { class: "text-sm font-medium text-gray-700 mb-1", "Description" }
               Input {
                                    place_holer: "Course name",
                                    value: "{plan_config_signal.read().description.clone()}",
                                    oninput: move |e: Event<FormData>| {
                                        plan_config_signal.write().description = e.value() .to_string();
                                    },
                                }
                }

                div { class: "flex flex-col",
                label { class: "text-sm font-medium text-gray-700 mb-1", "Date" }
                input {
                    type: "date",
                    class: "px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500",
                    value: "{plan_config_signal.read().date}",
                    onchange: move |e: Event<FormData>| {
                        if let Ok(date) = NaiveDate::parse_from_str(&e.value(), "%Y-%m-%d") {
                            plan_config_signal.write().date = date;
                        }
                    },
                }
                }

                div { class: "flex flex-col",
                label { class: "text-sm font-medium text-gray-700 mb-1", "Language" }
                    select { onchange: move |e| {
                       let selected_language = e.value();
                        // Update the plan_config_signal with the selected language
                        plan_config_signal.write().language = match selected_language.as_str() {
                            "eng" => Language::English,
                            "deu" => Language::German,
                            _ => Language::English, // Default to English if something goes wrong
                        };
                    },
                        option { value: "eng", "English" }
                        option { value: "deu", "German" }
                    }
                }
            }

            div { class: "flex gap-3 mt-6",
               ConfirmButton {
                        action: async_action!(
                            {
                                let mut storage = use_context::<Signal<StorageManager>>().write().clone();
                                let result = storage
                                    .update_plan_config_of_cook_and_run(cook_and_run_id, &plan_config_signal.read())
                                    .await;
                                if let Err(e) = &result {
                                     console::error_1(&format!("Error updating plan config: {}", e) .into());
                                }else {
                                     console::log_1(&"Plan configuration updated successfully".into());
                                }
                             }
                        ),
                        text: "Save Settings".to_string(),
                    }

            }
            }
        }
    )
}

#[component]
fn CalculatePlans(
    cook_and_run_id: Uuid,
    course_list: Vec<CourseData>,
    start_point: Option<MeetingPointData>,
    end_point: Option<MeetingPointData>,
) -> Element {
    // Check start and end point times vs course times
    let all_course_times = course_list.iter().map(|c| c.time).collect::<Vec<_>>();

    if !all_course_times.is_empty() {
        if let Some(start) = start_point.as_ref().map(|mp| mp.time) {
            if start > *all_course_times.iter().min().unwrap() {
                return rsx!(ErrorPage {
                    error_text: "The start point time must be before all course times.".to_string(),
                    error_details:
                        "Please adjust the start point time to be before the earliest course time."
                            .to_string(),
                });
            }
        }
        if let Some(end) = end_point.as_ref().map(|mp| mp.time) {
            if end < *all_course_times.iter().max().unwrap() {
                return rsx!(ErrorPage {
                    error_text: "The end point time must be after all course times.".to_string(),
                    error_details:
                        "Please adjust the end point time to be after the latest course time."
                            .to_string(),
                });
            }
        }
    }

    let storage = use_context::<Signal<StorageManager>>();
    let team_list_result: Resource<Result<Vec<TeamData>, String>> = use_resource(move || {
        let storage = storage.clone();
        async move {
            let storage = storage.read().clone();
            let team_list = storage
                .select_cook_and_run_team_list(cook_and_run_id)
                .await?;
            Ok(team_list)
        }
    });

    let plan_result: Resource<Result<Option<PlanData>, String>> = use_resource(move || {
        let storage = storage.clone();
        async move {
            let storage = storage.read().clone();
            storage.select_plan_of_cook_and_run(cook_and_run_id).await
        }
    });

    let team_list = match &*team_list_result.read_unchecked() {
        None => return rsx!(LoadingPage {}),
        Some(Err(e)) => {
            return rsx!(ErrorPage {
            error_text:
                "Could not load team list. You may need to log in or the servers may be offline."
                    .to_string(),
            error_details: e.clone(),
        })
        }
        Some(Ok(team_list)) => team_list.clone(),
    };

    let end_point_signal = use_signal(|| end_point.map(|mp| Point::from_data(&mp.address)));
    let team_list_signal: Signal<Vec<Team>> =
        use_signal(|| team_list.iter().map(|team| Team::from_data(team)).collect());
    let course_list_signal: Signal<Vec<Course>> = use_signal(|| {
        course_list
            .iter()
            .map(|course| Course::from_data(course))
            .collect()
    });

    if end_point_signal.read().is_none() {
        return rsx!(div { class: "flex items-center gap-2 text-gray-500",
         "No plan found for this Cook and Run project. Please run the calculation to generate a plan."
            "No end point found for this project. Please set an end point to generate a plan."
        });
    };

    let plan = match &*plan_result.read_unchecked() {
        None => return rsx!(LoadingPage {}),
        Some(Err(e)) => {
            return rsx!(ErrorPage {
            error_text:
                "Could not load team list. You may need to log in or the servers may be offline."
                    .to_string(),
            error_details: e.clone(),
        })
        }
        Some(Ok(None)) => {
            return rsx!(div { class: "flex items-center gap-2 text-gray-500",
                "No plan found for this Cook and Run project. Please run the calculation to generate a plan."

                      ConfirmButton {
                        action: async_action!(
                            {
                                let end_point = end_point_signal.as_ref() .expect("End point is not available");

                                let team_list = team_list_signal.read() ;
                                let course_list = course_list_signal.read() ;
                                let calculator_result = SchemaCalculator::new(&end_point, &team_list, &course_list);

                                let calculator = match calculator_result {
                                    Ok(calculator) => calculator,
                                    Err(e) => {
                                        console::error_1(&format!("Error creating calculator: {}", e).into());
                                        return;
                                    }
                                };

                                let plan = calculator.calculate();
                                let plan_data = plan.to_data();
                                let mut storage = use_context::<Signal<StorageManager>>().write().clone();
                                let result = storage
                                    .update_plan_of_cook_and_run(cook_and_run_id, &plan_data)
                                    .await;
                                if let Err(e) = &result {
                                    console::error_1(&format!("Error updating plan config: {}", e).into());
                                } else {
                                    console::log_1(&"Plan updated successfully".into());
                                }

                             }
                        ),
                        text: "Calculate".to_string(),
                    }
            });
        }
        Some(Ok(Some(plan))) => plan.clone(),
    };

    rsx!({
        plan.hosting_list.iter().map(|hosting| {
            let team = team_list.iter().find(|team| team.id == hosting.host);
            if let Some(team) = team {
                rsx!(div {
                    class: "p-4 border rounded",
                    div { class: "font-bold mb-2", "team.name.clone()" }
                    // Here we would display the actual plan details for the team. For now, we can just display a placeholder.
                    div { class: "mt-2 text-gray-500",
                        "This is where the plan details for the team will be displayed."
                    }
                })
            } else {
                rsx!(div {
                    class: "p-4 border rounded",
                    div { class: "font-bold mb-2", "Team not found" }
                    // Here we would display the actual plan details for the team. For now, we can just display a placeholder.
                    div { class: "mt-2 text-gray-500",
                        "This is where the plan details for the team will be displayed."
                    }
                })
            }
        })
    })
}

#[component]
fn CalculatePreview(
    cook_and_run_id: Uuid,
    plan_config: Signal<PlanConfigData>,
    course_list: Vec<CourseData>,
    start_point: Option<MeetingPointData>,
    end_point: Option<MeetingPointData>,
) -> Element {
    if course_list.is_empty() {
        return rsx! {
            div { class: "flex items-center gap-2 text-gray-500",
                "At least one course is required to display the preview."
            }
        };
    }

    let schedule = Schedule::default(false, true, 3, 2, 2, true, true, true, true);

    rsx!(RunSchedule {
        plan_config: plan_config.read().clone(),
        schedule
    })
}
