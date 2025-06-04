use std::sync::{Arc, Mutex};

use chrono::NaiveTime;
use dioxus::prelude::*;
use uuid::Uuid;
use web_sys::console;

use crate::{
    side::{debounce, Input, InputError, InputTime, RedButton, SavingIcon},
    storage::{CourseData, LocalStorage, StorageW},
};

#[derive(PartialEq, Clone)]
pub struct CoursesParam {
    cook_and_run_id: Uuid,
    courses_signal: Signal<Vec<CourseParam>>,
    selected_course: Signal<Option<Uuid>>,
}

#[derive(PartialEq, Clone, Copy)]
pub struct CourseParam {
    cook_and_run_id: Uuid,
    id: Uuid,
    name_signal: Signal<String>,
    name_error_signal: Signal<String>,
    time_signal: Signal<NaiveTime>,
    time_error_signal: Signal<String>,
    saving_signal: Signal<bool>,
    saving_error_signal: Signal<String>,
}

impl CoursesParam {
    pub(crate) fn new(cook_and_run_id: Uuid, course_data_list: Vec<CourseData>) -> Self {
        CoursesParam {
            cook_and_run_id,
            courses_signal: use_signal(|| {
                course_data_list
                    .iter()
                    .map(|c| CourseParam::new(cook_and_run_id, c.clone()))
                    .collect()
            }),
            selected_course: use_signal(|| course_data_list.get(0).map(|c| c.id)),
        }
    }
}

impl CourseParam {
    fn new(cook_and_run_id: Uuid, course_data: CourseData) -> Self {
        CourseParam {
            cook_and_run_id,
            id: course_data.id,
            name_signal: use_signal(|| course_data.name.clone()),
            name_error_signal: use_signal(|| "".to_string()),
            time_signal: use_signal(|| course_data.time),
            time_error_signal: use_signal(|| "".to_string()),
            saving_signal: use_signal(|| false),
            saving_error_signal: use_signal(|| "".to_string()),
        }
    }

    fn default(cook_and_run_id: Uuid) -> Self {
        CourseParam {
            cook_and_run_id,
            id: Uuid::new_v4(),
            name_signal: use_signal(|| "".to_string()),
            name_error_signal: use_signal(|| "".to_string()),
            time_signal: use_signal(|| {
                NaiveTime::from_hms_opt(0, 0, 0).expect("Expect static time to be correct")
            }),
            time_error_signal: use_signal(|| "".to_string()),
            saving_signal: use_signal(|| false),
            saving_error_signal: use_signal(|| "".to_string()),
        }
    }

    fn to_course_data(&self) -> CourseData {
        CourseData {
            id: self.id,
            name: self.name_signal.read().clone(),
            time: self.time_signal.read().clone(),
        }
    }
}

#[component]
pub fn Course(course: CourseParam, course_list_signal: Signal<Vec<CourseParam>>) -> Element {
    rsx! {
        a { class: "relative  shadow-md rounded-xl p-6 hover:shadow-lg transition-all",
            SavingIcon {
                saving_signal: course.saving_signal,
                error_signal: course.saving_error_signal,
            }

            div { class: "grid grid-cols-10 gap-4",
                div { class: "flex flex-col col-span-7", key: course.id,
                    span { class: "text-sm font-semibold mb-1 text-gray-700", "Name:" }
                    Input {
                        place_holer: "Course name",
                        value: course.name_signal,
                        error_signal: course.name_error_signal,
                        oninput: move |e: Event<FormData>| {
                            let name_value = e.value();
                            course.name_signal.set(name_value.clone());
                            if name_value.trim().is_empty() {
                                course.name_error_signal.set("Course name cannot be empty!".to_string());
                            } else {
                                course.name_error_signal.set("".to_string());
                            }
                        },
                    }
                    InputError { error_signal: course.name_error_signal }
                }

                div { class: "flex flex-col col-span-3",
                    span { class: "text-sm font-semibold mb-1 text-gray-700", "Time:" }
                    InputTime {
                        value: course.time_signal,
                        error_signal: course.time_error_signal,
                        oninput: move |event: Event<FormData>| {
                            let time = NaiveTime::parse_from_str(&event.value(), "%H:%M");
                            if time.is_err() {
                                console::error_1(
                                    &format!(
                                        "Time format is not correct: {}",
                                        time.expect_err("Expect error"),
                                    )
                                        .into(),
                                );
                                course.time_error_signal.set("Time format is not correct!".to_string());
                                return;
                            }
                            debounce(
                                course.time_signal,
                                course.saving_signal,
                                move |_| {
                                    let storage = use_context::<Arc<Mutex<LocalStorage>>>();
                                    let mut storage = storage.lock().expect("Expected storage lock");
                                    let result = storage
                                        .update_course_in_cook_and_run(
                                            course.cook_and_run_id,
                                            course.to_course_data(),
                                        );
                                    if result.is_err() {
                                        course
                                            .clone()
                                            .saving_error_signal
                                            .set("Error while saving data!".to_string());
                                    }
                                },
                            );
                            course.time_error_signal.set("".to_string());
                            let time = time.expect("Expect time");
                            course.time_signal.set(time);
                        },
                    }
                    InputError { error_signal: course.time_error_signal }
                }
            }

            div { class: "flex flex-wrap items-center gap-3",


                RedButton {
                    text: "Delete",
                    onclick: move |_| {
                        let mut to_delete_index_opt = None;
                        for (index, element) in course_list_signal.iter().enumerate() {
                            if element.id.eq(&course.clone().id) {
                                to_delete_index_opt = Some(index);
                            }
                        }
                        if let Some(to_delete_index) = to_delete_index_opt {
                            course_list_signal.remove(to_delete_index);
                        }
                    },
                }


                div { class: "flex items-center gap-2",

                    input {
                        r#type: "radio",
                        name: "multi_participant_course",
                        checked: true,
                        onchange: move |_| {},
                    }
                    label { class: "text-sm text-gray-700", "Allow multiple participants" }
                }
            }
        }
    }
}

#[component]
pub fn Courses(param: CoursesParam) -> Element {
    rsx! {
        section {
            h2 { class: "text-2xl font-bold mb-4", "Courses" }

            // Scrollable grid
            div { class: "grid grid-cols-1 gap-4 p-8 max-h-[calc(100vh-16rem)] overflow-y-auto pr-6",

                for course in param.courses_signal.read().iter() {
                    Course {
                        course: course.clone(),
                        course_list_signal: param.courses_signal.clone(),
                    }
                }
                div {
                    a {
                        class: "border-4 border-dashed border-gray-300 rounded-xl p-6 flex items-center justify-center text-gray-400 hover:bg-gray-50 hover:text-blue-500 hover:scale-105 transition-all duration-200 cursor-pointer",
                        onclick: move |_| {
                            let course = CourseParam::default(param.cook_and_run_id);
                            let storage = use_context::<Arc<Mutex<LocalStorage>>>();
                            let mut storage = storage.lock().expect("Expected storage lock");
                            let result = storage
                                .add_course_in_cook_and_run(param.cook_and_run_id, course.to_course_data());
                            if result.is_ok() {
                                param.courses_signal.push(course);
                            }
                        },
                        div {
                            div { class: "text-5xl font-bold", "+" }
                        }
                    }
                }
            
            }
        }
    }
}
