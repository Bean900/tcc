use async_std::task::sleep;
use std::time::Duration;
use std::vec;

use crate::{
    async_action,
    calculator::{Calculator, Course, DataMapper, Point, SchemaCalculator, Team},
    side::{
        details::{
            run_schedule::{run_schedule::RunSchedule, Schedule},
            ErrorPage, LoadingPage,
        },
        AsyncAction, ConfirmButton, Headline1, Headline2, Input, SecondaryButton,
    },
    storage::{
        CourseData, Language, MeetingPointData, PlanConfigData, PlanData, StorageManager, TeamData,
    },
    Route,
};
use chrono::NaiveDate;
use dioxus::prelude::*;
use uuid::Uuid;
use web_sys::console;

struct TabelContent {
    host: TeamData,
    guest_list: Vec<(TeamData, bool)>,
}

impl TabelContent {
    fn new_list(team_list: &Vec<TeamData>, plan: &PlanData) -> Vec<Self> {
        let team_map = team_list
            .iter()
            .map(|team| (team.id, team.clone()))
            .collect::<std::collections::HashMap<_, _>>();

        let hosting_map = plan
            .hosting_list
            .iter()
            .map(|hosting| (hosting.id, hosting.host))
            .collect::<std::collections::HashMap<_, _>>();

        plan.walking_path
            .iter()
            .map(|(team_id, host_list_id)| {
                let team_opt = team_map.get(team_id);
                let host = if let Some(team) = team_opt {
                    team.clone()
                } else {
                    console::error_1(&format!("Team with id {} not found!", team_id).into());
                    TeamData::default()
                };
                let guest_list = host_list_id
                    .iter()
                    .map(|host_id| {
                        let guest_id_opt = hosting_map.get(host_id);
                        let guest_id = if let Some(guest_id) = guest_id_opt {
                            guest_id
                        } else {
                            console::error_1(
                                &format!("Hosting with id {} not found!", host_id).into(),
                            );
                            return (TeamData::default(), false);
                        };

                        let team_opt = team_map.get(guest_id);
                        if let Some(team) = team_opt {
                            (team.clone(), team_id == guest_id)
                        } else {
                            console::error_1(
                                &format!("Team with id {} not found!", team_id).into(),
                            );
                            (TeamData::default(), false)
                        }
                    })
                    .collect::<Vec<(TeamData, bool)>>();
                TabelContent { host, guest_list }
            })
            .collect::<Vec<_>>()
    }
}

// ─────────────────────────────────────────────
//  Root
// ─────────────────────────────────────────────

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

    let plan_config = match &*plan_config_result.read_unchecked() {
        None => return rsx!(LoadingPage {}),
        Some(Err(e)) => return rsx!(ErrorPage {
            error_text: "Could not load plan configuration. You may need to log in or the servers may be offline."
                .to_string(),
            error_details: e.clone(),
        }),
        Some(Ok(plan_config)) => plan_config.clone(),
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
        div { class: "px-8 py-6 space-y-8",

            // ── Page header ──────────────────────────────────────
            Headline1 { headline: "Calculation" }

            // ── Settings + Preview ────────────────────────────────
            div { class: "grid grid-cols-2 gap-6 items-start",
                {calculate_settings}
                {calculate_preview}
            }

            // ── Walking-path table ────────────────────────────────
            {calculate_plans}
        }
    )
}

// ─────────────────────────────────────────────
//  Settings card
// ─────────────────────────────────────────────

// ─────────────────────────────────────────────
//  CSS: Keyframe-Animation für den grünen Glow
// ─────────────────────────────────────────────

const SAVE_GLOW_CSS: &str = r#"
@keyframes save-glow {
    0%   {
        border-color: #d1fae5;
        box-shadow: 0 0 0 0px rgba(34, 197, 94, 0),   0 1px 3px 0 rgba(0,0,0,0.06);
    }
    20%  {
        border-color: #22c55e;
        box-shadow: 0 0 0 5px rgba(34, 197, 94, 0.22), 0 1px 3px 0 rgba(0,0,0,0.06);
    }
    55%  {
        border-color: #16a34a;
        box-shadow: 0 0 0 5px rgba(34, 197, 94, 0.10), 0 1px 3px 0 rgba(0,0,0,0.06);
    }
    100% {
        border-color: #bbf7d0;
        box-shadow: 0 0 0 0px rgba(34, 197, 94, 0),   0 1px 3px 0 rgba(0,0,0,0.06);
    }
}
.save-glow-card { animation: save-glow 2s ease-in-out forwards; }
.save-glow-card .save-glow-header {
    background-color: rgba(240,253,244,0.70) !important;
    border-bottom-color: #bbf7d0 !important;
    transition: background-color 0.4s ease, border-color 0.4s ease;
}
.save-glow-card .save-glow-accent {
    background-color: rgba(34,197,94,0.75) !important;
    transition: background-color 0.4s ease;
}
.save-glow-card .save-glow-title {
    color: #166534 !important;
    transition: color 0.4s ease;
}
"#;

#[component]
fn CalculateSettings(cook_and_run_id: Uuid, plan_config_signal: Signal<PlanConfigData>) -> Element {
    const LBL: &str =
        "block text-[11px] font-semibold tracking-[0.12em] uppercase text-amber-700/70 mb-1.5";
    const NATIVE_INPUT: &str =
        "w-full px-3 py-2 rounded-xl border border-amber-200 bg-amber-50/40 \
         text-sm text-zinc-800 \
         focus:outline-none focus:ring-2 focus:ring-amber-400/40 focus:border-amber-400 \
         transition-colors duration-150 mb-4";

    // true  → grüner Glow aktiv für 2 s
    let mut save_success_signal = use_signal(|| false);
    // true  → ungespeicherte Änderungen vorhanden → Button aktiv
    let mut has_unsaved_changes = use_signal(|| false);

    let is_success = *save_success_signal.read();
    let can_save = *has_unsaved_changes.read();

    let card_class = if is_success {
        "bg-white rounded-2xl border overflow-hidden save-glow-card"
    } else {
        "bg-white rounded-2xl border border-amber-100 shadow-sm overflow-hidden"
    };
    let header_class = if is_success {
        "px-5 py-3.5 border-b flex items-center gap-2.5 save-glow-header"
    } else {
        "px-5 py-3.5 bg-amber-50/70 border-b border-amber-100 flex items-center gap-2.5"
    };
    let accent_class = if is_success {
        "w-1.5 h-5 rounded-full save-glow-accent"
    } else {
        "w-1.5 h-5 rounded-full bg-amber-400/70"
    };
    let title_class = if is_success {
        "text-sm font-semibold save-glow-title"
    } else {
        "text-sm font-semibold text-zinc-800"
    };
    let save_btn_wrapper_class = if can_save {
        ""
    } else {
        "opacity-40 pointer-events-none cursor-not-allowed"
    };

    rsx!(
        style { dangerous_inner_html: SAVE_GLOW_CSS }

        div { class: "{card_class}",

            // Card header
            div { class: "{header_class}",
                div { class: "{accent_class}" }
                span { class: "{title_class}", "Plan Configuration" }
            }

            // Form body
            div { class: "px-5 py-5 space-y-4",

                // Title
                div {
                    label { class: "{LBL}", "Title" }
                    Input {
                        place_holer: "Plan title",
                        value: "{plan_config_signal.read().title.clone()}",
                        oninput: move |e: Event<FormData>| {
                            has_unsaved_changes.set(true);
                            plan_config_signal.write().title = e.value().to_string();
                        },
                    }
                }

                // Description
                div {
                    label { class: "{LBL}", "Description" }
                    Input {
                        place_holer: "Short description",
                        value: "{plan_config_signal.read().description.clone()}",
                        oninput: move |e: Event<FormData>| {
                            has_unsaved_changes.set(true);
                            plan_config_signal.write().description = e.value().to_string();
                        },
                    }
                }

                // Date + Language side by side
                div { class: "grid grid-cols-2 gap-3",

                    div {
                        label { class: "{LBL}", "Date" }
                        input {
                            r#type: "date",
                            class: "{NATIVE_INPUT}",
                            value: "{plan_config_signal.read().date}",
                            onchange: move |e: Event<FormData>| {
                                if let Ok(date) = NaiveDate::parse_from_str(&e.value(), "%Y-%m-%d") {
                                    has_unsaved_changes.set(true);
                                    plan_config_signal.write().date = date;
                                }
                            },
                        }
                    }

                    div {
                        label { class: "{LBL}", "Language" }
                        select {
                            class: "{NATIVE_INPUT} cursor-pointer",
                            onchange: move |e| {
                                has_unsaved_changes.set(true);
                                plan_config_signal.write().language = Language::from_string(e.value());
                            },
                            value: "{plan_config_signal.read().language.to_string()}",
                            option { value: "eng", "English" }
                            option { value: "deu", "German" }
                        }
                    }
                }
            }

            // Card footer
            div { class: "px-5 pb-5 pt-1",
                div { class: "{save_btn_wrapper_class}",
                    ConfirmButton {
                        action: async_action!(
                            {
                                let mut storage = use_context::<Signal<StorageManager>>().write().clone();
                                let result = storage
                                    .update_plan_config_of_cook_and_run(cook_and_run_id, &plan_config_signal.read())
                                    .await;
                                if let Err(e) = &result {
                                    console::error_1(&format!("Error updating plan config: {}", e).into());
                                } else {
                                    console::log_1(&"Plan configuration updated successfully".into());
                                    has_unsaved_changes.set(false);
                                    save_success_signal.set(true);
                                    spawn(async move {
                                        sleep(Duration::from_millis(2000)).await;
                                        save_success_signal.set(false);
                                    });
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

// ─────────────────────────────────────────────
//  Walking-path table
// ─────────────────────────────────────────────

#[component]
fn CalculatePlans(
    cook_and_run_id: Uuid,
    course_list: Vec<CourseData>,
    start_point: Option<MeetingPointData>,
    end_point: Option<MeetingPointData>,
) -> Element {
    // Validate time ordering
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

    let mut plan_result: Resource<Result<Option<PlanData>, String>> = use_resource(move || {
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

    // ── Empty state: no end point ──────────────────────────────────
    if end_point_signal.read().is_none() {
        return rsx!(
            div { class: "rounded-2xl border border-amber-100 bg-amber-50/40 px-6 py-8 text-center",
                div { class: "w-10 h-10 rounded-full bg-amber-100 flex items-center justify-center mx-auto mb-3",
                    span { class: "text-amber-500 text-lg", "⚑" }
                }
                p { class: "text-sm font-medium text-zinc-700 mb-1", "No end point configured" }
                p { class: "text-xs text-zinc-400",
                    "Please set an end point for this project before generating a plan."
                }
            }
        );
    }

    // ── Empty state: no plan yet ───────────────────────────────────

    let plan = match &*plan_result.read_unchecked() {
        None => return rsx!(LoadingPage {}),
        Some(Err(e)) => {
            return rsx!(ErrorPage {
                error_text:
                    "Could not load plan. You may need to log in or the servers may be offline."
                        .to_string(),
                error_details: e.clone(),
            })
        }
        Some(Ok(None)) => {
            return rsx!(
                div { class: "rounded-2xl border border-amber-100 bg-amber-50/40 px-6 py-8 flex flex-col items-center gap-4",
                    div { class: "w-10 h-10 rounded-full bg-amber-100 flex items-center justify-center",
                        span { class: "text-amber-500 text-lg", "⊕" }
                    }
                    div { class: "text-center",
                        p { class: "text-sm font-medium text-zinc-700 mb-1", "No plan generated yet" }
                        p { class: "text-xs text-zinc-400",
                            "Run the calculation to create a walking-path plan for this Cook & Run."
                        }
                    }
                    ConfirmButton {
                        action: async_action!(
                            {
                                let end_point = end_point_signal.as_ref().expect("End point is not available");
                                let team_list = team_list_signal.read();
                                let course_list = course_list_signal.read();
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
                                     plan_result.restart();
                                }
                            }
                        ),
                        text: "Calculate".to_string(),
                    }
                }
            );
        }
        Some(Ok(Some(plan))) => plan.clone(),
    };

    let table_content = TabelContent::new_list(&team_list, &plan);

    let headline_list = course_list
        .iter()
        .map(|course| course.name.clone())
        .collect::<Vec<String>>();

    let num_courses = headline_list.len();
    let num_teams = table_content.len();

    let grid_cols = format!(
        "grid-template-columns: minmax(200px, 260px) repeat({num_courses}, minmax(160px, 1fr));"
    );

    rsx! {
        div {

            // ── Section header ─────────────────────────────────────
            div { class: "mb-6 flex items-end justify-between",
                div {
                    Headline2 { headline: "Walking path" }
                    p { class: "text-zinc-500 text-sm mt-0.5",
                        "{num_teams} Teams  ·  {num_courses} Courses"
                    }
                }

                // Re-calculate button
                ConfirmButton {
                    action: async_action!(
                        {
                            let end_point = end_point_signal.as_ref().expect("End point is not available");
                            let team_list = team_list_signal.read();
                            let course_list = course_list_signal.read();
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
                    text: "Recalculate".to_string(),
                }
            }

            // ── Table ──────────────────────────────────────────────
            //
            // Color rationale:
            //   • Outer border / dividers : amber-200  — warm, matches card system
            //   • Header background       : #FAF0E2    — slightly deeper than even rows
            //   • Even rows               : #F8EFE1    — warm cream (original)
            //   • Odd rows                : #FDFAF6    — near-white warm, replaces cold pinkish #F8EFEF
            //   • Row hover               : amber-100/70 — warm, never goes dark
            //   • Host badge              : #D67229    — amber accent (unchanged)
            //   • Guest pill (host)       : #C66741 border + text — terracotta accent (unchanged)
            //   • Guest pill (visitor)    : amber-200 border, zinc-700 text — warm neutral
            //   • Address text            : amber-900/40 — warm muted instead of cold zinc-500
            //   • Fill dash               : amber-300  — warm placeholder

            div { class: "overflow-x-auto rounded-2xl border border-amber-200 shadow-sm",

                div { class: "min-w-max w-full",

                    // Header row
                    div {
                        class: "grid border-b border-amber-200 bg-[#FAF0E2]",
                        style: "{grid_cols}",

                        div { class: "px-5 py-3.5 flex items-center gap-2.5",
                            div { class: "w-1.5 h-4 rounded-full bg-amber-400/70" }
                            span {
                                class: "text-[11px] font-bold tracking-[0.18em] uppercase text-amber-700/60",
                                "Team"
                            }
                        }

                        for (idx, course_name) in headline_list.iter().enumerate() {
                            {
                                rsx! {
                                    div {
                                        key: "{idx}",
                                        class: "px-5 py-3.5 border-l border-amber-200 flex items-center gap-2.5",
                                        span {
                                            class: "text-[11px] font-bold tracking-[0.18em] uppercase text-amber-700/60",
                                            "{course_name}"
                                        }
                                    }
                                }
                            }
                        }
                    }

                    // Data rows
                    for (row_idx, row) in table_content.iter().enumerate() {
                        {
                            // Warm zebra: cream / near-white — no more cold pink
                            let row_bg = if row_idx % 2 == 0 {
                                "bg-[#F8EFE1]"
                            } else {
                                "bg-[#FDFAF6]"
                            };
                            let host_id = row.host.id;
                            rsx! {
                                div {
                                    key: "{row_idx}",
                                    class: "grid border-b border-amber-200/70 last:border-b-0 \
                                            transition-colors duration-100 hover:bg-amber-100/70 \
                                            {row_bg} cursor-pointer",
                                    style: "{grid_cols}",
                                    onclick: move |_| {
                                        use_navigator().push(Route::Plan { cook_and_run_id, team_id: host_id });
                                    },

                                    // Host cell
                                    div {
                                        class: "px-5 py-4 border-r border-amber-200/70 flex items-start gap-3",

                                        div {
                                            class: "mt-1.5 w-6 h-6 shrink-0 rounded-md bg-[#D67229] \
                                                    flex items-center justify-center",
                                            span {
                                                class: "text-white text-[10px] font-bold",
                                                "{row_idx + 1}"
                                            }
                                        }

                                        div { class: "mt-0.5 flex flex-col min-w-0",
                                            span {
                                                class: "font-semibold text-sm leading-snug truncate text-zinc-800",
                                                "{row.host.name}"
                                            }
                                        }
                                    }

                                    // Guest cells
                                    for (idx, (guest, is_host)) in row.guest_list.iter().enumerate() {
                                        {
                                            // Host of this course: terracotta accent
                                            // Visitor: warm amber border, neutral text
                                            let text_color = if *is_host {
                                                "text-[#C66741]"
                                            } else {
                                                "text-zinc-700"
                                            };
                                            let border_color = if *is_host {
                                                "border-[#C66741]"
                                            } else {
                                                "border-amber-200"
                                            };

                                            rsx! {
                                                div {
                                                    key: "guest-{idx}",
                                                    class: "px-4 py-4 border-l border-amber-200/70 \
                                                            flex flex-col justify-center gap-1",

                                                    div {
                                                        class: "inline-flex items-center gap-1.5 px-2.5 py-1 \
                                                                rounded-lg border {border_color} \
                                                                w-fit max-w-full",
                                                        span {
                                                            class: "text-xs font-semibold truncate {text_color}",
                                                            "{guest.name}"
                                                        }
                                                    }

                                                    span {
                                                        class: "text-amber-900/40 text-[11px] leading-tight pl-1 truncate",
                                                        "{guest.address.address}"
                                                    }
                                                }
                                            }
                                        }
                                    }

                                    // Fill empty columns
                                    for fill_idx in row.guest_list.len()..num_courses {
                                        div {
                                            key: "fill-{fill_idx}",
                                            class: "px-4 py-4 border-l border-amber-200/70 flex items-center",
                                            span { class: "text-amber-300 text-base select-none", "—" }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

// ─────────────────────────────────────────────
//  Schedule preview card
// ─────────────────────────────────────────────

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
            div { class: "bg-white rounded-2xl border border-amber-100 shadow-sm overflow-hidden",
                div { class: "px-5 py-3.5 bg-amber-50/70 border-b border-amber-100 flex items-center gap-2.5",
                    div { class: "w-1.5 h-5 rounded-full bg-amber-400/70" }
                    span { class: "text-sm font-semibold text-zinc-800", "Schedule Preview" }
                }
                div { class: "px-6 py-10 flex flex-col items-center gap-3 text-center",
                    div { class: "w-10 h-10 rounded-full bg-amber-100 flex items-center justify-center",
                        span { class: "text-amber-500 text-lg", "◷" }
                    }
                    p { class: "text-sm font-medium text-zinc-700", "No courses configured yet" }
                    p { class: "text-xs text-zinc-400",
                        "Add at least one course to see the schedule preview."
                    }
                }
            }
        };
    }

    let schedule = Schedule::default(false, true, 3, 2, 2, true, true, true, true);

    rsx!(
        div { class: "bg-white rounded-2xl border border-amber-100 shadow-sm overflow-hidden",
            div { class: "px-5 py-3.5 bg-amber-50/70 border-b border-amber-100 flex items-center gap-2.5",
                div { class: "w-1.5 h-5 rounded-full bg-amber-400/70" }
                span { class: "text-sm font-semibold text-zinc-800", "Schedule Preview" }
            }
            div { class: "p-4",
                RunSchedule {
                    plan_config: plan_config.read().clone(),
                    schedule
                }
            }
        }
    )
}
