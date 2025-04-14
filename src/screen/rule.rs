use std::{any, rc::Rc};

use env_logger::fmt::style;
use iced::{
    border::Radius,
    widget::{
        checkbox, container, text,
        text_input::{default, Style},
    },
    Border, Color, Element,
};

use crate::Message;
use iced::widget::{button, column, row, text_input, Column};

use super::Screen;

#[derive(Debug, Clone)]
struct Position {
    latitude: Option<String>,
    latitude_err: bool,
    longitude: Option<String>,
    longitude_err: bool,
}

#[derive(Debug, Clone, PartialEq)]
enum FieldName {
    Latitude,
    Longitude,
}

impl Position {
    fn new() -> Self {
        Position {
            latitude: None,
            latitude_err: false,
            longitude: None,
            longitude_err: false,
        }
    }
}

pub(crate) struct RuleScreen {
    course_name_list: Vec<String>,
    start_point: Option<Position>,
    goal_point: Option<Position>,
    start_point_checkbox_state: bool,
    goal_point_checkbox_state: bool,
}

impl Screen for RuleScreen {
    fn get(&self) -> Element<Message> {
        column![
            self.get_course_name(),
            row![self.get_start_point(), self.get_goal_point()]
        ]
        .into()
    }
    fn update(&mut self, event: Message) {
        match event {
            Message::ShowStartPositionInputField(state) => {
                self.start_point_checkbox_state = state;
            }
            Message::ShowGoalPositionInputField(state) => {
                self.goal_point_checkbox_state = state;
            }

            Message::CheckInputCoordinateStartPointLatitude(content) => {
                set_position_data(&mut self.start_point, content, FieldName::Latitude);
            }
            Message::CheckIbputCoordinateStartPointLongitude(content) => {
                set_position_data(&mut self.start_point, content, FieldName::Longitude);
            }
            Message::CheckInputCoordinateGoalPointLatitude(content) => {
                set_position_data(&mut self.goal_point, content, FieldName::Latitude);
            }
            Message::CheckInputCoordinateGoalPointLongitude(content) => {
                set_position_data(&mut self.goal_point, content, FieldName::Longitude);
            }
            /*   Message::UpdateCourseNameList(index, value) => {
                self.course_name_list[index] = value;
                let len = self.course_name_list.len();
                if len > 1 && self.course_name_list[len - 1].is_empty() {
                    self.course_name_list.remove(len - 1);
                }

                if self.course_name_list.is_empty()
                    || !self.course_name_list[self.course_name_list.len() - 1].is_empty()
                {
                    self.course_name_list.push("".to_string());
                }
            }
            Message::DeleteCourseName(index) => {
                self.course_name_list.remove(index);
                if self.course_name_list.is_empty()
                    || !self.course_name_list[self.course_name_list.len() - 1].is_empty()
                {
                    self.course_name_list.push("".to_string());
                }
            }*/
            _ => {}
        }
    }
}

impl RuleScreen {
    pub fn new() -> Self {
        RuleScreen {
            course_name_list: vec![],
            start_point: None,
            goal_point: None,
            start_point_checkbox_state: false,
            goal_point_checkbox_state: false,
        }
    }

    fn get_course_name(&self) -> Element<Message> {
        let course_name_headline = text("Course names:");

        let mut course_name_rows: Column<Message> = column![];
        for (i, el) in self.course_name_list.iter().enumerate() {
            course_name_rows = course_name_rows.push(row![
                text(format!("{}: ", i)),
                text_input("Type course name here...", el)
                    .on_input(move |content| Message::UpdateCourseNameList(i, content)),
                button("-").on_press(Message::DeleteCourseName(i))
            ]);
        }
        container(row![course_name_headline, course_name_rows]).into()
    }

    fn get_start_point(&self) -> Element<Message> {
        let headline = checkbox("Start point:", self.start_point_checkbox_state)
            .on_toggle(Message::ShowStartPositionInputField);

        if self.start_point_checkbox_state {
            let latitude = get_input_box(
                &self.start_point,
                FieldName::Latitude,
                Message::CheckInputCoordinateStartPointLatitude,
            );
            let longitude = get_input_box(
                &self.start_point,
                FieldName::Longitude,
                Message::CheckIbputCoordinateStartPointLongitude,
            );

            container(row![headline, latitude, longitude]).into()
        } else {
            container(row![headline]).into()
        }
    }

    fn get_goal_point(&self) -> Element<Message> {
        let headline = checkbox("Goal point:", self.goal_point_checkbox_state)
            .on_toggle(Message::ShowGoalPositionInputField);
        if self.goal_point_checkbox_state {
            let latitude = get_input_box(
                &self.goal_point,
                FieldName::Latitude,
                Message::CheckInputCoordinateGoalPointLatitude,
            );
            let longitude = get_input_box(
                &self.goal_point,
                FieldName::Longitude,
                Message::CheckInputCoordinateGoalPointLongitude,
            );

            container(row![headline, latitude, longitude]).into()
        } else {
            container(row![headline]).into()
        }
    }
}

fn set_position_data(
    position_option: &mut Option<Position>,
    content: String,
    field_name: FieldName,
) {
    let position: &mut Position = match position_option.as_mut() {
        Some(position) => position,
        None => {
            position_option.get_or_insert_with(|| Position::new());
            position_option.as_mut().expect("Expect  point to be set!")
        }
    };
    let _ = string_to_number::<f64>(content.as_str())
        .map(|number| {
            if field_name == FieldName::Latitude {
                position.latitude = Some(number.to_string());
                position.latitude_err = false
            } else {
                position.longitude = Some(number.to_string());
                position.longitude_err = false
            }
        })
        .map_err(|err_number| {
            if field_name == FieldName::Latitude {
                position.latitude = Some(err_number);
                position.latitude_err = true
            } else {
                position.longitude = Some(err_number);
                position.longitude_err = true
            }
        });
}

fn string_to_number<T: std::str::FromStr>(content: &str) -> Result<T, String> {
    let normalized_content = content.replace(',', ".");
    let mut new_content = String::new();
    let mut has_decimal = false;

    for c in normalized_content.chars() {
        if c.is_digit(10) {
            new_content.push(c);
        } else if (c == '.') && !has_decimal {
            has_decimal = true;
            new_content.push(c);
        }
    }

    match new_content.parse::<T>() {
        Ok(number) => Ok(number),
        Err(_) => Err(new_content),
    }
}

fn get_input_box(
    position_option: &Option<Position>,
    field_name: FieldName,
    on_input: impl Fn(String) -> Message + 'static,
) -> Element<Message> {
    let error_style = move |theme, status| Style {
        border: Border {
            color: Color::from_rgb(0.8, 0.8, 0.8),
            radius: Radius {
                top_left: 2.0,
                top_right: 2.0,
                bottom_right: 2.0,
                bottom_left: 2.0,
            },
            width: 1.0,
        },
        ..text_input::default(theme, status)
    };
    let normale_style = move |theme, status| Style {
        ..text_input::default(theme, status)
    };

    position_option
        .as_ref()
        .clone()
        .map_or_else(
            || text_input("Type coordinate here", ""),
            |position| match field_name {
                FieldName::Latitude => position.latitude.as_ref().map_or_else(
                    || text_input("Type coordinate here", ""),
                    |coordinate| text_input("Type coordinate here", coordinate),
                ),
                FieldName::Longitude => position.longitude.as_ref().map_or_else(
                    || text_input("Type coordinate here", ""),
                    |coordinate| text_input("Type coordinate here", coordinate),
                ),
            },
        )
        .on_input(on_input)
        .into()
}
