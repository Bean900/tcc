pub(crate) mod calculate;
pub(crate) mod load;
pub(crate) mod rule;

use calculate::CalculateScreen;
use iced::Element;
use rule::RuleScreen;

use crate::{calculator::CalculatorConfig, LoadScreen, Message};

trait Screen {
    fn get(&self) -> Element<Message>;
    fn update(&mut self, event: Message);
}

pub struct AvailableScreens {
    load_screen: LoadScreen,
    rule_screen: RuleScreen,
    calculate_screen: CalculateScreen,
    active_screen: ScreenName,
}

impl AvailableScreens {
    pub fn new() -> Self {
        AvailableScreens {
            load_screen: LoadScreen::new(),
            rule_screen: RuleScreen::new(),
            calculate_screen: CalculateScreen::new(),
            active_screen: ScreenName::LoadData,
        }
    }

    pub fn get(&self) -> Element<Message> {
        self.get_active_screen().get()
    }

    pub fn set_active_screen(&mut self, screen_name: ScreenName) {
        if self.active_screen == ScreenName::Calculate && screen_name != ScreenName::Calculate {
            self.calculate_screen.stop_calculation();
            return;
        }

        if screen_name == ScreenName::Calculate {
            self.active_screen = self.start_calculation();
            return;
        }

        self.active_screen = screen_name;
    }

    pub fn update(&mut self, event: Message) {
        self.get_active_screen_mut().update(event);
    }

    fn get_active_screen(&self) -> &dyn Screen {
        match self.active_screen {
            ScreenName::LoadData => &self.load_screen,
            ScreenName::AddRules => &self.rule_screen,
            ScreenName::Calculate => &self.calculate_screen,
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
            ScreenName::Calculate => &mut self.calculate_screen,
            ScreenName::Result => todo!(),
        }
    }

    fn start_calculation(&mut self) -> ScreenName {
        let contact_list = self.load_screen.get_contact_list();
        if contact_list.is_none() {
            log::error!("Contact list is empty");
            return ScreenName::LoadData;
        }

        let course_name_list = self.rule_screen.get_course_name_list();
        if course_name_list.is_none() {
            log::error!("Course name list is None");
            return ScreenName::AddRules;
        }

        let calculator_config = CalculatorConfig::new_with_start_and_goal(
            self.rule_screen.get_start_point(),
            self.rule_screen.get_goal_point(),
            None,
            course_name_list.expect("Expect course name list"),
            contact_list.expect("Expect contact list"),
        );

        self.calculate_screen.start_calculation(calculator_config);

        return ScreenName::Calculate;
    }

    pub fn needs_constant_update(&self) -> bool {
        match self.active_screen {
            ScreenName::LoadData => false,
            ScreenName::AddRules => false,
            ScreenName::Calculate => true,
            ScreenName::Result => false,
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
