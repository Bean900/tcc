use calculator::Calculator;
use iced::{
    widget::{
        button, column, container, horizontal_space, row, scrollable, text, text_input, Column, Row,
    },
    Element, Length, Theme,
};

use iced::Fill;

mod contact;

mod calculator;

const TCC: &str = "Traveling Cook Calculator";

pub fn main() -> iced::Result {
    iced::application(TCC, TCCScreen::update, TCCScreen::view)
        .theme(TCCScreen::theme)
        .centered()
        .run()
}

pub struct TCCScreen {
    screen: Screen,
    err_message: String,
    contact_list: Option<Vec<contact::Contact>>,
    start_point_latitude: String,
    start_point_longitude: String,
    goal_point_latitude: String,
    goal_point_longitude: String,
    course_name_list: Vec<String>,
    calculator: Option<Calculator>,
}

#[derive(Debug, Clone)]
pub enum Message {
    LoadData,
    CreatePreview,
    GoToLoadDataScreen,
    GoToCheckDataScreen,
    GoToAddRulesScreen,
    GoToCalculateScreen,
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
                let start_point_latitude = self.start_point_latitude.parse::<f64>();
                let start_point_longitude = self.start_point_longitude.parse::<f64>();
                let goal_point_latitude = self.goal_point_latitude.parse::<f64>();
                let goal_point_longitude = self.goal_point_longitude.parse::<f64>();

                if start_point_latitude.is_ok()
                    && start_point_longitude.is_ok()
                    && goal_point_latitude.is_ok()
                    && goal_point_longitude.is_ok()
                {
                    let calculator = Calculator::new(
                        start_point_latitude.unwrap(),
                        start_point_longitude.unwrap(),
                        goal_point_latitude.unwrap(),
                        goal_point_longitude.unwrap(),
                        self.course_name_list.clone(),
                        self.contact_list.clone().unwrap(),
                    );

                    calculator.calculate();

                    self.calculator = Some(calculator);
                    self.screen = Screen::Calculate;
                } else {
                    let mut error_message: String = "Format of ".to_owned();
                    if start_point_latitude.is_err() {
                        error_message.push_str("start point latitude,");
                    }
                    if start_point_longitude.is_err() {
                        error_message.push_str("start point longitude,");
                    }
                    if goal_point_latitude.is_err() {
                        error_message.push_str("goal point latitude,");
                    }
                    if goal_point_longitude.is_err() {
                        error_message.push_str("goal point longitude,");
                    }
                    error_message = error_message[..error_message.len() - 2].to_owned();
                    error_message.push_str("is not a number!");
                    self.err_message = error_message;
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
                        text(contact.longitude.clone())
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
        column![row![
            "Start point (optional):",
            column![
                text_input("latitude", &self.start_point_latitude),
                text_input("longitude", &self.start_point_longitude),
            ]
        ]]
        .push(row![
            "Goal point (optional):",
            column![
                text_input("latitude", &self.goal_point_latitude),
                text_input("longitude", &self.goal_point_longitude)
            ]
        ])
        .push(row!["Courses:", text_input("", &self.course_name_list[0])])
        .push(button("Next").on_press(Message::GoToCalculateScreen))
    }

    fn calculate(&self) -> Column<Message> {
        column![
            row![
                column!["Shortest walking distance:", "10 min"],
                column!["Longest walking distance:", "20 min"]
            ],
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

impl Default for TCCScreen {
    fn default() -> Self {
        Self {
            screen: Screen::LoadData,
            err_message: "".to_string(),
            contact_list: None,
            start_point_latitude: "".to_string(),
            start_point_longitude: "".to_string(),
            goal_point_latitude: "".to_string(),
            goal_point_longitude: "".to_string(),
            course_name_list: Vec::new(),
            calculator: None,
        }
    }
}
