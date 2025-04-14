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

pub struct Position {
    latitude: f64,
    longitude: f64,
}

pub struct TCCScreen {
    screen: AvailableScreens,
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
    //Switch screens
    GoToLoadDataScreen,
    GoToAddRulesScreen,
    GoToCalculateScreen,
    GoToResultScreen,

    //Load screen actions
    LoadData,

    //Rule screen actions
    ShowStartPositionInputField(bool),
    ShowGoalPositionInputField(bool),
    CheckInputCoordinateStartPointLatitude(String),
    CheckIbputCoordinateStartPointLongitude(String),
    CheckInputCoordinateGoalPointLatitude(String),
    CheckInputCoordinateGoalPointLongitude(String),

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
            Message::GoToLoadDataScreen => {
                self.screen.set_active_screen(ScreenName::LoadData);
            }
            Message::GoToAddRulesScreen => {
                self.screen.set_active_screen(ScreenName::AddRules);
            }
            Message::GoToCalculateScreen => {
                self.screen.set_active_screen(ScreenName::Calculate);
            }

            Message::GoToResultScreen => {
                self.screen.set_active_screen(ScreenName::Result);
            }
            _ => {
                self.screen.update(event);
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let screen = self.screen.get();

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
            match self.screen.get_active_screen_name() {
                ScreenName::LoadData => row![
                    Button::new(self.image_collection.upload.get_selected())
                        //.on_press(Message::GoToLoadDataScreen)
                        .style(move |_, _| button_style),
                    self.image_collection.next_line.get_next(),
                    Button::new(self.image_collection.add_course.get_next())
                        //.on_press(Message::GoToAddRulesScreen)
                        .style(move |_, _| button_style),
                    self.image_collection.next_line.get_next(),
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
                    self.image_collection.next_line.get_next(),
                    Button::new(self.image_collection.calc.get_next())
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
                    self.image_collection.line.get_previous(),
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

    fn theme(&self) -> Theme {
        Theme::Light
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
