use async_std::task::sleep;
use chrono::NaiveTime;
use dioxus::prelude::*;
use std::time::Duration;
use uuid::Uuid;
use web_sys::console;

use crate::{
    async_action,
    side::{
        details::{ErrorPage, LoadingPage},
        AsyncAction,
    },
    storage::{CourseCreate, CourseData, CourseUpdate, StorageManager},
    ui::{
        buttons::{ConfirmButton, DashedActionButton, DeleteButtonProps},
        cards::{BaseCard, CardHeader},
        forms::{Input, InputError, InputTime},
        tokens::SAVE_GLOW_CSS,
        typography::{FieldLabel, Headline1},
    },
};

// ─────────────────────────────────────────────
//  CourseParam & Helpers (logik unverändert)
// ─────────────────────────────────────────────

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
//  Root Component
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
        None => rsx!(
            LoadingPage {}
        ),
        Some(Err(e)) => rsx!(
            ErrorPage {
                error_text: "Could not load project. You may need to log in or the servers may be offline."
                    .to_string(),
                error_details: e.clone(),
            }
        ),
        Some(Ok(course_list)) => rsx!(
            CoursesContent { cook_and_run_id, course_list: course_list.clone() }
        ),
    }
}

// ─────────────────────────────────────────────
//  Content Component (Mobil-Optimiert)
// ─────────────────────────────────────────────

#[component]
fn CoursesContent(cook_and_run_id: Uuid, course_list: Vec<CourseData>) -> Element {
    let mut course_list_signal = use_signal(|| {
        course_list
            .iter()
            .map(|c| CourseParam::from_course_data(c))
            .collect::<Vec<CourseParam>>()
    });

    let mut save_success_signal = use_signal(|| false);

    let can_save = course_list_signal
        .read()
        .iter()
        .any(|c| c.is_new || c.is_updated);

    let is_success = *save_success_signal.read();

    let card_class = if is_success {
        "save-glow-card"
    } else {
        ""
    };

    let header_class = if is_success {
        "save-glow-header"
    } else {
        ""
    };

    let save_btn_wrapper_class = if can_save {
        ""
    } else {
        "opacity-40 pointer-events-none cursor-not-allowed"
    };

    rsx! {
        // Keyframe-CSS einbinden
        style { dangerous_inner_html: SAVE_GLOW_CSS }

        // Container-Optimierung: w-full, px-3 auf Mobilgeräten
        section { class: "w-full max-w-5xl mx-auto px-3 sm:px-6 lg:px-8 py-4 sm:py-8 space-y-6 sm:space-y-8 overflow-hidden",

            // Page Header
            Headline1 {
                headline: "Courses".to_string(),
                subtitle: Some(
                    "Define the menu courses and serving times for your cooking event.".to_string(),
                ),
            }

            // Course List Grid
            div { class: "w-full space-y-4 sm:space-y-6",

                for course in course_list_signal.read().iter().cloned() {

                    // Course Card
                    BaseCard { key: "{course.id}",
                        CardHeader {
                            title: if course.name.is_empty() { "New Course".to_string() } else { course.name.clone() },
                            subtitle: Some(format!("Serving time: {}", course.time.format("%H:%M"))),
                        }

                        // Responsive Inner Padding (p-4 sm:p-6)
                        div { class: "p-4 sm:p-6 space-y-4",

                            // Form Grid mit min-w-0 gegen Überbreite
                            div { class: "grid grid-cols-1 sm:grid-cols-10 gap-3 sm:gap-4",

                                div { class: "sm:col-span-7 min-w-0 w-full",
                                    FieldLabel { text: "Course Name".to_string() }
                                    Input {
                                        place_holer: Some("e.g. Appetizer, Main Course, Dessert".to_string()),
                                        value: course.name.clone(),
                                        is_error: !course.name_error.is_empty(),
                                        oninput: move |e: FormEvent| {
                                            let name_value = e.value();
                                            let mut list = course_list_signal.write();
                                            if let Some(c) = list.iter_mut().find(|c| c.id == course.id) {
                                                check_name(c, &name_value);
                                            }
                                        },
                                    }
                                    InputError { error: course.name_error.clone() }
                                }

                                div { class: "sm:col-span-3 min-w-0 w-full",
                                    FieldLabel { text: "Serving Time".to_string() }
                                    InputTime {
                                        value: course.time.format("%H:%M").to_string(),
                                        oninput: move |e: FormEvent| {
                                            let mut list = course_list_signal.write();
                                            if let Some(c) = list.iter_mut().find(|c| c.id == course.id) {
                                                check_time(c, &e.value());
                                            }
                                        },
                                    }
                                    InputError { error: course.time_error.clone() }
                                }
                            }

                            // Footer Row
                            div { class: "flex items-center justify-end pt-3 border-t border-amber-100/60",
                                {
                                    DeleteButtonProps::new(
                                        async_action!(
                                            { if ! course.is_new { let mut storage_signal = use_context::< Signal <
                                            StorageManager >> (); let mut storage = storage_signal.write(); let
                                            result = storage.delete_course_of_cook_and_run(cook_and_run_id, course
                                            .id). await; if let Err(e) = result { console::error_1(&
                                            format!("Error deleting course: {}", e) .into(),); return; } } let mut
                                            list = course_list_signal.write(); list.retain(| c | c.id != course.id);
                                            }
                                        ),
                                        None,
                                    )
                                }
                            }
                        }
                    }
                }

                // Add Course Button
                div { class: "w-full",
                    DashedActionButton {
                        text: "Add New Course".to_string(),
                        onclick: move |_| {
                            let mut list = course_list_signal.write();
                            list.push(CourseParam::default());
                        },
                    }
                }

                // Action Save Card
                BaseCard { class: card_class.to_string(),
                    CardHeader {
                        title: "Save Courses".to_string(),
                        subtitle: Some("Apply and store all changes to courses and times.".to_string()),
                        class: header_class.to_string(),
                    }

                    div { class: "p-4 sm:p-6 flex items-center justify-end",
                        div { class: "w-full sm:w-auto {save_btn_wrapper_class}",
                            ConfirmButton {
                                text: if is_success { "Saved!".to_string() } else { "Save Changes".to_string() },
                                action: async_action!(
                                    { let mut storage_signal = use_context::< Signal < StorageManager >> (); let mut
                                    storage = storage_signal.write(); let mut list = course_list_signal.write(); for
                                    course in list.iter_mut() { let name = course.name.clone(); let time = course
                                    .time.format("%H:%M").to_string(); if ! check_name(course, & name) || !
                                    check_time(course, & time) { console::error_1(&
                                    format!("Validation errors in course: {}", course.id) .into(),); continue; } if
                                    course.is_new { let result = storage
                                    .create_course_of_cook_and_run(cook_and_run_id, course.id, & course
                                    .to_create_course(),). await; if let Err(e) = result { console::error_1(&
                                    format!("Error inserting course: {}", e) .into(),); return; } else { course
                                    .is_new = false; course.is_updated = false; } } else if course.is_updated { let
                                    result = storage.update_course_of_cook_and_run(cook_and_run_id, course.id, &
                                    course.to_update_course(),). await; if let Err(e) = result { console::error_1(&
                                    format!("Error updating course: {}", e) .into(),); return; } else { course
                                    .is_updated = false; } } } save_success_signal.set(true); spawn(async move {
                                    sleep(Duration::from_millis(2000)). await; save_success_signal.set(false); }); }
                                ),
                            }
                        }
                    }
                }
            }
        }
    }
}

// ─────────────────────────────────────────────
//  Validation Helpers (logik unverändert)
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