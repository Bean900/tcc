use chrono::NaiveTime;
use dioxus::prelude::*;
use uuid::Uuid;
use web_sys::console;

use crate::{
    error,
    side::{Headline1, Input, InputError, InputTime, SavingIcon, WarnButton},
    storage::{CourseData, StorageManager},
};

#[derive(PartialEq, Clone, Copy)]
pub struct CoursesParam {
    cook_and_run_id: Uuid,
    course_list: Signal<Vec<CourseParam>>,
    selected_course: Signal<Option<Uuid>>,
    creating_course: Signal<bool>,
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
        let mut course_list: Vec<CourseParam> = course_data_list
            .iter()
            .map(|c| CourseParam::new(cook_and_run_id, c.clone()))
            .collect();

        course_list.sort_by_key(|course| course.time);

        CoursesParam {
            cook_and_run_id,
            course_list: use_signal(|| course_list),
            selected_course: use_signal(|| selected_course),
            creating_course: use_signal(|| false),
        }
    }
}

fn update_course<'a>(
    cook_and_run_id: Uuid,
    course_param: &'a CourseParam,
) -> Result<&'a CourseParam, String> {
    let mut storage = StorageManager::get_lock()?;
    let course = course_param.to_course_data();
    storage.update_course_of_cook_and_run(cook_and_run_id, &course)?;
    Ok(course_param)
}

fn del_course(cook_and_run_id: Uuid, course_id: Uuid) -> Result<(), String> {
    let mut storage = StorageManager::get_lock()?;
    storage.delete_course_of_cook_and_run(cook_and_run_id, course_id)?;
    Ok(())
}

fn create_course(cook_and_run_id: Uuid) -> Result<CourseParam, String> {
    let course = CourseParam::default(cook_and_run_id);

    let mut storage = StorageManager::get_lock()?;
    storage.create_course_of_cook_and_run(cook_and_run_id, &course.to_course_data())?;
    Ok(course)
}

fn set_course_with_more_hosts(cook_and_run_id: Uuid, course_id: Uuid) -> Result<(), String> {
    let mut storage = StorageManager::get_lock()?;
    storage.update_course_with_more_hosts_of_cook_and_run(cook_and_run_id, course_id)?;
    Ok(())
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
    let cook_and_run_id = param.cook_and_run_id;
    let mut course_list = param.course_list.clone();
    let mut selected_course = param.selected_course.clone();
    let mut creating_course = param.creating_course.clone();
    rsx! {
        section {
            Headline1 { headline: "Courses".to_string() }

            // Scrollable grid
            div { class: "grid grid-cols-1 gap-4 p-8 max-h-[calc(100vh-16rem)] overflow-y-auto pr-6",
                for (index , course) in course_list.iter().enumerate() {
                    div { class: "relative bg-[#fdfaf6] shadow-md rounded-xl p-6 hover:shadow-lg transition-all",
                        SavingIcon {
                            saving: course.saving,
                            error: "{course.saving_error}",
                        }
                        div { class: "grid grid-cols-10 gap-4",
                            div {
                                class: "flex flex-col col-span-7",
                                key: course_id,
                                span { class: "text-sm font-semibold mb-1 text-gray-700",
                                    "Name:"
                                }
                                Input {
                                    place_holer: "Course name",
                                    value: "{course.name}",
                                    is_error: !course.name_error.is_empty(),
                                    oninput: move |e: Event<FormData>| {
                                        let name_value = e.value();
                                        course_list
                                            .with_mut(|course_list_mut| {
                                                let course = course_list_mut.get_mut(index).expect("Expected course");
                                                course.saving = true;
                                                course.name = name_value.clone();
                                                if name_value.trim().is_empty() {
                                                    course.name_error = "Course name cannot be empty!".to_string();
                                                } else {
                                                    course.name_error = "".to_string();
                                                }
                                            });
                                        let course = course_list.read().get(index).expect("Expected course").clone();
                                        spawn(async move {
                                            let saving_error = match update_course(cook_and_run_id, &course) {
                                                Ok(_) => {
                                                    console::log_1(
                                                        &format!("Course {} saved successfully", course.name).into(),
                                                    );
                                                    "".to_string()
                                                }
                                                Err(e) => {
                                                    console::error_1(&format!("Error saving course: {}", e).into());
                                                    "Error when saving course!".to_string()
                                                }
                                            };
                                            course_list
                                                .with_mut(|course_list| {
                                                    let course = course_list.get_mut(index).expect("Expected course");
                                                    course.saving = false;
                                                    course.saving_error = saving_error;
                                                });
                                        });
                                    },
                                }
                                InputError { error: "{course.name_error}" }
                            }
                            div { class: "flex flex-col col-span-3",
                                span { class: "text-sm font-semibold mb-1 text-gray-700",
                                    "Time:"
                                }
                                InputTime {
                                    value: course.time,
                                    is_error: !course.time_error.is_empty(),
                                    oninput: move |event: Event<FormData>| {
                                        let time = NaiveTime::parse_from_str(&event.value(), "%H:%M");
                                        course_list
                                            .with_mut(|course_list| {
                                                let course = course_list.get_mut(index).expect("Expected course");
                                                match time {
                                                    Ok(t) => {
                                                        course.saving = true;
                                                        course.time = t;
                                                        course.time_error = "".to_string();
                                                    }
                                                    Err(_) => {
                                                        course.time_error = "Time format is not correct!".to_string();
                                                    }
                                                };
                                            });
                                        if let Some(e) = time.err() {
                                            console::error_1(&format!("Time format is not correct: {}", e).into());
                                            return;
                                        }
                                        let course = course_list.read().get(index).expect("Expected course").clone();
                                        spawn(async move {
                                            let saving_error = match update_course(cook_and_run_id, &course) {
                                                Ok(_) => {
                                                    console::log_1(
                                                        &format!("Course {} saved successfully", course.name).into(),
                                                    );
                                                    "".to_string()
                                                }
                                                Err(e) => {
                                                    console::error_1(&format!("Error saving course: {}", e).into());
                                                    "Error when saving course!".to_string()
                                                }
                                            };
                                            course_list
                                                .with_mut(|course_list| {
                                                    let course = course_list.get_mut(index).expect("Expected course");
                                                    course.saving = false;
                                                    course.saving_error = saving_error;
                                                });
                                        });
                                    },
                                }
                                InputError { error: "{course.time_error}" }
                            }
                        }
                        div { class: "flex flex-wrap items-center gap-3",
                            WarnButton {
                                text: "Delete",
                                onclick: move |_| {
                                    course_list
                                        .with_mut(|course_list| {
                                            let course = course_list.get_mut(index).expect("Expected course");
                                            course.saving = true;
                                            course.saving_error = "".to_string();
                                        });
                                    let course = course_list.read().get(index).expect("Expected course").clone();
                                    spawn(async move {
                                        let result = del_course(cook_and_run_id, course.id);
                                        match result {
                                            Ok(_) => {
                                                console::log_1(
                                                    &format!("Course {} deleted successfully", course.name).into(),
                                                );
                                                course_list
                                                    .with_mut(|course_list| {
                                                        course_list.remove(index);
                                                    });
                                            }
                                            Err(e) => {
                                                console::error_1(&format!("Error deleting course: {}", e).into());
                                                course_list
                                                    .with_mut(|course_list| {
                                                        let course = course_list
                                                            .get_mut(index)
                                                            .expect("Expected course");
                                                        course.saving_error = "Error when deleting course!"
                                                            .to_string();
                                                        course.saving = false;
                                                    });
                                            }
                                        }
                                    });
                                },
                            }
                            div { class: "flex items-center gap-2",
                                input {
                                    r#type: "radio",
                                    name: "multi_participant_course",
                                    checked: selected_course.read().is_some_and(|c| c.eq(&course.id)),
                                    onchange: move |_| {
                                        course_list
                                            .with_mut(|course_list| {
                                                let course = course_list.get_mut(index).expect("Expected course");
                                                course.saving = true;
                                                course.saving_error = "".to_string();
                                            });
                                        let course = course_list.read().get(index).expect("Expected course").clone();
                                        spawn(async move {
                                            let result = set_course_with_more_hosts(cook_and_run_id, course.id);
                                            match result {
                                                Ok(_) => {
                                                    console::log_1(
                                                        &format!(
                                                            "Course {} set successfully as main course",
                                                            course.name,
                                                        )
                                                            .into(),
                                                    );
                                                    course_list
                                                        .with_mut(|course_list| {
                                                            let course = course_list
                                                                .get_mut(index)
                                                                .expect("Expected course");
                                                            course.saving = false;
                                                        });
                                                    selected_course.set(Some(course.id));
                                                }
                                                Err(e) => {
                                                    console::error_1(
                                                        &format!("Error when setting main course: {}", e).into(),
                                                    );
                                                    course_list
                                                        .with_mut(|course_list| {
                                                            let course = course_list
                                                                .get_mut(index)
                                                                .expect("Expected course");
                                                            course.saving_error = "Error when setting main course!"
                                                                .to_string();
                                                            course.saving = false;
                                                        });
                                                }
                                            }
                                        });
                                    },
                                }
                                label { class: "text-sm text-gray-700", "Allow more hosts!" }
                            }
                        }
                    }
                }

                div {
                    if *creating_course.read() {
                        a { class: "border-4 border-dashed border-gray-300 rounded-xl p-6 flex items-center justify-center text-gray-400 hover:bg-[#fdfaf6] hover:text-[#C66741] hover:scale-105 transition-all duration-200 cursor-pointer",
                            div {
                                SavingIcon { saving: true, error: "".to_string() }
                            }
                        }
                    } else {
                        a {
                            class: "border-4 border-dashed border-gray-300 rounded-xl p-6 flex items-center justify-center text-gray-400 hover:bg-[#fdfaf6] hover:text-[#C66741] hover:scale-105 transition-all duration-200 cursor-pointer",
                            onclick: move |_| {
                                creating_course.set(true);
                                spawn(async move {
                                    let result = create_course(cook_and_run_id);
                                    match result {
                                        Ok(course) => {
                                            console::log_1(&format!("Course created successfully").into());
                                            course_list.push(course);
                                            creating_course.set(false);
                                        }
                                        Err(e) => {
                                            console::error_1(
                                                &format!("Error when setting main course: {}", e).into(),
                                            );
                                            error("Connection Error", "Error when creating course!");
                                        }
                                    }
                                });
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
}
