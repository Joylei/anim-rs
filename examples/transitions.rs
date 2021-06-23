// anim
//
// A framework independent animation library for rust, works nicely with Iced and the others
// Copyright: 2021, Joylei <leingliu@gmail.com>
// License: MIT

use anim::{
    easing::{self, EasingMode},
    timeline,
    transition::{fly, Transition},
};
use iced::{
    button, Application, Button, Clipboard, Command, Container, HorizontalAlignment, Length,
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
    transition: fly::Fly,
}

impl Application for State {
    type Executor = iced::executor::Default;
    type Flags = ();
    type Message = self::Message;

    fn new(_flags: ()) -> (Self, Command<Message>) {
        let transition = fly::Parameters::default()
            .offset(0.0, 300.0)
            .delay(Duration::from_millis(300))
            .duration(Duration::from_secs_f32(2.0))
            .easing(easing::quart_ease().mode(EasingMode::In))
            .fly_in();

        let app = Self {
            btn_test: Default::default(),
            transition,
        };
        (app, Command::none())
    }

    fn title(&self) -> String {
        "Map example".to_owned()
    }

    fn update(&mut self, message: Self::Message, _clipboard: &mut Clipboard) -> Command<Message> {
        match message {
            Message::Tick => {
                let status = self.transition.status();
                match status {
                    timeline::Status::Idle => {
                        self.transition.begin();
                    }
                    _ => {}
                }
                self.transition.update();
            }
            _ => {}
        }
        Command::none()
    }

    fn view(&mut self) -> iced::Element<'_, Self::Message> {
        let button = Button::new(
            &mut self.btn_test,
            Text::new("Fly in transition")
                .horizontal_alignment(HorizontalAlignment::Center)
                .vertical_alignment(VerticalAlignment::Center)
                .width(Length::Fill)
                .height(Length::Fill),
        )
        .style(style::Button)
        .on_press(Message::Idle);

        Container::new(self.transition.view(button))
            .align_x(iced::Align::Center)
            .align_y(iced::Align::Center)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        let status = self.transition.status();
        if status.is_animating() {
            const FPS: f32 = 60.0;
            iced::time::every(Duration::from_secs_f32(1.0 / FPS)).map(|_tick| Message::Tick)
        } else {
            iced::time::every(Duration::from_secs_f32(0.5)).map(|_tick| Message::Tick)
        }
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
