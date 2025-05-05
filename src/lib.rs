pub mod calculator;
pub mod contact;
mod image_collection;
pub mod screen;

use iced::widget::button;
use iced::window::{self, icon, Icon};

use iced::{
    alignment::Horizontal,
    widget::{column, container, row, Button},
    Element,
    Length::{self},
};
use image::ImageReader;
use image_collection::IMAGE_COLLECTION;
use std::path::Path;

use crate::screen::{load::LoadScreen, AvailableScreens, ScreenName};
use iced::time::{self, Duration};
use iced::{Alignment, Size, Subscription, Theme};

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
        .filter(None, LevelFilter::Info)
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
        .subscription(TCCScreen::subscription)
        .run()
}

pub struct TCCScreen {
    screen: AvailableScreens,
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
    AddCourseName(String),
    DeleteCourseName(usize),

    //Calculate screen actions
    Tick,
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
        let screen = container(self.screen.get())
            .height(Length::FillPortion(8))
            .width(Length::Fill);

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
                    Button::new(IMAGE_COLLECTION.upload.get_selected())
                        //.on_press(Message::GoToLoadDataScreen)
                        .style(move |_, _| button_style),
                    IMAGE_COLLECTION.next_line.get_next(),
                    Button::new(IMAGE_COLLECTION.add_course.get_next())
                        //.on_press(Message::GoToAddRulesScreen)
                        .style(move |_, _| button_style),
                    IMAGE_COLLECTION.next_line.get_next(),
                    Button::new(IMAGE_COLLECTION.calc.get_next())
                        //.on_press(Message::GoToCalculateScreen)
                        .style(move |_, _| button_style),
                    IMAGE_COLLECTION.next_line.get_next(),
                    Button::new(IMAGE_COLLECTION.result.get_next())
                        //.on_press(Message::GoToResultScreen)
                        .style(move |_, _| button_style),
                ],
                ScreenName::AddRules => row![
                    Button::new(IMAGE_COLLECTION.upload.get_previous())
                        .on_press(Message::GoToLoadDataScreen)
                        .style(move |_, _| button_style),
                    IMAGE_COLLECTION.line.get_previous(),
                    Button::new(IMAGE_COLLECTION.add_course.get_selected())
                        //.on_press(Message::GoToAddRulesScreen)
                        .style(move |_, _| button_style),
                    IMAGE_COLLECTION.next_line.get_next(),
                    Button::new(IMAGE_COLLECTION.calc.get_next())
                        //.on_press(Message::GoToCalculateScreen)
                        .style(move |_, _| button_style),
                    IMAGE_COLLECTION.next_line.get_next(),
                    Button::new(IMAGE_COLLECTION.result.get_next())
                        //.on_press(Message::GoToResultScreen)
                        .style(move |_, _| button_style),
                ],
                ScreenName::Calculate => row![
                    Button::new(IMAGE_COLLECTION.upload.get_previous())
                        .on_press(Message::GoToLoadDataScreen)
                        .style(move |_, _| button_style),
                    IMAGE_COLLECTION.line.get_previous(),
                    Button::new(IMAGE_COLLECTION.add_course.get_previous())
                        .on_press(Message::GoToAddRulesScreen)
                        .style(move |_, _| button_style),
                    IMAGE_COLLECTION.line.get_previous(),
                    Button::new(IMAGE_COLLECTION.calc.get_selected())
                        //.on_press(Message::GoToCalculateScreen)
                        .style(move |_, _| button_style),
                    IMAGE_COLLECTION.next_line.get_next(),
                    Button::new(IMAGE_COLLECTION.result.get_next())
                        //.on_press(Message::GoToResultScreen)
                        .style(move |_, _| button_style),
                ],
                ScreenName::Result => row![
                    Button::new(IMAGE_COLLECTION.upload.get_previous())
                        .on_press(Message::GoToLoadDataScreen)
                        .style(move |_, _| button_style),
                    IMAGE_COLLECTION.line.get_previous(),
                    Button::new(IMAGE_COLLECTION.add_course.get_previous())
                        .on_press(Message::GoToAddRulesScreen)
                        .style(move |_, _| button_style),
                    IMAGE_COLLECTION.line.get_previous(),
                    Button::new(IMAGE_COLLECTION.calc.get_previous())
                        .on_press(Message::GoToCalculateScreen)
                        .style(move |_, _| button_style),
                    IMAGE_COLLECTION.line.get_previous(),
                    Button::new(IMAGE_COLLECTION.result.get_selected())
                        //.on_press(Message::GoToResultScreen)
                        .style(move |_, _| button_style),
                ],
            }
            .align_y(Alignment::Center)
            .spacing(10),
        )
        .width(Length::Fill)
        .height(Length::FillPortion(2))
        .max_width(128 * 5)
        .align_y(iced::alignment::Vertical::Bottom)
        .into()
    }

    fn theme(&self) -> Theme {
        Theme::Light
    }

    fn subscription(&self) -> Subscription<Message> {
        if self.screen.needs_constant_update() {
            time::every(Duration::from_secs(1)).map(|_| Message::Tick)
        } else {
            Subscription::none()
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Layout {
    Row,
    Column,
}

impl Default for TCCScreen {
    fn default() -> Self {
        let available_screens = AvailableScreens::new();
        Self {
            screen: available_screens,
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
