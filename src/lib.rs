pub mod calculator;
pub mod contact;
pub mod screen;

use crate::calculator::Calculator;

use calculator::CalculatorConfig;
use contact::Contact;
use iced::widget::svg::Handle;
use iced::widget::{button, horizontal_space, svg, text, text_input, Svg};
use iced::window::{self, icon, Icon};

use iced::{
    alignment::{Horizontal, Vertical},
    border::Radius,
    widget::{column, container, row, scrollable, Button, Column, Row, Text},
    Alignment::Center,
    Border, Color, Element,
    Length::{self, Fill},
};
use image::ImageReader;
use std::path::Path;
use std::rc::Rc;

use crate::screen::{load::LoadScreen, AvailableScreens, ScreenName};
use iced::{Alignment, Size, Theme};

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
        size: Size::new(600_f32, 400_f32),
        min_size: Some(Size::new(600_f32, 400_f32)), // Minimale Größe des Fensters
        icon: Some(icon),
        ..window::Settings::default()
    };

    iced::application(TCC, TCCScreen::update, TCCScreen::view)
        .theme(TCCScreen::theme)
        .window(window)
        .run()
}

pub struct TCCScreen {
    screen: AvailableScreens,
    active_screen: ScreenName,
    err_message: String,
    contact_list: Option<Rc<Vec<Contact>>>,
    start_point_latitude: i32,
    start_point_longitude: i32,
    goal_point_latitude: i32,
    goal_point_longitude: i32,
    course_name_list: Vec<String>,
    calculator: Option<Calculator>,
    image_collection: ImageCollection,
}

struct ImageCollection {
    upload: SvgImage,
    add_course: SvgImage,
    calc: SvgImage,
    result: SvgImage,
    line: SvgImage,
    next_line: SvgImage,
}

struct SvgImage {
    selected: Handle,
    next: Handle,
    previous: Handle,
}

#[derive(Debug, Clone)]
pub enum Message {
    LoadData,
    CreatePreview,
    GoToLoadDataScreen,
    GoToAddRulesScreen,
    GoToCalculateScreen,
    GoToResultScreen,
    UpdateStartPointLatitude(String),
    UpdateStartPointLongitude(String),
    UpdateGoalPointLatitude(String),
    UpdateGoalPointLongitude(String),
    UpdateCourseNameList(usize, String),
    DeleteCourseName(usize),
}

impl TCCScreen {
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

                let contact_list = contact_list_result.expect("Contact list should exist");

                if contact_list.is_none() {
                    return;
                }

                let contact_list = Rc::new(contact_list.expect("Contact list should exist"));
                self.screen.set_contact_list(contact_list);
                //  self.screen = Screen::CheckData;
            }
            Message::CreatePreview => {
                // TODO
            }
            Message::GoToLoadDataScreen => {
                self.active_screen = ScreenName::LoadData;
            }
            Message::GoToAddRulesScreen => {
                self.active_screen = ScreenName::AddRules;
            }
            Message::GoToCalculateScreen => {
                self.course_name_list
                    .truncate(self.course_name_list.len() - 1);

                let config = CalculatorConfig::new(
                    self.course_name_list.clone(),
                    self.contact_list
                        .clone()
                        .expect("Expect contact list to exist")
                        .clone(),
                    None,
                );
                let calculator = Calculator::new(config);

                calculator.calculate();

                // self.calculator = Some(calculator);
                self.active_screen = ScreenName::Calculate;
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
            Message::GoToResultScreen => todo!(),
        }
    }

    fn view(&self) -> Element<Message> {
        let screen = self.screen.get(self.active_screen);

        let progress = self.progress();

        //let error_message = column![text(self.err_message.clone())];

        let content: Element<_> = column![screen, progress]
            .height(Length::Fill)
            .width(Length::Fill)
            .align_x(Horizontal::Center)
            .padding(10)
            .into();

        content
    }

    fn progress(&self) -> Element<Message> {
        let button_style = button::Style {
            background: None,
            ..Default::default()
        };
        container(
            match self.active_screen {
                ScreenName::LoadData => row![
                    Button::new(self.image_collection.upload.get_selected())
                        //.on_press(Message::GoToLoadDataScreen)
                        .style(move |_, _| button_style),
                    self.image_collection.line.get_next(),
                    Button::new(self.image_collection.add_course.get_next())
                        //.on_press(Message::GoToAddRulesScreen)
                        .style(move |_, _| button_style),
                    self.image_collection.line.get_next(),
                    Button::new(self.image_collection.calc.get_next())
                        //.on_press(Message::GoToCalculateScreen)
                        .style(move |_, _| button_style),
                    self.image_collection.next_line.get_next(),
                    Button::new(self.image_collection.result.get_next())
                        //.on_press(Message::GoToResultScreen)
                        .style(move |_, _| button_style),
                ],
                ScreenName::AddRules => row![
                    Button::new(self.image_collection.upload.get_previous())
                        .on_press(Message::GoToLoadDataScreen)
                        .style(move |_, _| button_style),
                    self.image_collection.line.get_previous(),
                    Button::new(self.image_collection.add_course.get_selected())
                        //.on_press(Message::GoToAddRulesScreen)
                        .style(move |_, _| button_style),
                    self.image_collection.line.get_next(),
                    Button::new(self.image_collection.calc.get_selected())
                        //.on_press(Message::GoToCalculateScreen)
                        .style(move |_, _| button_style),
                    self.image_collection.next_line.get_next(),
                    Button::new(self.image_collection.result.get_next())
                        //.on_press(Message::GoToResultScreen)
                        .style(move |_, _| button_style),
                ],
                ScreenName::Calculate => row![
                    Button::new(self.image_collection.upload.get_previous())
                        .on_press(Message::GoToLoadDataScreen)
                        .style(move |_, _| button_style),
                    self.image_collection.line.get_previous(),
                    Button::new(self.image_collection.add_course.get_previous())
                        .on_press(Message::GoToAddRulesScreen)
                        .style(move |_, _| button_style),
                    self.image_collection.line.get_previous(),
                    Button::new(self.image_collection.calc.get_selected())
                        //.on_press(Message::GoToCalculateScreen)
                        .style(move |_, _| button_style),
                    self.image_collection.next_line.get_next(),
                    Button::new(self.image_collection.result.get_next())
                        //.on_press(Message::GoToResultScreen)
                        .style(move |_, _| button_style),
                ],
                ScreenName::Result => row![
                    Button::new(self.image_collection.upload.get_previous())
                        .on_press(Message::GoToLoadDataScreen)
                        .style(move |_, _| button_style),
                    self.image_collection.line.get_previous(),
                    Button::new(self.image_collection.add_course.get_previous())
                        .on_press(Message::GoToAddRulesScreen)
                        .style(move |_, _| button_style),
                    self.image_collection.line.get_previous(),
                    Button::new(self.image_collection.calc.get_previous())
                        .on_press(Message::GoToCalculateScreen)
                        .style(move |_, _| button_style),
                    self.image_collection.next_line.get_previous(),
                    Button::new(self.image_collection.result.get_selected())
                        //.on_press(Message::GoToResultScreen)
                        .style(move |_, _| button_style),
                ],
            }
            .align_y(Alignment::Center)
            .spacing(10),
        )
        .width(Length::Fill)
        .height(Length::FillPortion(1))
        .max_width(128 * 5)
        .align_y(iced::alignment::Vertical::Bottom)
        .into()
    }

    fn check_contact(&self) -> Element<Message> {
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
            .into()
    }

    fn theme(&self) -> Theme {
        Theme::Light
    }

    fn add_rule(&self) -> Element<Message> {
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
        .push(button("Next").on_press(Message::GoToCalculateScreen))
        .into();
    }

    fn calculate(&self) -> Element<Message> {
        let binding = self.calculator.as_ref().unwrap();
        let top_score = binding.top_score.lock().unwrap();
        let score = top_score.score.unwrap_or_else(|| 0.0);
        let score_string = format!("{:.2}", score);
        let score_test = "Score:";
        column![
            row![column![score_test, text(score_string)]],
            button("STOP")
        ]
        .into()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Layout {
    Row,
    Column,
}

impl SvgImage {
    fn new(path: String) -> Self {
        Self {
            selected: Self::load(path.clone()),
            previous: Self::load(path.clone()),
            next: Self::load(path.clone()),
        }
    }

    fn get_style_previous(_: &Theme, status: svg::Status) -> svg::Style {
        let completed_step = Color::from_rgb(52.0 / 255.0, 152.0 / 255.0, 219.0 / 255.0);
        let hover_step = Color::from_rgb(41.0 / 255.0, 128.0 / 255.0, 185.0 / 255.0);
        svg::Style {
            color: if status == svg::Status::Hovered {
                Some(completed_step)
            } else {
                Some(hover_step)
            },
        }
    }

    fn get_style_selected(_: &Theme, _: svg::Status) -> svg::Style {
        let current_step = Color::from_rgb(46.0 / 255.0, 204.0 / 255.0, 113.0 / 255.0);
        svg::Style {
            color: Some(current_step),
        }
    }

    fn get_style_next(_: &Theme, _: svg::Status) -> svg::Style {
        let next_step = Color::from_rgb(243.0 / 255.0, 156.0 / 255.0, 18.0 / 255.0);
        svg::Style {
            color: Some(next_step),
        }
    }

    fn get_selected(&self) -> Svg {
        svg(self.selected.clone()).style(SvgImage::get_style_selected)
    }

    fn get_next(&self) -> Svg {
        svg(self.next.clone()).style(SvgImage::get_style_next)
    }

    fn get_previous(&self) -> Svg {
        svg(self.previous.clone()).style(SvgImage::get_style_previous)
    }

    fn load(path: String) -> Handle {
        Handle::from_path(format!("{}{}", env!("CARGO_MANIFEST_DIR"), path))
    }
}

impl ImageCollection {
    fn new() -> Self {
        let upload = SvgImage::new("/resources/upload.svg".to_string());
        let add_course = SvgImage::new("/resources/add-course.svg".to_string());
        let calc = SvgImage::new("/resources/calc.svg".to_string());
        let result = SvgImage::new("/resources/result.svg".to_string());
        let line = SvgImage::new("/resources/line.svg".to_string());
        let next_line = SvgImage::new("/resources/next-line.svg".to_string());
        Self {
            upload,
            add_course,
            calc,
            result,
            line,
            next_line,
        }
    }
}

impl Default for TCCScreen {
    fn default() -> Self {
        let available_screens = AvailableScreens::new();
        Self {
            screen: available_screens,
            active_screen: ScreenName::LoadData,
            err_message: "".to_string(),
            contact_list: None,
            start_point_latitude: 0_i32,
            start_point_longitude: 0_i32,
            goal_point_latitude: 0_i32,
            goal_point_longitude: 0_i32,
            course_name_list: vec!["".to_string()],
            calculator: None,
            image_collection: ImageCollection::new(),
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
