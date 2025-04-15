pub(crate) mod load;
pub(crate) mod rule;

use iced::Element;
use rule::RuleScreen;

use crate::{LoadScreen, Message};

trait Screen {
    fn get(&self) -> Element<Message>;
    fn update(&mut self, event: Message);
}

pub struct AvailableScreens {
    load_screen: LoadScreen,
    rule_screen: RuleScreen,
    active_screen: ScreenName,
}

impl AvailableScreens {
    pub fn new() -> Self {
        AvailableScreens {
            load_screen: LoadScreen::new(),
            rule_screen: RuleScreen::new(),
            active_screen: ScreenName::LoadData,
        }
    }

    pub fn get(&self) -> Element<Message> {
        self.get_active_screen().get()
    }

    pub fn set_active_screen(&mut self, screen_name: ScreenName) {
        self.active_screen = screen_name;
    }

    pub fn update(&mut self, event: Message) {
        self.get_active_screen_mut().update(event);
    }

    fn get_active_screen(&self) -> &dyn Screen {
        match self.active_screen {
            ScreenName::LoadData => &self.load_screen,
            ScreenName::AddRules => &self.rule_screen,
            ScreenName::Calculate => todo!(),
            ScreenName::Result => todo!(),
        }
    }

    pub fn get_active_screen_name(&self) -> ScreenName {
        self.active_screen
    }

    fn get_active_screen_mut(&mut self) -> &mut dyn Screen {
        match self.active_screen {
            ScreenName::LoadData => &mut self.load_screen,
            ScreenName::AddRules => &mut self.rule_screen,
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
