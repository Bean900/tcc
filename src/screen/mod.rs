pub(crate) mod load;
use std::rc::Rc;

use iced::Element;

use crate::{contact::Contact, LoadScreen, Message};

trait Screen {
    fn get(&self) -> Element<Message>;
    fn get_name(&self) -> ScreenName;
}

pub struct AvailableScreens {
    load_screen: LoadScreen,
}

impl AvailableScreens {
    pub fn new() -> Self {
        AvailableScreens {
            load_screen: LoadScreen::new(),
        }
    }

    pub fn set_contact_list(&mut self, contact_list: Rc<Vec<Contact>>) {
        self.load_screen.set_contact_list(Rc::clone(&contact_list));
    }

    pub fn get(&self, screen_name: ScreenName) -> Element<Message> {
        match screen_name {
            ScreenName::LoadData => self.load_screen.get(),
            ScreenName::AddRules => todo!(),
            ScreenName::Calculate => todo!(),
            ScreenName::Result => todo!(),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ScreenName {
    LoadData,
    AddRules,
    Calculate,
    Result,
}
