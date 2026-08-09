use async_std::task::sleep;
use std::time::Duration;

use crate::{
    async_action,
    calculator::{Calculator, Course, DataMapper, Point, SchemaCalculator, Team},
    side::{
        details::{
            run_schedule::{run_schedule::RunSchedule, Schedule},
            ErrorPage, LoadingPage,
        },
        AsyncAction,
    },
    storage::{
        CourseData, Language, MeetingPointData, PlanConfigData, PlanData, StorageManager, TeamData,
    },
    ui::{
        buttons::ConfirmButton,
        cards::{BaseCard, CardHeader},
        forms::{Input, InputError},
        tokens::SAVE_GLOW_CSS,
        typography::{FieldLabel, Headline1, Headline2},
    },
    Route,
};
use chrono::NaiveDate;
use dioxus::prelude::*;
use uuid::Uuid;
use web_sys::console;

struct TableContent {
    host: TeamData,
    guest_list: Vec<(TeamData, bool)>,
}

impl TableContent {
    fn new_list(team_list: &[TeamData], plan: &PlanData) -> Vec<Self> {
        let team_map = team_list
            .iter()
            .map(|team| (team.id, team.clone()))
            .collect::<std::collections::HashMap<_, _>>();

        let hosting_map = plan
            .hosting_list
            .iter()
            .map(|hosting| (hosting.id, hosting.host))
            .collect::<std::collections::HashMap<_, _>>();

        let mut table_content_list = plan
            .walking_path
            .iter()
            .map(|(team_id, host_list_id)| {
                let host = if let Some(team) = team_map.get(team_id) {
                    team.clone()
                } else {
                    console::error_1(&format!("Team with id {} not found!", team_id).into());
                    TeamData::default()
                };

                let guest_list = host_list_id
                    .iter()
                    .map(|host_id| {
                        let guest_id = if let Some(guest_id) = hosting_map.get(host_id) {
                            *guest_id
                        } else {
                            console::error_1(
                                &format!("Hosting with id {} not found!", host_id).into(),
                            );
                            return (TeamData::default(), false);
                        };

                        if let Some(team) = team_map.get(&guest_id) {
                            (team.clone(), *team_id == guest_id)
                        } else {
                            console::error_1(
                                &format!("Guest Team with id {} not found!", guest_id).into(),
                            );
                            (TeamData::default(), false)
                        }
                    })
                    .collect::<Vec<(TeamData, bool)>>();

                TableContent { host, guest_list }
            })
            .collect::<Vec<_>>();

        table_content_list.sort_by(|a, b| a.host.name.cmp(&b.host.name));
        table_content_list
    }
}

// ─────────────────────────────────────────────
//  Root Component
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
        Some(Err(e)) => {
            return rsx!(ErrorPage {
                error_text: "Could not load plan configuration. You may need to log in or the servers may be offline."
                    .to_string(),
                error_details: e.clone(),
            })
        }
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
        end_point: end_point.clone(),
    });

    let calculate_preview = rsx!(CalculatePreview {
        cook_and_run_id,
        plan_config: plan_config_signal.clone(),
        course_list,
        start_point,
        end_point,
    });

    rsx!(
        section { class: "w-full space-y-6 sm:space-y-8",

            // Page Header
            Headline1 {
                headline: "Calculation".to_string(),
                subtitle: Some(
                    "Configure plan settings, preview the event schedule, and calculate walking paths."
                        .to_string(),
                ),
            }

            // Settings & Preview Grid mit harmonischer Höhe
            div { class: "grid grid-cols-1 lg:grid-cols-2 gap-6 items-stretch w-full min-w-0",
                {calculate_settings}
                {calculate_preview}
            }

            // Walking Path Table / Prerequisite Warnings
            {calculate_plans}
        }
    )
}

// ─────────────────────────────────────────────
//  Settings Card
// ─────────────────────────────────────────────

#[component]
fn CalculateSettings(cook_and_run_id: Uuid, plan_config_signal: Signal<PlanConfigData>) -> Element {
    let mut save_success_signal = use_signal(|| false);
    let mut has_unsaved_changes = use_signal(|| false);

    let is_success = *save_success_signal.read();
    let can_save = *has_unsaved_changes.read();

    let title_val = plan_config_signal.read().title.clone();
    let is_title_empty = title_val.trim().is_empty();

    let card_class = if is_success { "save-glow-card" } else { "" };
    let header_class = if is_success { "save-glow-header" } else { "" };
    let save_btn_wrapper_class = if can_save && !is_title_empty {
        ""
    } else {
        "opacity-40 pointer-events-none cursor-not-allowed"
    };

    rsx!(
        style { dangerous_inner_html: SAVE_GLOW_CSS }

        BaseCard { class: card_class.to_string(),
            CardHeader {
                title: "Plan Configuration".to_string(),
                subtitle: Some("Set title, description, date and language for your event.".to_string()),
                class: header_class.to_string(),
            }

            div { class: "p-4 sm:p-6 space-y-4 flex flex-col justify-between h-full",

                div { class: "space-y-4",
                    // Title Input
                    div {
                        FieldLabel { text: "Title".to_string() }
                        Input {
                            place_holer: Some("Plan title".to_string()),
                            value: title_val,
                            is_error: is_title_empty,
                            oninput: move |e: FormEvent| {
                                has_unsaved_changes.set(true);
                                plan_config_signal.write().title = e.value();
                            },
                        }
                        if is_title_empty {
                            InputError { error: "Title cannot be empty!".to_string() }
                        }
                    }

                    // Description Input
                    div {
                        FieldLabel { text: "Description".to_string() }
                        Input {
                            place_holer: Some("Short description".to_string()),
                            value: plan_config_signal.read().description.clone(),
                            is_error: false,
                            oninput: move |e: FormEvent| {
                                has_unsaved_changes.set(true);
                                plan_config_signal.write().description = e.value();
                            },
                        }
                    }

                    // Date & Language Inputs
                    div { class: "grid grid-cols-1 sm:grid-cols-2 gap-4",

                        div {
                            FieldLabel { text: "Date".to_string() }
                            input {
                                r#type: "date",
                                class: "w-full px-3 py-2 rounded-xl border border-amber-200/80 bg-amber-50/30 text-sm text-zinc-800 focus:outline-none focus:ring-2 focus:ring-amber-400/40 focus:border-amber-400 transition-colors",
                                value: "{plan_config_signal.read().date}",
                                onchange: move |e: FormEvent| {
                                    if let Ok(date) = NaiveDate::parse_from_str(&e.value(), "%Y-%m-%d") {
                                        has_unsaved_changes.set(true);
                                        plan_config_signal.write().date = date;
                                    }
                                },
                            }
                        }

                        div {
                            FieldLabel { text: "Language".to_string() }
                            select {
                                class: "w-full px-3 py-2 rounded-xl border border-amber-200/80 bg-amber-50/30 text-sm text-zinc-800 focus:outline-none focus:ring-2 focus:ring-amber-400/40 focus:border-amber-400 transition-colors cursor-pointer",
                                value: "{plan_config_signal.read().language.to_string()}",
                                onchange: move |e: FormEvent| {
                                    has_unsaved_changes.set(true);
                                    plan_config_signal.write().language = Language::from_string(e.value());
                                },
                                option { value: "eng", "English" }
                                option { value: "deu", "German" }
                            }
                        }
                    }
                }

                // Card Footer Button
                div { class: "flex items-center justify-end pt-4 border-t border-amber-100/60 mt-4",
                    div { class: "{save_btn_wrapper_class}",
                        ConfirmButton {
                            text: if is_success { "Saved!".to_string() } else { "Save Settings".to_string() },
                            action: async_action!(
                                { let mut storage = use_context::< Signal < StorageManager >> ().write().clone();
                                let result = storage.update_plan_config_of_cook_and_run(cook_and_run_id, &
                                plan_config_signal.read()).await;
                                 if let Err(e) = &result { console::error_1(&
                                format!("Error updating plan config: {}", e) .into()); } else { console::log_1(&
                                "Plan configuration updated successfully".into()); has_unsaved_changes
                                .set(false); save_success_signal.set(true); spawn(async move {
                                sleep(Duration::from_millis(2000)).await;
                                save_success_signal.set(false);
                                });
                                }
                                }
                            ),
                        }
                    }
                }
            }
        }
    )
}

// ─────────────────────────────────────────────
//  Walking-path Table & Prerequisite Handling
// ─────────────────────────────────────────────

#[component]
fn CalculatePlans(
    cook_and_run_id: Uuid,
    course_list: Vec<CourseData>,
    start_point: Option<MeetingPointData>,
    end_point: Option<MeetingPointData>,
) -> Element {
    // ── Prerequisite Check 1: Time ordering conflicts ────────────
    let all_course_times = course_list.iter().map(|c| c.time).collect::<Vec<_>>();
    let mut time_error: Option<(&'static str, &'static str)> = None;

    if !all_course_times.is_empty() {
        if let Some(start) = start_point.as_ref().map(|mp| mp.time) {
            if start > *all_course_times.iter().min().unwrap() {
                time_error = Some((
                    "Invalid Start Point Time",
                    "The start point time must be scheduled before the earliest course serving time.",
                ));
            }
        }
        if let Some(end) = end_point.as_ref().map(|mp| mp.time) {
            if end < *all_course_times.iter().max().unwrap() {
                time_error = Some((
                    "Invalid End Point Time",
                    "The end point time must be scheduled after the latest course serving time.",
                ));
            }
        }
    }

    if let Some((err_title, err_desc)) = time_error {
        return rsx!(
            BaseCard {
                CardHeader {
                    title: "Time Sequence Error".to_string(),
                    subtitle: Some("Project timeline has configuration issues.".to_string()),
                }
                div { class: "p-6 flex flex-col items-center justify-center text-center space-y-3",
                    div { class: "w-10 h-10 rounded-full bg-red-100 flex items-center justify-center text-red-600 font-bold", "!" }
                    p { class: "font-semibold text-zinc-800 text-sm", "{err_title}" }
                    p { class: "text-xs text-zinc-500 max-w-md", "{err_desc}" }
                }
            }
        );
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
                error_text: "Could not load team list. You may need to log in or the servers may be offline."
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

    // ── Prerequisite Check 2: Missing End Point ────────────────────
    if end_point_signal.read().is_none() {
        return rsx!(
            BaseCard {
                CardHeader {
                    title: "End Point Required".to_string(),
                    subtitle: Some("Please set an end point for this project before generating a plan.".to_string()),
                }
                div { class: "p-6 sm:p-8 flex flex-col items-center justify-center text-center space-y-4",
                    div { class: "w-12 h-12 rounded-full bg-amber-100/80 flex items-center justify-center text-amber-700 text-xl font-bold", "⚑" }
                    div { class: "max-w-md space-y-1",
                        p { class: "font-semibold text-zinc-800 text-sm", "No End Point Configured" }
                        p { class: "text-xs text-zinc-500 leading-relaxed", "An end point is required to calculate distances and host allocations." }
                    }
                    button {
                        r#type: "button",
                        class: "px-4 py-2 bg-amber-500 hover:bg-amber-600 text-white rounded-xl text-xs font-semibold transition-all shadow-xs",
                        onclick: move |_| {
                            use_navigator().push(Route::StartEnd { cook_and_run_id });
                        },
                        "Configure End Point →"
                    }
                }
            }
        );
    }

    // ── Prerequisite Check 3: Missing Courses ──────────────────────
    if course_list.is_empty() {
        return rsx!(
            BaseCard {
                CardHeader {
                    title: "Courses Required".to_string(),
                    subtitle: Some("Add at least one course before calculating walking routes.".to_string()),
                }
                div { class: "p-6 sm:p-8 flex flex-col items-center justify-center text-center space-y-4",
                    div { class: "w-12 h-12 rounded-full bg-amber-100/80 flex items-center justify-center text-amber-700 text-xl font-bold", "🍽" }
                    div { class: "max-w-md space-y-1",
                        p { class: "font-semibold text-zinc-800 text-sm", "No Courses Found" }
                        p { class: "text-xs text-zinc-500 leading-relaxed", "You need to define the menu courses and times for your event first." }
                    }
                    button {
                        r#type: "button",
                        class: "px-4 py-2 bg-amber-500 hover:bg-amber-600 text-white rounded-xl text-xs font-semibold transition-all shadow-xs",
                        onclick: move |_| {
                            use_navigator().push(Route::Courses { cook_and_run_id });
                        },
                        "Add Courses →"
                    }
                }
            }
        );
    }

    // ── Prerequisite Check 4: No Plan Generated Yet ───────────────
    let plan = match &*plan_result.read_unchecked() {
        None => return rsx!(LoadingPage {}),
        Some(Err(e)) => {
            return rsx!(ErrorPage {
                error_text: "Could not load plan. You may need to log in or the servers may be offline."
                    .to_string(),
                error_details: e.clone(),
            })
        }
        Some(Ok(None)) => {
            return rsx!(
                BaseCard {
                    CardHeader {
                        title: "Ready to Calculate".to_string(),
                        subtitle: Some("All project data is set. Click below to generate the route plan.".to_string()),
                    }
                    div { class: "p-6 sm:p-8 flex flex-col items-center justify-center text-center space-y-4",
                        div { class: "w-12 h-12 rounded-full bg-amber-100/80 flex items-center justify-center text-amber-700 text-xl font-bold", "⚡" }
                        div { class: "max-w-md space-y-1",
                            p { class: "font-semibold text-zinc-800 text-sm", "No Plan Generated Yet" }
                            p { class: "text-xs text-zinc-500 leading-relaxed", "Run the calculation algorithm to create a walking-path matrix for all teams." }
                        }
                        ConfirmButton {
                            text: "Calculate Walking Path".to_string(),
                            action: async_action!(
                                { let end_point = end_point_signal.as_ref().expect("End point is not available");
                                let team_list = team_list_signal.read();
                                let course_list = course_list_signal.read();
                                 let calculator_result = SchemaCalculator::new(& end_point, & team_list,
                                & course_list); let calculator = match calculator_result { Ok(calculator) =>
                                calculator, Err(e) => { console::error_1(&
                                format!("Error creating calculator: {}", e) .into()); return; } }; let plan =
                                calculator.calculate(); let plan_data = plan.to_data(); let mut storage =
                                use_context::< Signal < StorageManager >> ().write().clone(); let result =
                                storage.update_plan_of_cook_and_run(cook_and_run_id, & plan_data). await; if let
                                Err(e) = & result { console::error_1(& format!("Error updating plan config: {}",
                                e) .into()); } else { console::log_1(& "Plan updated successfully".into());
                                plan_result.restart(); } }
                            ),
                        }
                    }
                }
            );
        }
        Some(Ok(Some(plan))) => plan.clone(),
    };

    let table_content = TableContent::new_list(&team_list, &plan);

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
        div { class: "space-y-4 w-full min-w-0",

            // Section Header & Recalculate Button
            div { class: "flex flex-col sm:flex-row sm:items-end justify-between gap-3",
                div {
                    Headline2 { headline: "Walking Path Matrix".to_string() }
                    p { class: "text-zinc-500 text-sm mt-0.5",
                        "{num_teams} Teams  ·  {num_courses} Courses"
                    }
                }

                ConfirmButton {
                    text: "Recalculate".to_string(),
                    action: async_action!(
                        { let end_point = end_point_signal.as_ref().expect("End point is not available");
                        let team_list = team_list_signal.read();
                        let course_list = course_list_signal.read();
                          let calculator_result = SchemaCalculator::new(& end_point, & team_list,
                        & course_list); let calculator = match calculator_result { Ok(calculator) =>
                        calculator, Err(e) => { console::error_1(&
                        format!("Error creating calculator: {}", e) .into()); return; } }; let plan =
                        calculator.calculate(); let plan_data = plan.to_data(); let mut storage =
                        use_context::< Signal < StorageManager >> ().write().clone(); let result =
                        storage.update_plan_of_cook_and_run(cook_and_run_id, & plan_data). await; if let
                        Err(e) = & result { console::error_1(& format!("Error updating plan config: {}",
                        e) .into()); } else { console::log_1(& "Plan updated successfully".into());
                        plan_result.restart(); } }
                    ),
                }
            }

            // Table Matrix Container
            div { class: "overflow-x-auto rounded-2xl border border-amber-200 shadow-sm bg-white",
                div { class: "min-w-max w-full",

                    // Header Row
                    div {
                        class: "grid border-b border-amber-200 bg-[#FAF0E2]",
                        style: "{grid_cols}",

                        div { class: "px-5 py-3.5 flex items-center gap-2.5",
                            div { class: "w-1.5 h-4 rounded-full bg-amber-400/70" }
                            span { class: "text-[11px] font-bold tracking-[0.18em] uppercase text-amber-700/60",
                                "Team"
                            }
                        }

                        for (idx , course_name) in headline_list.iter().enumerate() {
                            div {
                                key: "{idx}",
                                class: "px-5 py-3.5 border-l border-amber-200 flex items-center gap-2.5",
                                span { class: "text-[11px] font-bold tracking-[0.18em] uppercase text-amber-700/60",
                                    "{course_name}"
                                }
                            }
                        }
                    }

                    // Data Rows
                    for (row_idx , row) in table_content.iter().enumerate() {
                        div {
                            key: "{row_idx}",
                            class: format_args!(
                                "grid border-b border-amber-200/70 last:border-b-0 transition-colors duration-100 hover:bg-amber-100/70 cursor-pointer {}",
                                if row_idx % 2 == 0 { "bg-[#F8EFE1]" } else { "bg-[#FDFAF6]" },
                            ),
                            style: "{grid_cols}",
                            onclick: {
                                let host_id = row.host.id;
                                move |_| {
                                    use_navigator()
                                        .push(Route::Plan {
                                            cook_and_run_id,
                                            team_id: host_id,
                                        });
                                }
                            },

                            // Host Cell
                            div { class: "px-5 py-4 border-r border-amber-200/70 flex items-start gap-3",
                                div { class: "mt-1.5 w-6 h-6 shrink-0 rounded-md bg-[#D67229] flex items-center justify-center",
                                    span { class: "text-white text-[10px] font-bold",
                                        "{row_idx + 1}"
                                    }
                                }
                                div { class: "mt-0.5 flex flex-col min-w-0",
                                    span { class: "font-semibold text-sm leading-snug truncate text-zinc-800",
                                        "{row.host.name}"
                                    }
                                }
                            }

                            // Guest Cells
                            for (idx , (guest , is_host)) in row.guest_list.iter().enumerate() {
                                div {
                                    key: "guest-{idx}",
                                    class: "px-4 py-4 border-l border-amber-200/70 flex flex-col justify-center gap-1",

                                    div {
                                        class: format_args!(
                                            "inline-flex items-center gap-1.5 px-2.5 py-1 rounded-lg border w-fit max-w-full {}",
                                            if *is_host { "border-[#C66741]" } else { "border-amber-200" },
                                        ),
                                        span {
                                            class: format_args!(
                                                "text-xs font-semibold truncate {}",
                                                if *is_host { "text-[#C66741]" } else { "text-zinc-700" },
                                            ),
                                            "{guest.name}"
                                        }
                                    }

                                    span { class: "text-amber-900/40 text-[11px] leading-tight pl-1 truncate",
                                        "{guest.address.address}"
                                    }
                                }
                            }

                            // Fill Empty Columns
                            for fill_idx in row.guest_list.len()..num_courses {
                                div {
                                    key: "fill-{fill_idx}",
                                    class: "px-4 py-4 border-l border-amber-200/70 flex items-center",
                                    span { class: "text-amber-300 text-base select-none",
                                        "—"
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
//  Schedule Preview Card
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
            BaseCard {
                CardHeader {
                    title: "Schedule Preview".to_string(),
                    subtitle: Some("Visual timeline of courses and serving times.".to_string()),
                }
                div { class: "p-6 flex flex-col items-center justify-center text-center space-y-2 min-h-[180px]",
                    div { class: "w-10 h-10 rounded-full bg-amber-100/80 flex items-center justify-center text-amber-600 font-bold", "◷" }
                    p { class: "font-semibold text-zinc-800 text-sm", "No Courses Configured" }
                    p { class: "text-xs text-zinc-500 max-w-xs", "Add at least one course to see the schedule preview timeline." }
                }
            }
        };
    }

    let schedule = Schedule::default(false, true, 3, 2, 2, true, true, true, true);

    rsx!(
        BaseCard {
            CardHeader {
                title: "Schedule Preview".to_string(),
                subtitle: Some("Visual timeline of courses and serving times.".to_string()),
            }
            div { class: "p-4 sm:p-6",
                RunSchedule { plan_config: plan_config.read().clone(), schedule }
            }
        }
    )
}
