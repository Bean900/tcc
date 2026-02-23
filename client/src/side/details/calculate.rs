use std::vec;

use dioxus::prelude::*;
use uuid::Uuid;

use crate::{
    side::{
        details::run_schedule::{run_schedule::RunSchedule, Schedule},
        Headline1,
    },
    storage::{CourseData, MeetingPointData, PlanConfigData, PlanData, StorageManager, TeamData},
};

#[component]
pub fn Calculate(cook_and_run_id: Uuid) -> Element {
    let storage = use_context::<Signal<StorageManager>>();
    let plan_config: Resource<Result<PlanConfigData, String>> = use_resource(move || {
        let storage = storage.clone();
        async move {
            let storage = storage.read().clone();
            todo!("Plan configdata is currently not implemented in the database, so this will need to be implemented before we can fetch it.");
            let plan = storage
                .select_cook_and_run_plan_list(cook_and_run_id)
                .await?;
            Ok(plan)
        }
    });

    let course_list: Resource<Result<Vec<CourseData>, String>> = use_resource(move || {
        let storage = storage.clone();
        async move {
            let storage = storage.read().clone();
            let course = storage
                .select_cook_and_run_course_list(cook_and_run_id)
                .await?;
            Ok(course)
        }
    });

    let plan_config_signal = use_signal(|| None);

    use_effect(move || {
        if let Some(Ok(data)) = plan_config.read().as_ref() {
            plan_config_signal.set(Some(data.clone()));
        }
    });

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
fn CalculateSettings(
    cook_and_run_id: Uuid,
    plan_config_signal: Signal<Option<PlanConfigData>>,
) -> Element {
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
    plan_config_signal: Signal<Option<PlanConfigData>>,
    course_list: Resource<Result<Vec<CourseData>, String>>,
) -> Element {
    let storage = use_context::<Signal<StorageManager>>();
    let team_list: Resource<Result<Vec<TeamData>, String>> = use_resource(move || {
        let storage = storage.clone();
        async move {
            let storage = storage.read().clone();
            let team_list = storage
                .select_cook_and_run_team_list(cook_and_run_id)
                .await?;
            Ok(team_list)
        }
    });

    let plan: Resource<Result<Vec<PlanData>, String>> = use_resource(move || {
        let storage = storage.clone();
        async move {
            let storage = storage.read().clone();
            todo!("Plan data is currently not implemented in the database, so this will need to be implemented before we can fetch it.");
            let plan = storage
                .select_cook_and_run_plan_list(cook_and_run_id)
                .await?;
            Ok(plan)
        }
    });

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

    rsx!(
        RunSchedule { plan_config, schedule }
    )
}
