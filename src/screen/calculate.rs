use std::sync::atomic::Ordering;

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
    calculator::{self, Calculator, CalculatorConfig},
    contact::{self, Contact},
    image_collection::IMAGE_COLLECTION,
    Message,
};

use super::Screen;

pub(crate) struct CalculateScreen {
    calculator: Option<Calculator>,
    err_message: Option<String>,
}

impl Screen for CalculateScreen {
    fn get(&self) -> Element<Message> {
        let calculator_opt = self.calculator.as_ref();

        if calculator_opt.is_none() {
            return column![text("No calculator started").size(50)].into();
        }

        let calculator = calculator_opt.expect("Expected calculator");

        let start_time = calculator
            .start_time
            .map(|start_time| format!("{}s", start_time.elapsed().as_secs()))
            .unwrap_or_else(|| "Calculation not started".to_string());

        let iteration = calculator.iterations.load(Ordering::SeqCst).to_string();

        let score = calculator
            .top_plan
            .lock()
            .expect("Failed to lock top_plan")
            .as_ref()
            .map(|plan| format!("{:.0}", plan.score))
            .unwrap_or_else(|| "No route found".to_string());

        column![
            text("Calculator").size(50),
            container(row![
                column!["Start time:", "Iteration:", "score:"],
                column![text(start_time), text(iteration), text(score)]
            ])
        ]
        .into()
    }
    fn update(&mut self, event: Message) {
        match event {
            _ => {}
        }
    }
}

impl CalculateScreen {
    pub fn new() -> Self {
        CalculateScreen {
            calculator: None,
            err_message: None,
        }
    }

    pub fn start_calculation(&mut self, calculator_config: CalculatorConfig) {
        let mut calculator = calculator::Calculator::new(calculator_config);
        calculator.calculate();
        self.calculator = Some(calculator);
    }

    pub fn stop_calculation(&mut self) {
        if let Some(calculator) = &mut self.calculator {
            calculator.stop();
        }
    }
}
