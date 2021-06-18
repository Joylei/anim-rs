// anim
//
// An animation library, works nicely with Iced and the others
// Copyright: 2021, Joylei <leingliu@gmail.com>
// License: MIT

use anim::{
    easing,
    timeline::{self, Status},
    Animation, Options, Timeline,
};
use iced::{
    button, Application, Button, Clipboard, Command, Container, HorizontalAlignment, Length, Size,
    Subscription, Text, VerticalAlignment,
};
use std::time::Duration;

fn main() {
    State::run(Default::default()).unwrap();
}

#[derive(Debug, Clone)]
enum Message {
    Idle,
    /// animation frame
    Tick,
}

struct State {
    btn_test: button::State,
    timeline: Timeline<(Length, Length)>,
}

impl Application for State {
    type Executor = iced::executor::Default;
    type Flags = ();
    type Message = self::Message;

    fn new(_flags: ()) -> (Self, Command<Message>) {
        let app = Self {
            btn_test: Default::default(),
            timeline: Options::new(Size::new(130.0, 30.0), Size::new(500.0, 200.0))
                .duration(Duration::from_secs(2))
                .auto_reverse(true)
                .easing(easing::bounce_ease())
                .times(3)
                .build()
                .map(|size| {
                    (
                        Length::Units(size.width as u16),
                        Length::Units(size.height as u16),
                    )
                })
                .into(),
        };
        (app, Command::none())
    }

    fn title(&self) -> String {
        "Map example".to_owned()
    }

    fn update(&mut self, message: Self::Message, _clipboard: &mut Clipboard) -> Command<Message> {
        match message {
            Message::Tick => {
                let status = self.timeline.status();
                match status {
                    timeline::Status::Idle => {
                        self.timeline.begin();
                    }
                    _ => {}
                }
                self.timeline.update();
            }
            _ => {}
        }
        Command::none()
    }

    fn view(&mut self) -> iced::Element<'_, Self::Message> {
        let size = self.timeline.value();
        let status = self.timeline.status();
        let button = Button::new(
            &mut self.btn_test,
            Text::new(if status == Status::Completed {
                "stopped"
            } else {
                "size changes"
            })
            .horizontal_alignment(HorizontalAlignment::Center)
            .vertical_alignment(VerticalAlignment::Center)
            .width(Length::Fill)
            .height(Length::Fill),
        )
        .style(style::Button)
        .width(size.0)
        .height(size.1)
        .on_press(Message::Idle);

        Container::new(button)
            .align_x(iced::Align::Center)
            .align_y(iced::Align::Center)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        const FPS: f32 = 60.0;
        iced::time::every(Duration::from_secs_f32(1.0 / FPS)).map(|_tick| Message::Tick)
    }
}

mod style {
    use iced::{button, Color};

    pub struct Button;
    impl button::StyleSheet for Button {
        fn active(&self) -> button::Style {
            button::Style {
                background: Some(Color::BLACK.into()),
                text_color: Color::WHITE,
                ..Default::default()
            }
        }
    }
}
