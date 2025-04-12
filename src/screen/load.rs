use std::rc::Rc;

use iced::{
    alignment::{Horizontal, Vertical},
    border::Radius,
    widget::{button, column, container, row, scrollable, Button, Column, Row, Text},
    Alignment::Center,
    Border, Color, Element,
    Length::{self, Fill},
};

use crate::{contact::Contact, Message};

pub(crate) struct LoadScreen {
    contact_list: Option<Rc<Vec<Contact>>>,
}
impl LoadScreen {
    pub fn new() -> Self {
        LoadScreen { contact_list: None }
    }

    pub fn get(&self) -> Element<Message> {
        if self.contact_list.is_none() {
            LoadScreen::get_choose_file()
        } else {
            self.get_check_data()
        }
    }

    pub fn set_contact_list(&mut self, contact_list: Rc<Vec<Contact>>) {
        self.contact_list = Some(contact_list);
    }

    fn get_check_data(&self) -> Element<Message> {
        let butto_load_data =
            container(Button::new("Load Data").on_press(Message::LoadData)).center_x(Fill);
        let button_next = container(button("Next Step").on_press(Message::GoToAddRulesScreen));
        let mut contact_data_row = Row::new().spacing(1).align_y(Vertical::Top);
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
                contact_data_row = Row::new().spacing(1).align_y(Vertical::Top);
            }

            let contact_box = container(row![
                container(Text::new(contact.id))
                    .style(move |_| container::Style {
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
                        ..Default::default()
                    })
                    .center(Fill)
                    .width(24)
                    .height(24),
                container(column![
                    Text::new(contact.team_name.clone()),
                    Text::new(contact.address.clone()),
                    Text::new(format!("({} | {})", contact.latitude, contact.longitude))
                ])
                .style(move |_| container::Style {
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
                    ..Default::default()
                })
                .padding(2)
                .width(180)
                .height(100)
            ])
            .padding(5);
            contact_data_row = contact_data_row.push(contact_box);
            index += 1;
        }

        let data_container = container(scrollable(contact_dara_column).width(Fill))
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
            .height(Length::FillPortion(4))
            .padding(5)
            .center_x(Fill)
            .into();
        container(column![row![butto_load_data, button_next], data_container]).into()
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
