use std::sync::atomic::Ordering;

use iced::{
    alignment::{
        Horizontal::{self, Left},
        Vertical,
    },
    border::Radius,
    widget::{button, center, column, container, row, scrollable, text, Button, Column, Row, Text},
    Alignment::Center,
    Border, Color, Element,
    Length::{self, Fill},
};

use crate::{
    calculator::{self, Calculator, CalculatorConfig, Plan},
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
            .map(|start_time| {
                if let Some(stop_time) = calculator.stop_time {
                    let duration = stop_time.duration_since(start_time);
                    format!(
                        "{:02}:{:02}",
                        duration.as_secs() / 60,
                        duration.as_secs() % 60
                    )
                } else {
                    let elapsed = start_time.elapsed();
                    format!(
                        "{:02}:{:02}",
                        elapsed.as_secs() / 60,
                        elapsed.as_secs() % 60
                    )
                }
            })
            .unwrap_or_else(|| "00:00".to_string());

        let iteration = calculator.iterations.load(Ordering::SeqCst).to_string();

        let score = calculator
            .top_plan
            .lock()
            .expect("Failed to lock top_plan")
            .as_ref()
            .map(|plan| format!("{:.0}", plan.score))
            .unwrap_or_else(|| "-".to_string());

        let progress_info = container(row![
            column!["Time:", "Iteration:", "Score:"]
                .align_x(Horizontal::Left)
                .padding(10),
            column![text(start_time), text(iteration), text(score)]
                .align_x(Horizontal::Right)
                .padding(10),
        ])
        .align_x(Horizontal::Center);

        let next_button = container(
            if calculator
                .top_plan
                .lock()
                .expect("Failed to lock top_plan")
                .as_ref()
                .is_some()
            {
                button("Show Result").on_press(Message::GoToResultScreen)
            } else {
                button("Show Result")
            },
        );

        container(
            container(
                column![
                    text("Calculation in Progress").size(24),
                    progress_info,
                    next_button,
                ]
                .padding(10)
                .align_x(Horizontal::Center),
            )
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
            }),
        )
        .align_y(Vertical::Center)
        .align_x(Horizontal::Center)
        .height(Fill)
        .width(Fill)
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

    pub fn get_top_plan(&self) -> Option<Plan> {
        self.calculator.as_ref().and_then(|calculator| {
            calculator
                .top_plan
                .lock()
                .expect("Expect to get lock on top plan")
                .clone()
        })
    }
}
