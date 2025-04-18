use iced::{
    alignment::{
        Horizontal::{self, Left},
        Vertical,
    },
    border::Radius,
    widget::{button, column, container, row, scrollable, text, Button, Column, Row, Text},
    Alignment::Center,
    Border, Color, Element,
    Length::{self, Fill},
};

use crate::{
    contact::{self, Contact},
    image_collection::IMAGE_COLLECTION,
    Message,
};

use super::Screen;

pub(crate) struct LoadScreen {
    contact_list: Option<Vec<Contact>>,
    err_message: Option<String>,
}

impl Screen for LoadScreen {
    fn get(&self) -> Element<Message> {
        if self.contact_list.is_none() {
            LoadScreen::get_choose_file()
        } else {
            self.get_check_data()
        }
    }
    fn update(&mut self, event: Message) {
        match event {
            Message::LoadData => {
                let contact_loader = contact::ContactLoader::new();

                let contact_list_result = contact_loader.load();

                if contact_list_result.is_err() == true {
                    log::error!(
                        "Error loading contact list: {}",
                        contact_list_result
                            .as_ref()
                            .err()
                            .expect("Expect error when failed loading contact list")
                    );
                    self.err_message = Some("Error while loading contact list!".to_string());
                    return;
                } else {
                    self.err_message = None;
                }

                let contact_list = contact_list_result.expect("Contact list should exist");

                if contact_list.is_none() {
                    return;
                }

                self.contact_list = contact_list.clone();
            }
            _ => {}
        }
    }
}

impl LoadScreen {
    pub fn new() -> Self {
        LoadScreen {
            contact_list: None,
            err_message: None,
        }
    }

    fn get_check_data(&self) -> Element<Message> {
        let headline = Text::new("Cook And Run Teams").size(25).align_x(Left);
        let butto_load_data =
            container(Button::new("Load Data").on_press(Message::LoadData)).align_right(Fill);
        let button_next =
            container(button("Next Step").on_press(Message::GoToAddRulesScreen)).align_right(Fill);

        container(column![
            row![headline, butto_load_data, button_next],
            self.get_contact_list_element()
        ])
        .height(Length::FillPortion(4))
        .into()
    }

    fn get_contact_list_element(&self) -> Element<Message> {
        let mut contact_data_row = Row::new().spacing(10).align_y(Vertical::Top);
        let mut contact_dara_column = Column::new()
            .spacing(5)
            .align_x(Horizontal::Left)
            .padding(5);

        let mut index = 0;

        for contact in self
            .contact_list
            .as_ref()
            .expect("Expect contacts to be loaded!")
            .iter()
        {
            if index % 2 == 0 && index != 0 {
                contact_dara_column = contact_dara_column.push(contact_data_row.padding(5));
                contact_data_row = Row::new().spacing(10).align_y(Vertical::Top);
            }

            let contact_box = Self::team_card(
                &contact.team_name,
                &contact.address,
                (contact.latitude, contact.longitude),
                index,
            );
            contact_data_row = contact_data_row.push(contact_box);
            index += 1;
        }

        if index % 2 != 0 {
            contact_dara_column = contact_dara_column.push(contact_data_row.padding(5));
        }

        container(scrollable(contact_dara_column).width(Fill))
            .style(move |_| container::Style {
                border: Border {
                    color: Color::from_rgb(0.5, 0.5, 0.5),
                    radius: Radius {
                        top_left: 10.0,
                        top_right: 10.0,
                        bottom_right: 10.0,
                        bottom_left: 10.0,
                    },
                    width: 3.0,
                },
                ..Default::default()
            })
            .padding(5)
            .center_x(Fill)
            .into()
    }

    fn team_card<'a>(
        name: &'a str,
        street: &'a str,
        coords: (i32, i32),
        index: usize,
    ) -> Element<'a, Message> {
        let icons = column![
            IMAGE_COLLECTION.team.get(20),
            IMAGE_COLLECTION.user_card.get(24),
            IMAGE_COLLECTION.pin.get(18)
        ]
        .spacing(8);

        let text = column![
            text(format!("Team {}", (index + 1))).size(20),
            text(name).size(20).color(Color::from_rgb(0.0, 0.0, 0.5)),
            text(street).size(18),
            text(format!("({:?} | {:?})", coords.0, coords.1)).size(16)
        ];

        let content = row![icons, text].padding(10);

        container(content)
            .width(250)
            .height(120)
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
            .into()
    }

    pub fn get_contact_list(&self) -> Option<Vec<Contact>> {
        self.contact_list.clone()
    }

    fn get_choose_file() -> Element<'static, Message> {
        let headline = Text::new("Traveling Cook Calculator").size(50);
        let sub_headline = container(Text::new("Load Contact-Data").size(20).align_x(Center))
            .width(Fill)
            .center_x(Fill)
            .padding(10);
        let short_description =
            container(Text::new("Please load the contact data from a CSV-File.").size(15))
                .width(Fill)
                .center_x(Fill);
        let button = container(Button::new("Load Data").on_press(Message::LoadData)).center_x(Fill);
        let upload_area = container(column![sub_headline, short_description, button])
            .style(move |_| container::Style {
                border: Border {
                    color: Color::from_rgb(0.5, 0.5, 0.5),
                    radius: Radius {
                        top_left: 10.0,
                        top_right: 10.0,
                        bottom_right: 10.0,
                        bottom_left: 10.0,
                    },
                    width: 3.0,
                },
                ..Default::default()
            })
            .width(Fill)
            .center(Fill)
            .height(Fill)
            .padding(40);

        container(column![headline, upload_area])
            .height(Length::FillPortion(4))
            .padding(5)
            .center_x(Fill)
            .into()
    }
}
