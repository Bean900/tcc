use iced::{
    alignment::{
        Horizontal::{Left, Right},
        Vertical,
    },
    border::Radius,
    widget::{checkbox, container, text, text_input::Style},
    Border, Color, Element,
    Length::Fill,
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
        let headline = text("Cook And Run Courses and Start / Goal point")
            .size(25)
            .align_x(Left);

        let button_next =
            container(button("Next Step").on_press(Message::GoToCalculateScreen)).align_right(Fill);

        column![
            row![headline, button_next],
            row![
                self.get_course_name(),
                container(
                    container(
                        column![
                            self.get_start_point_element(),
                            self.get_goal_point_element()
                        ]
                        .spacing(10)
                    )
                    .width(140)
                    .height(200)
                    .max_width(140)
                    .max_height(200)
                    .padding(10)
                    .style(move |_| container::Style {
                        border: Border {
                            color: Color::from_rgb(0.9, 0.9, 0.9),
                            radius: Radius {
                                top_left: 12.0,
                                top_right: 12.0,
                                bottom_right: 12.0,
                                bottom_left: 12.0,
                            },
                            width: 1.0,
                        },
                        ..Default::default()
                    })
                )
                .padding(20)
                .align_x(Right)
            ]
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
            Message::UpdateCourseNameList(index, value) => {
                self.course_name_list[index] = value;
                let len = self.course_name_list.len();
                if len > 1 && self.course_name_list[len - 1].is_empty() {
                    self.course_name_list.remove(len - 1);
                }
            }
            Message::DeleteCourseName(index) => {
                self.course_name_list.remove(index);
            }
            Message::AddCourseName(value) => {
                self.course_name_list.push(value);
            }
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
        let course_name_headline = text("Course names:").size(20);

        let mut course_name_rows: Column<Message> = column![];
        for (i, el) in self.course_name_list.iter().enumerate() {
            course_name_rows = course_name_rows.push(row![
                text(format!("{}: ", i + 1))
                    .align_y(Vertical::Center)
                    .width(15),
                //.height(Fill),
                text_input("Type course name here...", el)
                    .on_input(move |content| Message::UpdateCourseNameList(i, content)),
                button("-").on_press(Message::DeleteCourseName(i))
            ]);
        }
        let len = self.course_name_list.len();

        if len < 7 {
            course_name_rows = course_name_rows.push(row![
                text(format!("{}: ", len + 1))
                    .align_y(Vertical::Center)
                    .width(15),
                text_input("Type course name here...", "")
                    .on_input(move |content| Message::AddCourseName(content)),
                button("-")
            ]);
        }
        container(column![course_name_headline, course_name_rows])
            .padding(20)
            .into()
    }

    fn get_start_point_element(&self) -> Element<Message> {
        let headline = checkbox("Start point:", self.start_point_checkbox_state)
            .on_toggle(Message::ShowStartPositionInputField);

        let latitude = get_input_box(
            &self.start_point,
            FieldName::Latitude,
            self.start_point_checkbox_state
                .then(|| Message::CheckInputCoordinateStartPointLatitude),
        );
        let longitude = get_input_box(
            &self.start_point,
            FieldName::Longitude,
            self.start_point_checkbox_state
                .then(|| Message::CheckIbputCoordinateStartPointLongitude),
        );

        container(column![headline, latitude, longitude]).into()
    }

    fn get_goal_point_element(&self) -> Element<Message> {
        let headline = checkbox("Goal point:", self.goal_point_checkbox_state)
            .on_toggle(Message::ShowGoalPositionInputField);
        let latitude = get_input_box(
            &self.goal_point,
            FieldName::Latitude,
            self.goal_point_checkbox_state
                .then(|| Message::CheckInputCoordinateGoalPointLatitude),
        );
        let longitude = get_input_box(
            &self.goal_point,
            FieldName::Longitude,
            self.goal_point_checkbox_state
                .then(|| Message::CheckInputCoordinateGoalPointLongitude),
        );

        container(column![headline, latitude, longitude]).into()
    }

    pub fn get_course_name_list(&self) -> Option<Vec<String>> {
        if self.course_name_list.is_empty() {
            None
        } else {
            Some(self.course_name_list.clone())
        }
    }

    pub fn get_start_point(&self) -> Option<(i32, i32)> {
        self.start_point.as_ref().and_then(|start_point| {
            if self.start_point_checkbox_state
                && start_point.latitude.is_some()
                && start_point.longitude.is_some()
            {
                Some((
                    start_point.latitude.as_ref()?.parse::<i32>().ok()?,
                    start_point.longitude.as_ref()?.parse::<i32>().ok()?,
                ))
            } else {
                None
            }
        })
    }

    pub fn get_goal_point(&self) -> Option<(i32, i32)> {
        self.goal_point.as_ref().and_then(|goal_point| {
            if self.goal_point_checkbox_state
                && goal_point.latitude.is_some()
                && goal_point.longitude.is_some()
            {
                Some((
                    goal_point.latitude.as_ref()?.parse::<i32>().ok()?,
                    goal_point.longitude.as_ref()?.parse::<i32>().ok()?,
                ))
            } else {
                None
            }
        })
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
    on_input: Option<impl Fn(String) -> Message + 'static>,
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

    let text_input = position_option.as_ref().clone().map_or_else(
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
    );

    if on_input.is_some() {
        text_input.on_input(on_input.unwrap()).into()
    } else {
        text_input.into()
    }
}
