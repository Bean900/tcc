use crate::side::{AsyncAction, DeleteButtonProps};
use crate::storage::{CourseCreate, CourseUpdate};
use crate::{
    async_action,
    side::{
        details::{ErrorPage, LoadingPage},
        ConfirmButton, Headline1, Input, InputError, InputTime,
    },
    storage::{CourseData, StorageManager},
};
use async_std::task::sleep;
use chrono::NaiveTime;
use dioxus::prelude::*;
use std::time::Duration;
use uuid::Uuid;
use web_sys::console;

// ─────────────────────────────────────────────
//  CSS: Keyframe-Animation für den grünen Glow
// ─────────────────────────────────────────────

const SAVE_GLOW_CSS: &str = r#"
@keyframes save-glow {
    0%   {
        border-color: #d1fae5;
        box-shadow: 0 0 0 0px rgba(34, 197, 94, 0),
                    0 1px 3px 0 rgba(0, 0, 0, 0.06);
    }
    20%  {
        border-color: #22c55e;
        box-shadow: 0 0 0 5px rgba(34, 197, 94, 0.22),
                    0 1px 3px 0 rgba(0, 0, 0, 0.06);
    }
    55%  {
        border-color: #16a34a;
        box-shadow: 0 0 0 5px rgba(34, 197, 94, 0.10),
                    0 1px 3px 0 rgba(0, 0, 0, 0.06);
    }
    100% {
        border-color: #bbf7d0;
        box-shadow: 0 0 0 0px rgba(34, 197, 94, 0),
                    0 1px 3px 0 rgba(0, 0, 0, 0.06);
    }
}
.save-glow-card {
    animation: save-glow 2s ease-in-out forwards;
}
.save-glow-card .save-glow-header {
    background-color: rgba(240, 253, 244, 0.70) !important;
    border-bottom-color: #bbf7d0 !important;
    transition: background-color 0.4s ease, border-color 0.4s ease;
}
.save-glow-card .save-glow-accent {
    background-color: rgba(34, 197, 94, 0.75) !important;
    transition: background-color 0.4s ease;
}
.save-glow-card .save-glow-title {
    color: #166534 !important;
    transition: color 0.4s ease;
}
"#;

#[derive(PartialEq, Clone)]
struct CourseParam {
    id: Uuid,
    name: String,
    name_error: String,
    time: NaiveTime,
    time_error: String,
    has_multiple_hosts: bool,
    is_updated: bool,
    is_new: bool,
}

impl Default for CourseParam {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4(),
            name: "".to_string(),
            name_error: "".to_string(),
            time: NaiveTime::from_hms_opt(0, 0, 0).unwrap(),
            time_error: "".to_string(),
            has_multiple_hosts: false,
            is_updated: false,
            is_new: true,
        }
    }
}

impl CourseParam {
    fn from_course_data(course_data: &CourseData) -> Self {
        Self {
            id: course_data.id,
            name: course_data.name.clone(),
            name_error: "".to_string(),
            time: course_data.time,
            time_error: "".to_string(),
            has_multiple_hosts: course_data.has_multiple_hosts,
            is_updated: false,
            is_new: false,
        }
    }

    fn to_create_course(&self) -> CourseCreate {
        CourseCreate {
            name: self.name.clone(),
            time: self.time,
            has_multiple_hosts: self.has_multiple_hosts,
        }
    }

    fn to_update_course(&self) -> CourseUpdate {
        CourseUpdate {
            name: self.name.clone(),
            time: self.time,
            has_multiple_hosts: self.has_multiple_hosts,
        }
    }
}

// ─────────────────────────────────────────────
//  Root
// ─────────────────────────────────────────────

#[component]
pub fn Courses(cook_and_run_id: Uuid) -> Element {
    let storage = use_context::<Signal<StorageManager>>();
    let course_list = use_resource(move || {
        let storage = storage.clone();
        async move {
            let storage = storage.read().clone();
            storage
                .select_cook_and_run_course_list(cook_and_run_id)
                .await
        }
    });

    match &*course_list.read_unchecked() {
        None => rsx!(LoadingPage {}),
        Some(Err(e)) => rsx!(ErrorPage {
            error_text:
                "Could not load project. You may need to log in or the servers may be offline."
                    .to_string(),
            error_details: e.clone(),
        }),
        Some(Ok(course_list)) => rsx!(CoursesContent {
            cook_and_run_id,
            course_list: course_list.clone()
        }),
    }
}

// ─────────────────────────────────────────────
//  Content
// ─────────────────────────────────────────────

#[component]
fn CoursesContent(cook_and_run_id: Uuid, course_list: Vec<CourseData>) -> Element {
    let mut course_list_signal = use_signal(|| {
        course_list
            .iter()
            .map(|c| CourseParam::from_course_data(c))
            .collect::<Vec<CourseParam>>()
    });

    // true  → grüner Glow auf der Aktions-Card für 2 s
    let mut save_success_signal = use_signal(|| false);

    // Shared label token
    const LBL: &str =
        "block text-[11px] font-semibold tracking-[0.12em] uppercase text-amber-700/70 mb-1.5";

    // Abgeleitet: ungespeicherte Änderungen vorhanden wenn mind. ein Kurs
    // neu ist oder geändert wurde → Button aktiv
    let can_save = course_list_signal
        .read()
        .iter()
        .any(|c| c.is_new || c.is_updated);

    let is_success = *save_success_signal.read();

    // Aktions-Card folgt dem selben Glow-Pattern wie overview / startend / calculate
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

    rsx! {
        // ── Keyframe-CSS einbinden ────────────────────────────────
        style { dangerous_inner_html: SAVE_GLOW_CSS }

        section { class: "px-8 py-6 space-y-8",

            // ── Page header ───────────────────────────────────────
            Headline1 { headline: "Courses".to_string() }

            // ── Card list ─────────────────────────────────────────
            div { class: "grid grid-cols-1 gap-4 max-h-[calc(100vh-16rem)] overflow-y-auto pr-1",

                for course in course_list_signal.read().iter().cloned() {

                    // ── Course card ───────────────────────────────
                    div { class: "bg-white rounded-2xl border border-amber-100 shadow-sm overflow-hidden \
                                  transition-shadow duration-150 hover:shadow-md",

                        // Card header bar
                        div { class: "px-5 py-3 bg-amber-50/70 border-b border-amber-100 flex items-center gap-2.5",
                            div { class: "w-1.5 h-4 rounded-full bg-amber-400/70" }
                            span { class: "text-[11px] font-semibold tracking-[0.12em] uppercase text-amber-700/70",
                                if course.name.is_empty() { "New course" } else { "{course.name}" }
                            }
                        }

                        // Card body
                        div { class: "px-5 py-4 space-y-4",

                            // Name + Time
                            div { class: "grid grid-cols-10 gap-4",

                                div { class: "flex flex-col col-span-7",
                                    label { class: "{LBL}", "Name" }
                                    Input {
                                        place_holer: "Course name",
                                        value: "{course.name.clone()}",
                                        is_error: !course.name_error.clone().is_empty(),
                                        oninput: move |e: Event<FormData>| {
                                            let name_value = e.value().to_string();
                                            let mut list = course_list_signal.write();
                                            if let Some(c) = list.iter_mut().find(|c| c.id == course.id) {
                                                check_name(c, &name_value);
                                            }
                                        },
                                    }
                                    InputError { error: "{course.name_error.clone()}" }
                                }

                                div { class: "flex flex-col col-span-3",
                                    label { class: "{LBL}", "Time" }
                                    InputTime {
                                        value: course.time.clone(),
                                        is_error: !course.time_error.clone().is_empty(),
                                        oninput: move |event: Event<FormData>| {
                                            let mut list = course_list_signal.write();
                                            if let Some(c) = list.iter_mut().find(|c| c.id == course.id) {
                                                check_time(c, &event.value());
                                            }
                                        },
                                    }
                                    InputError { error: "{course.time_error.clone()}" }
                                }
                            }

                            // Footer row: delete button
                            div { class: "flex items-center justify-between pt-1",
                                {
                                    DeleteButtonProps::new(
                                        async_action!(
                                            {
                                                if !course.is_new {
                                                    let mut storage_signal = use_context::<Signal<StorageManager>>();
                                                    let mut storage = storage_signal.write();
                                                    let result = storage
                                                        .delete_course_of_cook_and_run(cook_and_run_id, course.id)
                                                        .await;
                                                    if let Err(e) = result {
                                                        console::error_1(
                                                            &format!("Error deleting course: {}", e).into(),
                                                        );
                                                        return;
                                                    }
                                                }
                                                let mut list = course_list_signal.write();
                                                list.retain(|c| c.id != course.id);
                                            }
                                        ),
                                        None,
                                    )
                                }
                            }
                        }
                    }
                }

                // ── Add course button ─────────────────────────────
                a {
                    class: "flex items-center justify-center gap-3 \
                            rounded-2xl border-2 border-dashed border-amber-200 \
                            bg-amber-50/30 px-6 py-5 \
                            text-amber-400 hover:text-amber-600 \
                            hover:border-amber-400 hover:bg-amber-50/60 \
                            transition-all duration-200 cursor-pointer group",
                    onclick: move |_| {
                        let mut list = course_list_signal.write();
                        list.push(CourseParam::default());
                    },
                    div { class: "w-7 h-7 rounded-full border-2 border-current \
                                  flex items-center justify-center \
                                  text-lg font-bold leading-none \
                                  group-hover:scale-110 transition-transform duration-150",
                        "+"
                    }
                    span { class: "text-sm font-semibold tracking-wide", "Add course" }
                }

                // ── Aktions-Card: glowen wie alle anderen Speicher-Cards ──
                div { class: "{card_class}",

                    div { class: "{header_class}",
                        div { class: "{accent_class}" }
                        span { class: "{title_class}",
                            if is_success { "Courses  ✓" } else { "Courses" }
                        }
                    }

                    div { class: "px-5 pb-5 pt-4 flex justify-end",
                        div { class: "{save_btn_wrapper_class}",
                            ConfirmButton {
                                action: async_action!(
                                    {
                                        let mut storage_signal = use_context::<Signal<StorageManager>>();
                                        let mut storage = storage_signal.write();
                                        let mut list = course_list_signal.write();
                                        for course in list.iter_mut() {
                                            let name = course.name.clone();
                                            let time = course.time.format("%H:%M").to_string();
                                            if !check_name(course, &name) || !check_time(course, &time) {
                                                console::error_1(
                                                    &format!("Validation errors in course: {}", course.id).into(),
                                                );
                                                continue;
                                            }
                                            if course.is_new {
                                                let result = storage
                                                    .create_course_of_cook_and_run(
                                                        cook_and_run_id,
                                                        course.id,
                                                        &course.to_create_course(),
                                                    )
                                                    .await;
                                                if let Err(e) = result {
                                                    console::error_1(
                                                        &format!("Error inserting course: {}", e).into(),
                                                    );
                                                    return;
                                                } else {
                                                    course.is_new = false;
                                                    course.is_updated = false;
                                                }
                                            } else if course.is_updated {
                                                let result = storage
                                                    .update_course_of_cook_and_run(
                                                        cook_and_run_id,
                                                        course.id,
                                                        &course.to_update_course(),
                                                    )
                                                    .await;
                                                if let Err(e) = result {
                                                    console::error_1(
                                                        &format!("Error updating course: {}", e).into(),
                                                    );
                                                    return;
                                                } else {
                                                    course.is_updated = false;
                                                }
                                            }
                                        }
                                        // Erfolg: Glow für 2 s
                                        save_success_signal.set(true);
                                        spawn(async move {
                                            sleep(Duration::from_millis(2000)).await;
                                            save_success_signal.set(false);
                                        });
                                    }
                                ),
                                text: "Save".to_string(),
                            }
                        }
                    }
                }
            }
        }
    }
}

// ─────────────────────────────────────────────
//  Validation helpers
// ─────────────────────────────────────────────

fn check_name(course_param: &mut CourseParam, new_name: &str) -> bool {
    let name = new_name.trim();
    if course_param.name != name {
        course_param.is_updated = true;
        course_param.name = name.to_string();
    }

    if name.is_empty() {
        course_param.name_error = "Course name cannot be empty!".to_string();
        false
    } else {
        course_param.name_error = "".to_string();
        true
    }
}

fn check_time(course_param: &mut CourseParam, new_time: &str) -> bool {
    match NaiveTime::parse_from_str(new_time, "%H:%M") {
        Ok(t) => {
            course_param.time_error = "".to_string();
            if course_param.time != t {
                course_param.is_updated = true;
            }
            course_param.time = t;
            true
        }
        Err(_) => {
            if let Ok(time_with_sec) = NaiveTime::parse_from_str(new_time, "%H:%M:%S") {
                course_param.time_error = "".to_string();
                if course_param.time != time_with_sec {
                    course_param.is_updated = true;
                }
                course_param.time = time_with_sec;
                true
            } else {
                course_param.time_error = "Invalid time format!".to_string();
                false
            }
        }
    }
}
