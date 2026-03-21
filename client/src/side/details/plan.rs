use crate::side::details::run_schedule::run_schedule::{download, RunSchedule};
use crate::side::details::run_schedule::{ProjectSchedule, Schedule};
use crate::side::DownloadSVG;
use crate::storage::{PlanConfigData, PlanData, TeamData};
use crate::{
    side::details::{ErrorPage, LoadingPage},
    storage::StorageManager,
};
use dioxus::prelude::*;
use uuid::Uuid;
use web_sys::console;

#[component]
pub fn Plan(cook_and_run_id: Uuid, team_id: Uuid) -> Element {
    let storage = use_context::<Signal<StorageManager>>();

    let plan = use_resource(move || {
        let storage = storage.clone();
        async move {
            let storage = storage.read().clone();
            storage.select_plan_of_cook_and_run(cook_and_run_id).await
        }
    });

    let plan_config = use_resource(move || {
        let storage = storage.clone();
        async move {
            let storage = storage.read().clone();
            storage
                .select_plan_config_of_cook_and_run(cook_and_run_id)
                .await
        }
    });

    let course_list = use_resource(move || {
        let storage = storage.clone();
        async move {
            let storage = storage.read().clone();
            storage
                .select_cook_and_run_course_list(cook_and_run_id)
                .await
        }
    });

    let start_point = use_resource(move || {
        let storage = storage.clone();
        async move {
            let storage = storage.read().clone();
            storage
                .select_cook_and_run_start_point(cook_and_run_id)
                .await
        }
    });

    let end_point = use_resource(move || {
        let storage = storage.clone();
        async move {
            let storage = storage.read().clone();
            storage.select_cook_and_run_end_point(cook_and_run_id).await
        }
    });

    let plan = match &*plan.read_unchecked() {
        None => return rsx!(LoadingPage {}),
        Some(Err(e)) => {
            return rsx!(ErrorPage {
                error_text:
                    "Could not load project. You may need to log in or the servers may be offline."
                        .to_string(),
                error_details: e.clone(),
            })
        }
        Some(Ok(Some(plan))) => plan.clone(),
        Some(Ok(None)) => {
            return rsx!(ErrorPage {
                error_text: "No plan found in project.".to_string(),
                error_details: "Please come back later, when a plan is generated!",
            })
        }
    };

    let team_list_result = get_relevant_team_list(cook_and_run_id, team_id, &plan);
    let team_list_resource = if let Ok(team_list_resource) = team_list_result {
        team_list_resource
    } else {
        return rsx!(ErrorPage {
            error_text: "Internal error.".to_string(),
            error_details: "Please contact the administrator!",
        });
    };

    let mut team_list = vec![];
    for team_resource in team_list_resource {
        match &*team_resource.read_unchecked() {
            None => return rsx!(LoadingPage {}),
            Some(Err(e)) => {
                return rsx!(ErrorPage {
                error_text:
                    "Could not load project. You may need to log in or the servers may be offline."
                        .to_string(),
                error_details: e.clone(),
            })
            }
            Some(Ok(team)) => team_list.push(team.clone()),
        };
    }

    let plan_config = match &*plan_config.read_unchecked() {
        None => return rsx!(LoadingPage {}),
        Some(Err(e)) => {
            return rsx!(ErrorPage {
                error_text:
                    "Could not load project. You may need to log in or the servers may be offline."
                        .to_string(),
                error_details: e.clone(),
            })
        }
        Some(Ok(Some(plan_config))) => plan_config.clone(),
        Some(Ok(None)) => {
            return rsx!(ErrorPage {
                error_text: "No config for a plan found.".to_string(),
                error_details: "Please come back later, when a config for a plan is generated!",
            })
        }
    };

    let course_list = match &*course_list.read_unchecked() {
        None => return rsx!(LoadingPage {}),
        Some(Err(e)) => {
            return rsx!(ErrorPage {
                error_text:
                    "Could not load project. You may need to log in or the servers may be offline."
                        .to_string(),
                error_details: e.clone(),
            })
        }
        Some(Ok(course_list)) => course_list.clone(),
    };

    let start_point = match &*start_point.read_unchecked() {
        None => return rsx!(LoadingPage {}),
        Some(Err(e)) => {
            return rsx!(ErrorPage {
                error_text:
                    "Could not load project. You may need to log in or the servers may be offline."
                        .to_string(),
                error_details: e.clone(),
            })
        }
        Some(Ok(start_point)) => start_point.clone(),
    };

    let end_point = match &*end_point.read_unchecked() {
        None => return rsx!(LoadingPage {}),
        Some(Err(e)) => {
            return rsx!(ErrorPage {
                error_text:
                    "Could not load project. You may need to log in or the servers may be offline."
                        .to_string(),
                error_details: e.clone(),
            })
        }
        Some(Ok(end_point)) => end_point.clone(),
    };

    let project_schedule = ProjectSchedule::new(&team_list, &course_list, &start_point, &end_point);
    let plan_schedule = Schedule::new(team_id, &plan, &project_schedule);
    rsx!(PlanContent {
        plan_config,
        schedule: plan_schedule
    })
}

fn get_relevant_team_list(
    cook_and_run_id: Uuid,
    team_id: Uuid,
    plan: &PlanData,
) -> Result<Vec<Resource<Result<TeamData, String>>>, String> {
    let relevant_hosting_id_list = plan.walking_path.get(&team_id);
    let relevant_hosting_id_list = if let Some(relevant_hosting_id_list) = relevant_hosting_id_list
    {
        relevant_hosting_id_list
    } else {
        console::error_1(
            &format!(
                "No walking path for {} forund: {:?}",
                team_id, plan.walking_path
            )
            .into(),
        );
        return Err("Team not found in plan!".to_string());
    };

    let mut relevant_team_id_list = vec![];

    plan.hosting_list
        .iter()
        .filter(|hosting| relevant_hosting_id_list.contains(&hosting.id))
        .for_each(|hosting| {
            let host = hosting.host;
            if host == team_id {
                let guest_list = hosting.guest_list.clone();
                relevant_team_id_list.extend(guest_list);
            }

            relevant_team_id_list.push(host);
        });

    let storage = use_context::<Signal<StorageManager>>();

    Ok(relevant_team_id_list
        .iter()
        .map(|&team_id| {
            use_resource(move || {
                let storage = storage.clone();
                async move {
                    let storage = storage.read().clone();
                    storage
                        .select_cook_and_run_team(cook_and_run_id, team_id)
                        .await
                }
            })
        })
        .collect::<Vec<_>>())
}

#[component]
fn PlanContent(plan_config: PlanConfigData, schedule: Schedule) -> Element {
    let title = plan_config.title.clone();
    let team_name = schedule.host.name.clone();

    let mut hovered = use_signal(|| false);

    let base_style = "
        position: fixed; bottom: 24px; right: 24px; z-index: 1000;
        display: flex; align-items: center; justify-content: center;
        width: 56px; height: 56px; border-radius: 50%;
        border: none; cursor: pointer; background-color: #C66741;
        transition: transform 0.15s ease, box-shadow 0.15s ease;
    ";

    let hover_extra = if hovered() {
        "transform: scale(1.1); box-shadow: 0 6px 20px rgba(0,0,0,0.3);"
    } else {
        "box-shadow: 0 4px 12px rgba(0,0,0,0.2);"
    };

    rsx! {
        button {
            style: "{base_style}{hover_extra}",

            onmouseenter: move |_| hovered.set(true),
            onmouseleave: move |_| hovered.set(false),

            onclick: move |_| {
                let title = title.clone();
                let team_name =team_name.clone();
                async move {
                    download( title.clone(), team_name.clone()).await;
                }
            },

            DownloadSVG {}

        }

        RunSchedule {
            plan_config,
            schedule,
        }
    }
}
