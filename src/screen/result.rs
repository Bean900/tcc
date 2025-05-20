use std::sync::atomic::Ordering;

use iced::{
    alignment::{
        Horizontal::{self, Left},
        Vertical,
    },
    border::Radius,
    widget::{
        button, column, container, row,
        scrollable::{self, Direction, Scrollbar},
        text, Button, Column, Row, Scrollable, Text,
    },
    Alignment::Center,
    Border, Color, Element,
    Length::{self, Fill},
};

use crate::{
    calculator::{self, Calculator, CalculatorConfig, Course, Plan},
    contact::{self, Contact},
    image_collection::IMAGE_COLLECTION,
    Message,
};

use super::Screen;

pub(crate) struct ResultScreen {
    plan: Option<Plan>,
    err_message: Option<String>,
}

impl Screen for ResultScreen {
    fn get(&self) -> Element<Message> {
        if self.plan.is_none() {
            return text("No plan available").size(50).into();
        }
        let headline = Text::new("Cook And Run Teams").size(25).align_x(Left);

        let plan = self.plan.as_ref().expect("Expected plan");

        let mut row_element = Row::new();

        for (contact, walkin_path) in plan.walking_path.iter() {
            row_element = row_element.push(get_walking_path_element(contact, walkin_path));
        }

        let scrollbar =
            Scrollable::with_direction(row_element, Direction::Horizontal(Scrollbar::default()))
                .height(Fill);

        column![headline, scrollbar].into()
    }
    fn update(&mut self, event: Message) {
        match event {
            _ => {}
        }
    }
}

fn get_walking_path_element<'a>(
    contact: &'a Contact,
    walkin_path: &'a Vec<Course>,
) -> Element<'a, Message> {
    println!("{}:", contact.team_name);
    let team_name = text!("{}", contact.team_name.clone()).size(20);

    let mut element = column![].padding(10);

    for current_course in walkin_path.iter() {
        println!(
            "- {}:\t{}",
            current_course.name, current_course.host.team_name
        );

        // let is_own_course = current_course.host.eq(contact);
        let course_name = text!("{}", current_course.name.clone()).size(18);
        let contact_name = text!("{}", current_course.host.team_name.clone()).size(15);
        element = element.push(column![course_name, contact_name]);
    }
    println!("");

    column![team_name, element].into()
}

impl ResultScreen {
    pub fn new() -> Self {
        ResultScreen {
            plan: None,
            err_message: None,
        }
    }

    pub fn set_plan(&mut self, plan: Option<Plan>) {
        self.plan = plan;
    }
}
