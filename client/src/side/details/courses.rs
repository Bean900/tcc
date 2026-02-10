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
use chrono::NaiveTime;
use dioxus::prelude::*;
use uuid::Uuid;
use web_sys::console;

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

#[component]
fn CoursesContent(cook_and_run_id: Uuid, course_list: Vec<CourseData>) -> Element {
    let mut course_list_signal = use_signal(|| {
        course_list
            .iter()
            .map(|c| CourseParam::from_course_data(c))
            .collect::<Vec<CourseParam>>()
    });
    rsx! {
        section {
            Headline1 { headline: "Courses".to_string() }

            // Scrollable grid
            div { class: "grid grid-cols-1 gap-4 p-8 max-h-[calc(100vh-16rem)] overflow-y-auto pr-6",
                for course in course_list_signal.read().iter().cloned() {
                    div { class: "relative bg-[#fdfaf6] shadow-md rounded-xl p-6 hover:shadow-lg transition-all",

                        div { class: "grid grid-cols-10 gap-4",
                            div { class: "flex flex-col col-span-7",
                                span { class: "text-sm font-semibold mb-1 text-gray-700",
                                    "Name:"
                                }
                                Input {
                                    place_holer: "Course name",
                                    value: "{course.name.clone()}",
                                    is_error: !course.name_error.clone().is_empty(),
                                    oninput: move |e: Event<FormData>| {
                                        let name_value = e.value().trim().to_string();
                                        let mut list = course_list_signal.write();
                                        if let Some(c) = list.iter_mut().find(|c| c.id == course.id) {
                                            check_name(c, &name_value);
                                        }
                                    },
                                }
                                InputError { error: "{course.name_error.clone()}" }
                            }
                            div { class: "flex flex-col col-span-3",
                                span { class: "text-sm font-semibold mb-1 text-gray-700",
                                    "Time:"
                                }
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
                        div { class: "flex flex-wrap items-center gap-3",
                            div { class: "flex items-center gap-2",
                                input {
                                    r#type: "radio",
                                    name: "multi_participant_course",
                                    checked: course.has_multiple_hosts,
                                    onchange: move |_| {
                                        let mut list = course_list_signal.write();
                                        for c in list.iter_mut() {
                                            if c.id == course.id {
                                                if !c.has_multiple_hosts {
                                                    c.has_multiple_hosts = true;
                                                    c.is_updated = true;
                                                }
                                            } else {
                                                if c.has_multiple_hosts {
                                                    c.has_multiple_hosts = false;
                                                    c.is_updated = true;
                                                }
                                            }
                                        }
                                    },
                                }
                                label { class: "text-sm text-gray-700", "Allow more hosts!" }
                            }

                            div { class: "ml-auto",
                                {
                                    DeleteButtonProps::new(
                                        async_action!(
                                            { if ! course.is_new { let mut storage_signal = use_context::< Signal <
                                            StorageManager >> (); let mut storage = storage_signal.write(); let
                                            result = storage.delete_course_of_cook_and_run(cook_and_run_id, course
                                            .id). await; if let Err(e) = result { console::error_1(&
                                            format!("Error deleting course: {}", e) .into()); return; } } let mut
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

                div {
                    a {
                        class: "border-4 border-dashed border-gray-300 rounded-xl p-6 flex items-center justify-center text-gray-400 hover:bg-[#fdfaf6] hover:text-[#C66741] hover:scale-105 transition-all duration-200 cursor-pointer",
                        onclick: move |_| {
                            let mut list = course_list_signal.write();
                            list.push(CourseParam::default());
                        },
                        div {
                            div { class: "text-5xl font-bold", "+" }
                        }
                    }
                }

                div { class: "flex justify-end w-full ",
                    ConfirmButton {
                        action: async_action!(
                            { let mut storage_signal = use_context::< Signal < StorageManager >> (); let mut
                            storage = storage_signal.write(); let mut list = course_list_signal.write(); for
                            course in list.iter_mut() { let name = course.name.clone(); let time = course
                            .time.format("%H:%M").to_string(); if ! check_name(course, & name) ||!
                            check_time(course, & time) { console::error_1(&
                            format!("Validation errors in course: {}", course.id) .into()); continue; } if
                            course.is_new { let result = storage
                            .create_course_of_cook_and_run(cook_and_run_id, course.id, & course
                            .to_create_course()). await; if let Err(e) = result { console::error_1(&
                            format!("Error inserting course: {}", e) .into()); return; } else { course.is_new
                            = false; course.is_updated = false; } } else if course.is_updated { let result =
                            storage.update_course_of_cook_and_run(cook_and_run_id, course.id, & course
                            .to_update_course()). await; if let Err(e) = result { console::error_1(&
                            format!("Error updating course: {}", e) .into()); return; } else { course
                            .is_updated = false; } } } }
                        ),
                        text: "Save".to_string(),
                    }
                }
            }
        }
    }
}

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
            course_param.time_error = "Invalid time format!".to_string();
            false
        }
    }
}
