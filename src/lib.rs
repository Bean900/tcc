pub mod calculator;
pub mod contact;

use crate::contact::{Contact, ContactLoader};

use crate::calculator::Calculator;

use iced::window::{self, icon, Icon};
use iced::{
    widget::{
        button, column, container, horizontal_space, row, scrollable, text, text_input, Column, Row,
    },
    Element, Length, Theme,
};

use image::ImageReader;
use std::path::Path;

use iced::{Fill, Size};

use chrono::Local;
use env_logger::Builder;
use log::LevelFilter;
use std::io::Write;

const TCC: &str = "Traveling Cook Calculator";

pub fn startup() -> iced::Result {
    Builder::new()
        .format(|buf, record| {
            writeln!(
                buf,
                "{} [{}] - {}",
                Local::now().format("%Y-%m-%dT%H:%M:%S"),
                record.level(),
                record.args()
            )
        })
        .filter(None, LevelFilter::Warn)
        .init();

    let icon_path = "icon.png";

    let icon = load_icon(icon_path).expect("Failed to load icon");

    let window = window::Settings {
        size: Size::new(800_f32, 600_f32),
        icon: Some(icon),
        ..window::Settings::default()
    };

    iced::application(TCC, TCCScreen::update, TCCScreen::view)
        .theme(TCCScreen::theme)
        .window(window)
        .centered()
        .run()
}

pub struct TCCScreen<'a> {
    screen: Screen,
    err_message: String,
    contact_list: Option<Vec<contact::Contact>>,
    start_point_latitude: i32,
    start_point_longitude: i32,
    goal_point_latitude: i32,
    goal_point_longitude: i32,
    course_name_list: Vec<String>,
    calculator: Option<calculator::Calculator<'a, 'a>>,
}

#[derive(Debug, Clone)]
pub enum Message {
    LoadData,
    CreatePreview,
    GoToLoadDataScreen,
    GoToCheckDataScreen,
    GoToAddRulesScreen,
    GoToCalculateScreen,
    UpdateStartPointLatitude(String),
    UpdateStartPointLongitude(String),
    UpdateGoalPointLatitude(String),
    UpdateGoalPointLongitude(String),
    UpdateCourseNameList(usize, String),
    DeleteCourseName(usize),
}

impl<'a> TCCScreen<'a> {
    fn update(&mut self, event: Message) {
        match event {
            Message::LoadData => {
                let contact_loader = contact::ContactLoader::new();

                let contact_list_result = contact_loader.load();

                if contact_list_result.is_err() == true {
                    self.err_message = contact_list_result.unwrap_err().to_string();
                    return;
                }

                self.err_message = "".to_string();

                let contact_list = contact_list_result.unwrap();

                if contact_list.is_none() {
                    return;
                }

                self.contact_list = Some(contact_list.unwrap());
                self.screen = Screen::CheckData;
            }
            Message::CreatePreview => {
                // TODO
            }
            Message::GoToLoadDataScreen => {
                self.screen = Screen::LoadData;
            }
            Message::GoToCheckDataScreen => {
                self.screen = Screen::CheckData;
            }
            Message::GoToAddRulesScreen => {
                self.screen = Screen::AddRules;
            }
            Message::GoToCalculateScreen => {
                self.course_name_list
                    .truncate(self.course_name_list.len() - 1);

                let contact_list = self.contact_list.as_ref().unwrap();
                let calculator = Calculator::new(&self.course_name_list, contact_list);

                calculator.calculate();

                // self.calculator = Some(calculator);
                self.screen = Screen::Calculate;
            }
            Message::UpdateStartPointLatitude(content) => {
                let number = content.parse::<i32>();
                if number.is_err() {
                    self.err_message =
                        "Format of start point latitude is not a number!".to_string();
                } else {
                    self.start_point_latitude = number.unwrap();
                }
            }
            Message::UpdateStartPointLongitude(content) => {
                let number = content.parse::<i32>();
                if number.is_err() {
                    self.err_message =
                        "Format of start point longitude is not a number!".to_string();
                } else {
                    self.start_point_longitude = number.unwrap();
                }
            }
            Message::UpdateGoalPointLatitude(content) => {
                let number = content.parse::<i32>();
                if number.is_err() {
                    self.err_message = "Format of goal point latitude is not a number!".to_string();
                } else {
                    self.goal_point_latitude = number.unwrap();
                }
            }
            Message::UpdateGoalPointLongitude(content) => {
                let number = content.parse::<i32>();
                if number.is_err() {
                    self.err_message =
                        "Format of goal point longitude is not a number!".to_string();
                } else {
                    self.goal_point_longitude = number.unwrap();
                }
            }
            Message::UpdateCourseNameList(index, value) => {
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
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let screen = match self.screen {
            Screen::LoadData => self.load_data(),
            Screen::CheckData => self.check_contact(),
            Screen::AddRules => self.add_rule(),
            Screen::Calculate => self.calculate(),
        };

        let progress = self.progress();

        let error_message = column![text(self.err_message.clone())];

        let content: Element<_> = column![screen, progress, error_message]
            .max_width(540)
            .spacing(20)
            .padding(20)
            .into();

        container(content).center_y(Fill).into()
    }

    fn progress(&self) -> Row<Message> {
        match self.screen {
            Screen::LoadData => row![
                button("Load Data"),
                button("Check Data").style(button::secondary),
                button("Add Rules").style(button::secondary),
                button("Calculate").style(button::secondary),
                button("Check Results").style(button::secondary)
            ],
            Screen::CheckData => row![
                button("Load Data")
                    .style(button::primary)
                    .on_press(Message::GoToLoadDataScreen),
                button("Check Data"),
                button("Add Rules").style(button::secondary),
                button("Calculate").style(button::secondary),
                button("Check Results").style(button::secondary)
            ],
            Screen::AddRules => row![
                button("Load Data")
                    .style(button::primary)
                    .on_press(Message::GoToLoadDataScreen),
                button("Check Data")
                    .style(button::primary)
                    .on_press(Message::GoToCheckDataScreen),
                button("Add Rules"),
                button("Calculate").style(button::secondary),
                button("Check Results").style(button::secondary)
            ],
            Screen::Calculate => row![
                button("Load Data")
                    .style(button::primary)
                    .on_press(Message::GoToLoadDataScreen),
                button("Check Data")
                    .style(button::primary)
                    .on_press(Message::GoToCheckDataScreen),
                button("Add Rules")
                    .style(button::primary)
                    .on_press(Message::GoToAddRulesScreen),
                button("Calculate"),
                button("Check Results").style(button::secondary)
            ],
        }
    }

    fn check_contact(&self) -> Column<Message> {
        let table = column![
            horizontal_space(),
            row!["Team-name", "Address", "Latitude", "Longitude"].spacing(20)
        ]
        .push(iced::widget::Column::with_children(
            self.contact_list
                .as_ref()
                .unwrap()
                .iter()
                .map(|contact| {
                    row![
                        text(contact.team_name.clone()),
                        text(contact.address.clone()),
                        text(contact.latitude.clone()),
                        text(contact.longitude.clone()),
                    ]
                    .spacing(20)
                    .into()
                })
                .collect::<Vec<_>>(),
        ));

        column![scrollable(table)]
            .width(Length::Fixed(450.0))
            .height(Length::Fixed(450.0))
            .padding(40)
            .push(button("Next").on_press(Message::GoToAddRulesScreen))
    }

    fn load_data(&self) -> Column<Message> {
        column!["Welcome"]
            .push("The traveling cook calculator helps with planning routes for cook and run.")
            .push(button("Import contact data").on_press(Message::LoadData))
            .push(button("Create example file"))
    }

    fn theme(&self) -> Theme {
        Theme::Dracula
    }

    fn add_rule(&self) -> Column<Message> {
        let mut course_name_rows: Column<Message> = column![];
        for (i, el) in self.course_name_list.iter().enumerate() {
            course_name_rows = course_name_rows.push(row!(
                text_input("Course name", el)
                    .on_input(move |content| Message::UpdateCourseNameList(i, content)),
                button("-").on_press(Message::DeleteCourseName(i))
            ));
        }

        return column![row![
            "Start point (optional):",
            column![
                text_input("latitude", &self.start_point_latitude.to_string())
                    .on_input(Message::UpdateStartPointLatitude),
                text_input("longitude", &self.start_point_longitude.to_string())
                    .on_input(Message::UpdateStartPointLongitude),
            ]
        ]]
        .push(row![
            "Goal point (optional):",
            column![
                text_input("latitude", &self.goal_point_latitude.to_string())
                    .on_input(Message::UpdateGoalPointLatitude),
                text_input("longitude", &self.goal_point_longitude.to_string())
                    .on_input(Message::UpdateGoalPointLongitude)
            ]
        ])
        .push(course_name_rows)
        .push(button("Next").on_press(Message::GoToCalculateScreen));
    }

    fn calculate(&self) -> Column<Message> {
        let binding = self.calculator.as_ref().unwrap();
        let top_score = binding.top_score.lock().unwrap();
        let score = top_score.score.unwrap_or_else(|| 0.0);
        let score_string = format!("{:.2}", score);
        let score_test = "Score:";
        column![
            row![column![score_test, text(score_string)]],
            button("STOP")
        ]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Screen {
    LoadData,
    CheckData,
    AddRules,
    Calculate,
}

impl Screen {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Layout {
    Row,
    Column,
}

impl<'a> Default for TCCScreen<'a> {
    fn default() -> Self {
        Self {
            screen: Screen::LoadData,
            err_message: "".to_string(),
            contact_list: None,
            start_point_latitude: 0_i32,
            start_point_longitude: 0_i32,
            goal_point_latitude: 0_i32,
            goal_point_longitude: 0_i32,
            course_name_list: vec!["".to_string()],
            calculator: None,
        }
    }
}

fn load_icon<P: AsRef<Path>>(path: P) -> Result<Icon, String> {
    let image = ImageReader::open(path)
        .map_err(|_| "Bild konnte nicht geöffnet werden".to_string())?
        .decode()
        .map_err(|_| "Bild konnte nicht dekodiert werden".to_string())?;
    let image = image.to_rgba8();
    let (width, height) = image.dimensions();
    let rgba = image.into_raw();

    icon::from_rgba(rgba, width, height).map_err(|e| e.to_string())
}
