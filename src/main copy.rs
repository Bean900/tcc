use iced::widget::{
    button, checkbox, column, container, horizontal_space, radio, row,
    scrollable, slider, text, text_input, toggler, vertical_space,
};
use iced::widget::{Button, Column, Container, Slider};
use iced::{Center, Color, Element, Fill, Font, Pixels};

const TCC: &str = "Traveling Cook Calculator";

pub fn main() -> iced::Result {
 iced::application(TCC, Tour::update, Tour::view)
        .centered()
        .run()
}

pub struct Tour {
    screen: Screen,
    slider: u8,
    layout: Layout,
    spacing: u16,
    text_size: u16,
    text_color: Color,
    toggler: bool,
    input_value: String,
    input_is_secure: bool,
    input_is_showing_icon: bool,
    debug: bool,
}

#[derive(Debug, Clone)]
pub enum Message {
    LoadData,
    CreatePreview,
}

impl Tour {

    fn update(&mut self, event: Message) {
        match event {
            Message::LoadData => {
                if let Some(screen) = self.screen.previous() {
                    self.screen = screen;
                }
            }
            Message::CreatePreview => {
                if let Some(screen) = self.screen.next() {
                    self.screen = screen;
                }
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let controls =
            row![]
                .push_maybe(self.screen.previous().is_some().then(|| {
                    padded_button("Back")
                        .on_press(Message::BackPressed)
                        .style(button::secondary)
                }))
                .push(horizontal_space())
                .push_maybe(self.can_continue().then(|| {
                    padded_button("Next").on_press(Message::NextPressed)
                }));

        let screen = match self.screen {
            Screen::Welcome => self.welcome(),
            Screen::Radio => self.radio(),
            Screen::Toggler => self.toggler(),
            Screen::Slider => self.slider(),
            Screen::Text => self.text(),
            Screen::RowsAndColumns => self.rows_and_columns(),
            Screen::Scrollable => self.scrollable(),
            Screen::TextInput => self.text_input(),
            Screen::Debugger => self.debugger(),
            Screen::End => self.end(),
        };

        let content: Element<_> = column![screen, controls,]
            .max_width(540)
            .spacing(20)
            .padding(20)
            .into();

        let scrollable = scrollable(
            container(if self.debug {
                content.explain(Color::BLACK)
            } else {
                content
            })
            .center_x(Fill),
        );

        container(scrollable).center_y(Fill).into()
    }

    fn can_continue(&self) -> bool {
        match self.screen {
            Screen::LoadDataScreen => false,
        }
    }

    fn welcome(&self) -> Column<Message> {
        Self::container("Welcome!")
            .push(
                "This is a simple tour meant to showcase a bunch of widgets \
                 that can be easily implemented on top of Iced.",
            )
            .push(
                "Iced is a cross-platform GUI library for Rust focused on \
                 simplicity and type-safety. It is heavily inspired by Elm.",
            )
            .push(
                "It was originally born as part of Coffee, an opinionated \
                 2D game engine for Rust.",
            )
            .push(
                "On native platforms, Iced provides by default a renderer \
                 built on top of wgpu, a graphics library supporting Vulkan, \
                 Metal, DX11, and DX12.",
            )
            .push(
                "Additionally, this tour can also run on WebAssembly thanks \
                 to dodrio, an experimental VDOM library for Rust.",
            )
            .push(
                "You will need to interact with the UI in order to reach the \
                 end!",
            )
    }

    fn slider(&self) -> Column<Message> {
        Self::container("Slider")
            .push(
                "A slider allows you to smoothly select a value from a range \
                 of values.",
            )
            .push(
                "The following slider lets you choose an integer from \
                 0 to 100:",
            )
            .push(slider(0..=100, self.slider, Message::SliderChanged))
            .push(text(self.slider.to_string()).width(Fill).align_x(Center))
    }

    fn rows_and_columns(&self) -> Column<Message> {
        let row_radio = radio(
            "Row",
            Layout::Row,
            Some(self.layout),
            Message::LayoutChanged,
        );

        let column_radio = radio(
            "Column",
            Layout::Column,
            Some(self.layout),
            Message::LayoutChanged,
        );

        let layout_section: Element<_> = match self.layout {
            Layout::Row => {
                row![row_radio, column_radio].spacing(self.spacing).into()
            }
            Layout::Column => column![row_radio, column_radio]
                .spacing(self.spacing)
                .into(),
        };

        let spacing_section = column![
            slider(0..=80, self.spacing, Message::SpacingChanged),
            text!("{} px", self.spacing).width(Fill).align_x(Center),
        ]
        .spacing(10);

        Self::container("Rows and columns")
            .spacing(self.spacing)
            .push(
                "Iced uses a layout model based on flexbox to position UI \
                 elements.",
            )
            .push(
                "Rows and columns can be used to distribute content \
                 horizontally or vertically, respectively.",
            )
            .push(layout_section)
            .push("You can also easily change the spacing between elements:")
            .push(spacing_section)
    }

    fn text(&self) -> Column<Message> {
        let size = self.text_size;
        let color = self.text_color;

        let size_section = column![
            "You can change its size:",
            text!("This text is {size} pixels").size(size),
            slider(10..=70, size, Message::TextSizeChanged),
        ]
        .padding(20)
        .spacing(20);

        let color_sliders = row![
            color_slider(color.r, move |r| Color { r, ..color }),
            color_slider(color.g, move |g| Color { g, ..color }),
            color_slider(color.b, move |b| Color { b, ..color }),
        ]
        .spacing(10);

        let color_section = column![
            "And its color:",
            text!("{color:?}").color(color),
            color_sliders,
        ]
        .padding(20)
        .spacing(20);

        Self::container("Text")
            .push(
                "Text is probably the most essential widget for your UI. \
                 It will try to adapt to the dimensions of its container.",
            )
            .push(size_section)
            .push(color_section)
    }

    fn scrollable(&self) -> Column<Message> {
        Self::container("Scrollable")
            .push(
                "Iced supports scrollable content. Try it out! Find the \
                 button further below.",
            )
            .push(
                text("Tip: You can use the scrollbar to scroll down faster!")
                    .size(16),
            )
            .push(vertical_space().height(4096))
            .push(
                text("You are halfway there!")
                    .width(Fill)
                    .size(30)
                    .align_x(Center),
            )
            .push(vertical_space().height(4096))
            .push(text("You made it!").width(Fill).size(50).align_x(Center))
    }

    fn text_input(&self) -> Column<Message> {
        let value = &self.input_value;
        let is_secure = self.input_is_secure;
        let is_showing_icon = self.input_is_showing_icon;

        let mut text_input = text_input("Type something to continue...", value)
            .on_input(Message::InputChanged)
            .padding(10)
            .size(30);

        if is_showing_icon {
            text_input = text_input.icon(text_input::Icon {
                font: Font::default(),
                code_point: '🚀',
                size: Some(Pixels(28.0)),
                spacing: 10.0,
                side: text_input::Side::Right,
            });
        }

        Self::container("Text input")
            .push("Use a text input to ask for different kinds of information.")
            .push(text_input.secure(is_secure))
            .push(
                checkbox("Enable password mode", is_secure)
                    .on_toggle(Message::ToggleSecureInput),
            )
            .push(
                checkbox("Show icon", is_showing_icon)
                    .on_toggle(Message::ToggleTextInputIcon),
            )
            .push(
                "A text input produces a message every time it changes. It is \
                 very easy to keep track of its contents:",
            )
            .push(
                text(if value.is_empty() {
                    "You have not typed anything yet..."
                } else {
                    value
                })
                .width(Fill)
                .align_x(Center),
            )
    }

    fn debugger(&self) -> Column<Message> {
        Self::container("Debugger")
            .push(
                "You can ask Iced to visually explain the layouting of the \
                 different elements comprising your UI!",
            )
            .push(
                "Give it a shot! Check the following checkbox to be able to \
                 see element boundaries.",
            )
            .push(
                checkbox("Explain layout", self.debug)
                    .on_toggle(Message::DebugToggled),
            )
            .push("Feel free to go back and take a look.")
    }

    fn end(&self) -> Column<Message> {
        Self::container("You reached the end!")
            .push("This tour will be updated as more features are added.")
            .push("Make sure to keep an eye on it!")
    }

    fn container(title: &str) -> Column<'_, Message> {
        column![text(title).size(50)].spacing(20)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Screen {
    LoadDataScreen,

    Welcome,
    Slider,
    RowsAndColumns,
    Text,
    Radio,
    Toggler,
    Scrollable,
    TextInput,
    Debugger,
    End,
}

impl Screen {
    const ALL: &'static [Self] = &[
        Self::LoadDataScreen,

        Self::Welcome,
        Self::Slider,
        Self::RowsAndColumns,
        Self::Text,
        Self::Radio,
        Self::Toggler,
        Self::Scrollable,
        Self::TextInput,
        Self::Debugger,
        Self::End,
    ];

    pub fn next(self) -> Option<Screen> {
        Self::ALL
            .get(
                Self::ALL
                    .iter()
                    .copied()
                    .position(|screen| screen == self)
                    .expect("Screen must exist")
                    + 1,
            )
            .copied()
    }

    pub fn previous(self) -> Option<Screen> {
        let position = Self::ALL
            .iter()
            .copied()
            .position(|screen| screen == self)
            .expect("Screen must exist");

        if position > 0 {
            Some(Self::ALL[position - 1])
        } else {
            None
        }
    }
}

 
fn padded_button<Message: Clone>(label: &str) -> Button<'_, Message> {
    button(text(label)).padding([12, 24])
}

fn color_slider<'a>(
    component: f32,
    update: impl Fn(f32) -> Color + 'a,
) -> Slider<'a, f64, Message> {
    slider(0.0..=1.0, f64::from(component), move |c| {
        Message::TextColorChanged(update(c as f32))
    })
    .step(0.01)
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Layout {
    Row,
    Column,
}

impl Default for Tour {
    fn default() -> Self {
        Self {
            screen: Screen::Welcome,
            slider: 50,
            layout: Layout::Row,
            spacing: 20,
            text_size: 30,
            text_color: Color::BLACK,
            language: None,
            toggler: false,
            input_value: String::new(),
            input_is_secure: false,
            input_is_showing_icon: false,
            debug: false,
        }
    }
}