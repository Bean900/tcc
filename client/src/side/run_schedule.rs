use std::sync::{Arc, Mutex};

use chrono::NaiveTime;
use dioxus::prelude::*;
use uuid::Uuid;
use web_sys::{
    console, js_sys,
    wasm_bindgen::{JsCast, JsValue},
    window,
};

use crate::{
    side::{AddressSVG, PersonSVG, PhoneSVG, StartSVG, WarningSVG},
    storage::{
        mapper::{Hosting, Plan},
        AddressData, ContactData, CookAndRunData, CourseData, HostingData, LocalStorage,
        MeetingPointData, StorageR,
    },
    Route,
};

const POT: Asset = asset!("/assets/pot.png");
const SPATULA: Asset = asset!("/assets/spatula.png");
const LEAF_1: Asset = asset!("/assets/leaf_1.png");
const LEAF_2: Asset = asset!("/assets/leaf_2.png");
const CAKE: Asset = asset!("/assets/cake.png");
const CARROT: Asset = asset!("/assets/carrot.png");

fn get_cook_and_run(cook_and_run_id: Uuid) -> Result<CookAndRunData, String> {
    let storage = use_context::<Arc<Mutex<LocalStorage>>>();
    let storage = storage.lock().expect("Expected storage lock");

    let result = storage.select_cook_and_run(cook_and_run_id);
    result
}

#[component]
pub fn RunSchedule(cook_and_run_id: Uuid, contact_id: Uuid) -> Element {
    let cook_and_run = get_cook_and_run(cook_and_run_id);
    if cook_and_run.is_err() {
        console::error_1(
            &format!(
                "Error while loading cook and run: {}",
                cook_and_run.expect_err("Expect error"),
            )
            .into(),
        );
        return rsx!(
            div { "Cook and Run not found!" }
        );
    }
    let cook_and_run = cook_and_run.expect("Expect cook and run");
    let cook_and_run_date = cook_and_run.occur.format("%d.%m.%Y").to_string();
    let current_contact = cook_and_run
        .contact_list
        .iter()
        .filter(|c| c.id.eq(&contact_id))
        .next();
    if current_contact.is_none() {
        console::error_1(
            &format!(
                "Error while loading contact: {}",
                "Contact not found in cook and run",
            )
            .into(),
        );
        return rsx!(
            div { "Cook and Run not found!" }
        );
    }
    let current_contact = current_contact.expect("Expect current contact").clone();

    let plan_data = cook_and_run.top_plan;
    if plan_data.is_none() {
        console::error_1(&format!("Error while loading top plan: {}", "No top plan found",).into());
        return rsx!(
            div { "Cook and Run not found!" }
        );
    }

    let plan_data = plan_data.expect("Expect walking path");
    let plan = Plan::from_plan_data(
        &plan_data,
        &cook_and_run.course_list,
        &cook_and_run.contact_list,
    );

    let start_point = cook_and_run.start_point.clone();
    let end_point = cook_and_run.end_point.clone();

    let hosting_param_list: Vec<HostingParam> = plan
        .walking_path
        .iter()
        .filter(|current_walking_path| current_walking_path.0.id.eq(&current_contact.id))
        .flat_map(|current_walking_path| current_walking_path.1.iter())
        .map(|hosting| HostingParam::new(hosting, current_contact.id))
        .collect();

    let current_hosting = plan
        .hosting_list
        .iter()
        .find(|h| h.host.id.eq(&current_contact.id))
        .cloned()
        .expect("Expect one hosting for current contact");

    rsx!(
        div { class: "fixed bottom-4 right-4 z-50 flex gap-4",


            // Back Button
            button {
                class: "bg-gray-600 hover:bg-gray-700 text-white p-3 rounded-full shadow-lg",
                onclick: move |_| {
                    use_navigator()
                        .push(Route::ProjectCalculationPage {
                            cook_and_run_id,
                        });
                },
                svg {
                    class: "w-6 h-6",
                    fill: "none",
                    stroke: "currentColor",
                    stroke_width: "2",
                    view_box: "0 0 24 24",
                    xmlns: "http://www.w3.org/2000/svg",
                    path { d: "M15 19l-7-7 7-7" }
                }
            }
        

        }
        div {
            id: "section-to-print",
            class: "flex justify-center w-full h-full",
            div { class: "space-y-8 w-full max-w-3xl",
                div { class: "relative",
                    img {
                        src: POT,
                        class: "pointer-events-none absolute w-48 h-48 rotate-344",
                    }
                    img {
                        src: SPATULA,
                        class: "pointer-events-none absolute w-48 h-48 rotate-150 right-12",
                    }
                    img {
                        src: LEAF_2,
                        class: "pointer-events-none absolute w-24 h-24 rotate-300 right-10 top-55 transform -scale-x-100",
                    }
                }
                div { class: "text-center",
                    h1 { class: "font-chewy text-9xl text-[#543D2B] tracking-wide",
                        "COOK"
                    }
                    h1 { class: "font-chewy text-9xl text-[#543D2B] tracking-wide",
                        "AND RUN"
                    }
                }


                p { class: "font-gluten text-[#543D2B] mt-2",
                    if let Some(plan_text) = cook_and_run.plan_text {
                        "{plan_text}"
                    }
                }

                // MyInfo
                div { class: "grid grid-cols1 md:grid-cols-2 md:gap-x-4",
                    div {
                        MyInfo { contact: current_contact }
                    }
                    div {
                        MyHosting { hosting: current_hosting }
                    }
                }

                TimeLine {
                    cook_and_run_date,
                    start_point,
                    end_point,
                    walking_path: hosting_param_list,
                }
            }
        }
    )
}

#[derive(Default, Debug, Clone, PartialEq)]
struct HostingParam {
    id: Uuid,
    you_are_hosting: bool,
    course_name: String,
    course_team_name: String,
    course_team_tel: String,
    course_time: String,
    address: String,
    guests: Vec<(String, u32)>,
    diets: String,
}

impl HostingParam {
    fn new(hosting: &Hosting, current_contact_id: Uuid) -> Self {
        HostingParam {
            id: hosting.id,
            you_are_hosting: hosting.host.id.eq(&current_contact_id),
            course_name: hosting.course.name.clone(),
            course_team_name: hosting.host.team_name.clone(),
            course_team_tel: hosting.host.phone_number.clone(),
            course_time: hosting.course.time.format("%H:%M").to_string(),
            address: hosting.host.address.address.clone(),
            guests: hosting
                .guest_list
                .iter()
                .map(|g| (g.team_name.clone(), g.members))
                .collect(),
            diets: hosting
                .guest_list
                .iter()
                .flat_map(|g| g.diets.clone())
                .collect::<Vec<String>>()
                .join(", "),
        }
    }
}

#[component]
fn TimeLine(
    cook_and_run_date: String,
    start_point: Option<MeetingPointData>,
    end_point: Option<MeetingPointData>,
    walking_path: Vec<HostingParam>,
) -> Element {
    let headline = format!("{} - {}", "Timeline", cook_and_run_date);
    let start_name = start_point.clone().map_or("Start".to_string(), |s| s.name);
    let start_time = start_point
        .clone()
        .map_or("".to_string(), |s| s.time.format("%H:%S").to_string());
    let start_addr = start_point
        .clone()
        .map_or("".to_string(), |s| s.address.address);

    let end_name = end_point.clone().map_or("End".to_string(), |e| e.name);

    let end_time = end_point
        .clone()
        .map_or("".to_string(), |e| e.time.format("%H:%S").to_string());
    let end_addr = end_point
        .clone()
        .map_or("".to_string(), |e| e.address.address);

    rsx!(
        div { class: "flex items-center my-4",
            div { class: "flex-grow h-1 bg-[#C66741]" }
            span { class: "mx-4 text-xl font-gluten text-[#543D2B]", "{headline}" }
            div { class: "flex-grow h-1 bg-[#C66741]" }
        }

        div { class: "relative flex items-start",
            img {
                src: CARROT,
                class: "absolute mx-10 my-5 w-32 h-32 rotate-340",
            }
        }
        div {
            class: "relative h-(--container-height)",
            style: "
                  --container-height: 16rem;
                  --line-height: 0.2rem;
                  --item-height: calc((var(--container-height) - var(--line-height)) / 2);
                  --item-width: 10rem;
                  --item-overflow: calc(var(--item-width) / 2);
                  --item-margin: 1rem;
                ",

            div { class: "absolute top-1/2 -translate-y-1/2 h-(--line-height) w-full bg-[#543D2B]" }
            div { class: "flex h-(--container-height) items-baseline px-(--item-overflow)",
                if start_point.is_some() {
                    TimeLineElement {
                        is_up: false,
                        course_name: start_name,
                        course_time: start_time,
                        address: start_addr,
                        you_are_hosting: false,
                    }
                }
                for (i , event) in walking_path.iter().enumerate() {
                    TimeLineElement {
                        is_up: i % 2 != 0,
                        course_name: event.course_name.clone(),
                        course_team_name: event.course_team_name.clone(),
                        course_team_tel: event.course_team_tel.clone(),
                        course_time: event.course_time.clone(),
                        address: event.address.clone(),
                        you_are_hosting: event.you_are_hosting,
                    }
                }
                if end_point.is_some() {
                    if walking_path.len() % 2 != 0 {
                        TimeLineElement {
                            is_up: false,
                            course_name: end_name.clone(),
                            course_time: end_time.clone(),
                            address: end_addr.clone(),
                            you_are_hosting: false,
                        }
                    } else {
                        Placeholder {}
                    }
                }
            }
        }
        div { class: "relative flex items-start",
            img {
                src: CAKE,
                class: "absolute mx-155 -my-30 w-22 h-22 rotate-350",
            }
        }
    )
}

#[component]
fn Placeholder() -> Element {
    rsx!(
        div { class: "flex-1 text-center items-center" }
    )
}

#[component]
fn TimeLineElement(
    is_up: bool,
    course_name: String,
    course_team_name: Option<String>,
    course_team_tel: Option<String>,
    course_time: String,
    address: String,
    you_are_hosting: bool,
) -> Element {
    let point_format = if you_are_hosting {
        "bg-[#C66741] rotate-45"
    } else {
        "bg-[#543D2B] rounded-full"
    };

    let content = if you_are_hosting {
        rsx!(
            div { class: "font-thin flex items-start",
                StartSVG {}
                div { class: "mx-0 my-0.5 text-[#543D2B] font-gluten",
                    span { "Your Turn!" }
                }
            }
        )
    } else {
        let address_split: Vec<String> = address
            .split(",")
            .map(|s| s.to_string())
            .collect::<Vec<String>>();

        rsx!(

            if course_team_tel.is_some() {
                div { class: "font-thin flex items-start",
                    PhoneSVG {}
                    div { class: "mx-0   text-[#543D2B] font-gluten",
                        span { "{course_team_tel.clone().expect(\"Expect phone number of team\")}" }
                    }
                }
            }
            div { class: "font-thin flex items-start",
                div { class: "my-2", AddressSVG {} }
                div { class: "mx-0  text-[#543D2B] font-gluten leading-tight",
                    if address_split.len() == 2 {
                        span { "{address_split[0]}" }
                        div {}
                        span { "{address_split[1]}" }
                    } else {
                        span { "{address}" }
                    }
                }
            
            }
        )
    };
    rsx!(
        div { class: "text-[#543D2B] font-gluten group w-(--item-width) relative h-(--item-height) odd:self-end even:justify-end",
            div { class: "absolute h-[calc(100%-var(--item-margin)*2)] inset-y-0 -inset-x-(--item-overflow) m-(--item-margin) flex flex-col items-center justify-center",
                span { class: "font-bold", "{course_time} - {course_name}" }
                div {
                    if course_team_name.is_some() {
                        div { class: "font-thin flex items-start",
                            PersonSVG {}
                            div { class: "mx-0   text-[#543D2B] font-gluten",
                                span { "{course_team_name.clone().expect(\"Expect name of team\")}" }
                            }
                        }
                    }
                    {content}
                }
            }
            div { class: "absolute group-even:-bottom-2.5 group-odd:-top-2.5 left-1/2 -translate-x-1/2 w-4 h-4 {point_format}" }
        }
    )
}

#[component]
fn MyHosting(hosting: Hosting) -> Element {
    rsx!(
        div { class: "flex items-center my-4",
            div { class: "flex-grow h-1 bg-[#C66741]" }
            span { class: "mx-4 text-xl font-gluten text-[#543D2B]", "Guests" }
            div { class: "flex-grow h-1 bg-[#C66741]" }
        }


        for guest in hosting.guest_list {
            // Guest
            div { class: "flex items-start",
                PersonSVG {}
                div { class: "mx-2 text-[#543D2B] font-gluten font-bold",
                    span { "({guest.members}) {guest.team_name}" }
                }
            }

            // Phone number
            div { class: "flex items-start mx-6",
                PhoneSVG {}
                div { class: "mx-2 text-[#543D2B] font-gluten",
                    span { "{guest.phone_number}" }
                }
            }

            // Diets
            if !guest.diets.is_empty() {
                div { class: "flex items-start mx-6",
                    WarningSVG {}
                    div { class: "mx-2 text-[#543D2B] font-gluten",
                        span { "{guest.diets.join(\", \")}" }
                    }
                }
            }
        }
    )
}

#[component]
fn MyInfo(contact: ContactData) -> Element {
    let diets_string = contact.diets.join(", ");
    rsx!(
        div { class: "flex items-center my-4",
            div { class: "flex-grow h-1 bg-[#C66741]" }
            span { class: "mx-4 text-xl font-gluten text-[#543D2B]", "You" }
            div { class: "flex-grow h-1 bg-[#C66741]" }
        }

        // Team
        div { class: "flex items-start",
            PersonSVG {}
            div { class: "mx-2 text-[#543D2B] font-gluten font-bold",
                span { "({contact.members}) {contact.team_name}" }
            }
        }

        // Mail
        div { class: "flex items-start",
            svg {
                class: "w-6 h-6",
                xmlns: "http://www.w3.org/2000/svg",
                fill: "none",
                view_box: "0 0 24 24",
                stroke: "#C66741",
                path {
                    stroke_linecap: "round",
                    stroke_linejoin: "round",
                    stroke_width: "2",
                    d: "M3 8l7.89 5.26a3 3 0 003.22 0L22 8M5 6h14a2 2 0 012 2v8a2 2 0 01-2 2H5a2 2 0 01-2-2V8a2 2 0 012-2z",
                }
            }

            div { class: "mx-2 text-[#543D2B] font-gluten",
                span { "{contact.mail}" }
            }
        
        }

        // Phone
        div { class: "flex items-start",
            PhoneSVG {}

            div { class: "mx-2 text-[#543D2B] font-gluten",
                span { "{contact.phone_number}" }
            }
        
        }

        // Address
        div { class: "flex items-start",
            AddressSVG {}
            div { class: "mx-2 text-[#543D2B] font-gluten",
                span { "{contact.address.address}" }
            }
        }


        // Allergies (optional)
        if !diets_string.is_empty() {
            div { class: "flex items-start",
                WarningSVG {}
                div { class: "mx-2 text-[#543D2B] font-gluten",
                    span { "{diets_string}" }
                }
            }
        }
        div { class: "relative flex items-start",
            img {
                src: LEAF_1,
                class: "absolute mx-70 my-4 bottom-1 w-18 h-18 rotate-50",
            }
        }
    )
}

/*
// Share Button
            button {
                class: "bg-green-600 hover:bg-green-700 text-white p-3 rounded-full shadow-lg",
                onclick: |_| {},
                svg {
                    class: "w-6 h-6",
                    fill: "none",
                    stroke: "currentColor",
                    stroke_width: "2",
                    stroke_linecap: "round",
                    stroke_linejoin: "round",
                    view_box: "0 0 24 24",
                    xmlns: "http://www.w3.org/2000/svg",
                    // Linien
                    path { d: "M16 5l-8 5v4l8 5" }
                    // Kreise (als Punkte)
                    circle { cx: "16", cy: "5", r: "2" }
                    circle { cx: "6", cy: "12", r: "2" }
                    circle { cx: "16", cy: "19", r: "2" }
                }
            }
*/
