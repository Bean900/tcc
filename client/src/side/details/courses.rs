use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use chrono::NaiveTime;
use dioxus::prelude::*;
use uuid::Uuid;
use web_sys::console;

use crate::{
    side::{debounce, Input, InputError, InputTime, RedButton, SavingIcon},
    storage::{CourseData, LocalStorage, StorageW},
};

#[derive(PartialEq, Clone, Copy)]
pub struct CoursesParam {
    cook_and_run_id: Uuid,
    course_map: Signal<HashMap<Uuid, CourseParam>>,
    selected_course: Signal<Option<Uuid>>,
}

#[derive(PartialEq, Clone)]
pub struct CourseParam {
    cook_and_run_id: Uuid,
    id: Uuid,
    name: String,
    name_error: String,
    time: NaiveTime,
    time_error: String,
    saving: bool,
    saving_error: String,
}

impl CoursesParam {
    pub(crate) fn new(
        cook_and_run_id: Uuid,
        course_data_list: Vec<CourseData>,
        selected_course: Option<Uuid>,
    ) -> Self {
        let course_map: HashMap<Uuid, CourseParam> = course_data_list
            .iter()
            .map(|c| (c.id, CourseParam::new(cook_and_run_id, c.clone())))
            .collect();

        CoursesParam {
            cook_and_run_id,
            course_map: use_signal(|| course_map),
            selected_course: use_signal(|| selected_course),
        }
    }

    fn get_sorted_course_key_list(&self) -> Vec<Uuid> {
        let mut course_list: Vec<Uuid> = self
            .course_map
            .read()
            .iter()
            .map(|(k, _)| k.clone())
            .collect();
        course_list.sort_by_key(|id| self.get_course(id).time);
        course_list
    }

    fn get_course(&self, course_id: &Uuid) -> CourseParam {
        self.course_map
            .read()
            .get(&course_id)
            .cloned()
            .expect("Expect value in map!")
    }
    fn set_course(&mut self, course_param: CourseParam) {
        let storage = use_context::<Arc<Mutex<LocalStorage>>>();
        let mut storage = storage.lock().expect("Expected storage lock");
        let result = storage
            .update_course_in_cook_and_run(self.cook_and_run_id, course_param.to_course_data());
        if result.is_err() {
            console::error_1(
                &format!(
                    "Error while saving course: {}",
                    result.expect_err("Expect error"),
                )
                .into(),
            );
            return;
        }

        self.course_map
            .write()
            .insert(course_param.id, course_param);
    }

    fn del_course(&mut self, course_id: &Uuid) {
        let storage = use_context::<Arc<Mutex<LocalStorage>>>();
        let mut storage = storage.lock().expect("Expected storage lock");
        let result = storage.delete_course_in_cook_and_run(self.cook_and_run_id, course_id.clone());
        if result.is_err() {
            console::error_1(
                &format!(
                    "Error while deleting course: {}",
                    result.expect_err("Expect error"),
                )
                .into(),
            );
            return;
        }

        self.course_map.write().remove(course_id);
    }

    fn add_course_in_cook_and_rund_course(&mut self) {
        let course = CourseParam::default(self.cook_and_run_id);

        let storage = use_context::<Arc<Mutex<LocalStorage>>>();
        let mut storage = storage.lock().expect("Expected storage lock");
        let result =
            storage.add_course_in_cook_and_run(self.cook_and_run_id, course.to_course_data());
        if result.is_err() {
            console::error_1(
                &format!(
                    "Error while adding course: {}",
                    result.expect_err("Expect error"),
                )
                .into(),
            );
            return;
        }

        self.course_map.write().insert(course.id.clone(), course);
    }

    fn set_course_with_more_hosts(&mut self, course_id: &Uuid) {
        let storage = use_context::<Arc<Mutex<LocalStorage>>>();
        let mut storage = storage.lock().expect("Expected storage lock");
        let result = storage
            .update_course_with_more_hosts_in_cook_and_run(self.cook_and_run_id, course_id.clone());
        if result.is_err() {
            console::error_1(
                &format!(
                    "Error while setting course with more hosts: {}",
                    result.expect_err("Expect error"),
                )
                .into(),
            );
            return;
        }
        self.selected_course.set(Some(*course_id));
    }
}

impl CourseParam {
    fn new(cook_and_run_id: Uuid, course_data: CourseData) -> Self {
        CourseParam {
            cook_and_run_id,
            id: course_data.id,
            name: course_data.name,
            name_error: "".to_string(),
            time: course_data.time,
            time_error: "".to_string(),
            saving: false,
            saving_error: "".to_string(),
        }
    }

    fn default(cook_and_run_id: Uuid) -> Self {
        CourseParam {
            cook_and_run_id,
            id: Uuid::new_v4(),
            name: "".to_string(),
            name_error: "".to_string(),
            time: NaiveTime::from_hms_opt(0, 0, 0).expect("Expect time!"),
            time_error: "".to_string(),
            saving: false,
            saving_error: "".to_string(),
        }
    }

    fn to_course_data(&self) -> CourseData {
        CourseData {
            id: self.id,
            name: self.name.clone(),
            time: self.time.clone(),
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


                for course_id in param.get_sorted_course_key_list() {

                    {
                        rsx! {
                            div { class: "relative bg-[#fdfaf6] shadow-md rounded-xl p-6 hover:shadow-lg transition-all",
                                SavingIcon {
                                    saving: param.get_course(&course_id).saving,
                                    error: param.get_course(&course_id).saving_error,
                                }
                            
                                div { class: "grid grid-cols-10 gap-4",
                                    div { class: "flex flex-col col-span-7", key: course_id,
                                        span { class: "text-sm font-semibold mb-1 text-gray-700", "Name:" }
                                        Input {
                                            place_holer: "Course name",
                                            value: param.get_course(&course_id).name,
                                            is_error: !param.get_course(&course_id).name_error.is_empty(),
                                            oninput: move |e: Event<FormData>| {
                                                let name_value = e.value();
                                                let mut course = param.get_course(&course_id);
                                                course.name = name_value.clone();
                                                if name_value.trim().is_empty() {
                                                    course.name_error = "Course name cannot be empty!".to_string();
                                                } else {
                                                    course.name_error = "".to_string();
                                                }
                                                param.set_course(course);
                                            },
                                        }
                                        InputError { error: param.get_course(&course_id).name_error }
                                    }
                            
                                    div { class: "flex flex-col col-span-3",
                                        span { class: "text-sm font-semibold mb-1 text-gray-700", "Time:" }
                                        InputTime {
                                            value: param.get_course(&course_id).time,
                                            is_error: !param.get_course(&course_id).time_error.is_empty(),
                                            oninput: move |event: Event<FormData>| {
                                                let time = NaiveTime::parse_from_str(&event.value(), "%H:%M");
                                                let mut course = param.get_course(&course_id);
                                                if time.is_err() {
                                                    console::error_1(
                                                        &format!(
                                                            "Time format is not correct: {}",
                                                            time.expect_err("Expect error"),
                                                        )
                                                            .into(),
                                                    );
                                                    course.time_error = "Time format is not correct!".to_string();
                                                    param.set_course(course);
                                                    return;
                                                }
                                                course.time_error = "".to_string();
                                                let time = time.expect("Expect time");
                                                course.time = time;
                                                param.set_course(course);
                                            },
                                        }
                                        InputError { error: param.get_course(&course_id).time_error }
                                    }
                                }
                            
                                div { class: "flex flex-wrap items-center gap-3",
                            
                            
                                    RedButton {
                                        text: "Delete",
                                        onclick: move |_| {
                                            param.del_course(&course_id);
                                        },
                                    }
                            
                            
                                    div { class: "flex items-center gap-2",
                            
                                        input {
                                            r#type: "radio",
                                            name: "multi_participant_course",
                                            checked: param.selected_course.read().is_some_and(|c| c.eq(&course_id)),
                                            onchange: move |_| {
                                                param.set_course_with_more_hosts(&course_id);
                                            },
                                        }
                                        label { class: "text-sm text-gray-700", "Allow more hosts!" }
                                    }
                                }
                            }
                        }
                    }
                }

                div {
                    a {
                        class: "border-4 border-dashed border-gray-300 rounded-xl p-6 flex items-center justify-center text-gray-400 hover:bg-[#fdfaf6] hover:text-[#C66741] hover:scale-105 transition-all duration-200 cursor-pointer",
                        onclick: move |_| {
                            param.add_course_in_cook_and_rund_course();
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
