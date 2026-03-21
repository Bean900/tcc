use chrono::NaiveDate;
use dioxus::prelude::*;
use pulldown_cmark::{html, Parser};

use crate::{
    side::{
        details::run_schedule::{Course, MeetingPoint, Schedule, Team},
        AddressSVG, PersonSVG, PhoneSVG, StartSVG, WarningSVG,
    },
    storage::{Language, PlanConfigData},
};

enum TextModule {
    You,
    Guests,
    Timeline,
    DietaryRestrictions,
}

impl TextModule {
    fn get_text(&self, language: &Language) -> &str {
        match (self, language) {
            (TextModule::You, Language::English) => "You",
            (TextModule::You, Language::German) => "Du",
            (TextModule::Guests, Language::English) => "Guests",
            (TextModule::Guests, Language::German) => "Gäste",
            (TextModule::Timeline, Language::English) => "Timeline",
            (TextModule::Timeline, Language::German) => "Zeitplan",
            (TextModule::DietaryRestrictions, Language::English) => "No restrictions",
            (TextModule::DietaryRestrictions, Language::German) => "Keine Einschränkungen",
        }
    }
}

const POT: Asset = asset!("/assets/pot.png");
const SPATULA: Asset = asset!("/assets/spatula.png");
const LEAF_1: Asset = asset!("/assets/leaf_1.png");
const LEAF_2: Asset = asset!("/assets/leaf_2.png");
const CAKE: Asset = asset!("/assets/cake.png");
const CARROT: Asset = asset!("/assets/carrot.png");

pub async fn download(plan_title: String, team_name: String) {
    let js = format!(
        r#"
        const el = document.getElementById('section-to-print');
        htmlToImage.toPng(el).then(dataUrl => {{
            const link = document.createElement('a');
            link.download = '{plan_title}_{team_name}.png';
            link.href = dataUrl;
            link.click();
        }});
        "#
    );
    let _ = document::eval(&js).await;
}

#[component]
pub fn RunSchedule(plan_config: PlanConfigData, schedule: Schedule) -> Element {
    let titel_html = {
        let text = plan_config.title.as_str();
        let parser = Parser::new(text);
        let mut html_output = String::new();
        html::push_html(&mut html_output, parser);
        html_output
    };

    let description_html = {
        let text = plan_config.description.as_str();
        let parser = Parser::new(text);
        let mut html_output = String::new();
        html::push_html(&mut html_output, parser);
        html_output
    };

    rsx!(
        document::Script {
            src: "https://unpkg.com/html-to-image@1.11.11/dist/html-to-image.js"
        }
        div {
            id: "section-to-print",
            class: "flex justify-center w-full h-full bg-[#F8EFE1]",
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
                        dangerous_inner_html:titel_html
                    }
                }

                p { class: "font-gluten text-[#543D2B] mt-2", dangerous_inner_html:description_html }

                // MyInfo
                div { class: "grid grid-cols1 md:grid-cols-2 md:gap-x-4",
                    div {
                        MyInfo {    language: plan_config.language.clone(),team: schedule.host }
                    }
                    div {
                        MyHosting {   language: plan_config.language.clone(), guest_list: schedule.guest_list }
                    }
                }

                TimeLine {
                    language: plan_config.language.clone(),
                    cook_and_run_date: plan_config.date,
                    start_point: schedule.start_point,
                    end_point: schedule.end_point,
                    walking_path: schedule.walking_path,
                }
            }
        }
    )
}

#[component]
fn TimeLine(
    language: Language,
    cook_and_run_date: NaiveDate,
    start_point: Option<MeetingPoint>,
    end_point: Option<MeetingPoint>,
    walking_path: Vec<(Course, Team)>,
) -> Element {
    let headline = format!(
        "{} - {}",
        TextModule::Timeline.get_text(&language),
        cook_and_run_date
    );

    let start_name = start_point.as_ref().map_or("Start", |s| s.name.as_str());
    let start_time = start_point.as_ref().map_or("", |s| s.time.as_str());
    let start_addr = start_point.as_ref().map_or("", |s| s.address.as_str());

    let end_name = end_point.as_ref().map_or("End", |e| e.name.as_str());
    let end_time = end_point.as_ref().map_or("", |e| e.time.as_str());
    let end_addr = end_point.as_ref().map_or("", |e| e.address.as_str());

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
                for (i , (course , host)) in walking_path.iter().enumerate() {
                    TimeLineElement {
                        is_up: i % 2 != 0,
                        course_name: course.name.clone(),
                        course_team_name: Some(host.name.clone()),
                        course_team_tel: host.phone.clone(),
                        course_time: &course.time,
                        address: host.address.clone(),
                        you_are_hosting: false,
                    }
                }
                if end_point.is_some() {
                    if walking_path.len() % 2 != 0 {
                        TimeLineElement {
                            is_up: false,
                            course_name: end_name,
                            course_time: end_time,
                            address: end_addr,
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
    rsx!(div {
        class: "flex-1 text-center items-center"
    })
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
fn MyHosting(language: Language, guest_list: Vec<Team>) -> Element {
    rsx!(
        div { class: "flex items-center my-4",
            div { class: "flex-grow h-1 bg-[#C66741]" }
            span { class: "mx-4 text-xl font-gluten text-[#543D2B]", "{TextModule::Guests.get_text(&language)}" }
            div { class: "flex-grow h-1 bg-[#C66741]" }
        }

        for guest in guest_list.iter() {
            // Guest
            div { class: "flex items-start",
                PersonSVG {}
                div { class: "mx-2 text-[#543D2B] font-gluten font-bold",
                    span { "{guest.name}" }
                }
            }

            // Phone number
            if let Some(phone) = &guest.phone {
                div { class: "flex items-start mx-6",
                    PhoneSVG {}
                    div { class: "mx-2 text-[#543D2B] font-gluten",
                        span { "{phone}" }
                    }
                }
            }

            // Diets
            div { class: "flex items-start mx-6",
                WarningSVG {}
                div { class: "mx-2 text-[#543D2B] font-gluten",
                    if let Some(diets) = &guest.diets {
                        span { "{diets}" }
                    } else {
                        span { "No dietary restrictions" }
                    }
                }
            }
        }
    )
}

#[component]
fn MyInfo(language: Language, team: Team) -> Element {
    rsx!(
        div { class: "flex items-center my-4",
            div { class: "flex-grow h-1 bg-[#C66741]" }
            span { class: "mx-4 text-xl font-gluten text-[#543D2B]", "{TextModule::You.get_text(&language)}" }
            div { class: "flex-grow h-1 bg-[#C66741]" }
        }

        // Team
        div { class: "flex items-start",
            PersonSVG {}
            div { class: "mx-2 text-[#543D2B] font-gluten font-bold",
                span { "{team.name}" }
            }
        }

        // Mail
        if let Some(mail) = &team.mail {
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
                    span { "{mail}" }
                }
            }
        }

        // Phone
        if let Some(phone) = &team.phone {
            div { class: "flex items-start",
                PhoneSVG {}

                div { class: "mx-2 text-[#543D2B] font-gluten",
                    span { "{phone}" }
                }
            }
        }

        // Address
        div { class: "flex items-start",
            AddressSVG {}
            div { class: "mx-2 text-[#543D2B] font-gluten",
                span { "{team.address}" }
            }
        }

        // Allergies (optional)
        if let Some(diets) = &team.diets {
            div { class: "flex items-start",
                WarningSVG {}
                div { class: "mx-2 text-[#543D2B] font-gluten",
                    span { "{diets}" }
                }
            }
        } else {
            div { class: "flex items-start",
                WarningSVG {}
                div { class: "mx-2 text-[#543D2B] font-gluten",
                    span { "No dietary restrictions" }
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
