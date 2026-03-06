use std::vec;

use dioxus::prelude::*;
use uuid::Uuid;

use crate::{
    side::{
        details::{
            run_schedule::{run_schedule::RunSchedule, Schedule},
            ErrorPage, LoadingPage,
        },
        Headline1,
    },
    storage::{CourseData, MeetingPointData, PlanConfigData, PlanData, StorageManager, TeamData},
};

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

    let course_list_result: Resource<Result<Vec<CourseData>, String>> = use_resource(move || {
        let storage = storage.clone();
        async move {
            let storage = storage.read().clone();
            let course = storage
                .select_cook_and_run_course_list(cook_and_run_id)
                .await?;
            Ok(course)
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

    let course_list = match &*course_list_result.read_unchecked() {
        None => return rsx!(LoadingPage {}),
        Some(Err(e)) => {
            return rsx!(ErrorPage {
            error_text:
                "Could not load course list. You may need to log in or the servers may be offline."
                    .to_string(),
            error_details: e.clone(),
        })
        }
        Some(Ok(course_list)) => course_list.clone(),
    };

    let mut plan_config_signal = use_signal(|| plan_config);

    rsx!(
        div { class: "p-6",
            Headline1 { headline: "Calculation" }
            // Here we would add the actual calculation UI components, such as settings, plans, and preview.
            // For now, we can just display a placeholder.
            div { class: "mt-4 text-gray-500",
                "This is where the calculation settings, plans, and preview will be displayed."
            }
        }
    )
}

#[component]
fn CalculateSettings(cook_and_run_id: Uuid, plan_config_signal: Signal<PlanConfigData>) -> Element {
    rsx!(
        div { class: "p-6",
            // Here we would add the actual calculation UI components, such as settings, plans, and preview.
            // For now, we can just display a placeholder.
            div { class: "mt-4 text-gray-500",
                "This is where the calculation settings, plans, and preview will be displayed."
            }
        }
    )
}

#[component]
fn CalculatePlans(
    cook_and_run_id: Uuid,
    plan_config_signal: Signal<PlanConfigData>,
    course_list: Vec<CourseData>,
) -> Element {
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
            })
        }
        Some(Ok(Some(plan))) => plan.clone(),
    };

    //Liste der Teams um Laufzettel einzeln zu betrachten
    rsx!(
        div { class: "p-6",
            // Here we would add the actual calculation UI components, such as settings, plans, and preview.
            // For now, we can just display a placeholder.
            div { class: "mt-4 text-gray-500",
                "This is where the calculation settings, plans, and preview will be displayed."
            }
        }
    )
}

#[component]
fn CalculatePreview(
    cook_and_run_id: Uuid,
    plan_config: PlanConfigData,
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
        plan_config,
        schedule
    })
}
