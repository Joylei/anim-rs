// anim
//
// A framework independent animation library for rust, works nicely with Iced and the others
// Copyright: 2021, Joylei <leingliu@gmail.com>
// License: MIT
#![allow(unused)]

use anim::{
    easing::{self, EasingMode},
    timeline::{self, Status},
    transition::{fade, fly, slide, Transition},
};
use iced::{
    button, Application, Button, Clipboard, Command, Container, Element, HorizontalAlignment,
    Length, Space, Subscription, Text, VerticalAlignment,
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
    transition: Transitions,
}

impl Application for State {
    type Executor = iced::executor::Default;
    type Flags = ();
    type Message = self::Message;

    fn new(_flags: ()) -> (Self, Command<Message>) {
        //let transition = Transitions::Slide(SlideTransition::new());
        //let transition = Transitions::Fly(FlyTransition::new());
        let transition = Transitions::Fade(FadeTransition::new());

        let app = Self { transition };
        (app, Command::none())
    }

    fn title(&self) -> String {
        "Map example".to_owned()
    }

    fn update(&mut self, message: Self::Message, _clipboard: &mut Clipboard) -> Command<Message> {
        match message {
            Message::Tick => {
                self.transition.update();
            }
            _ => {}
        }
        Command::none()
    }

    fn view(&mut self) -> iced::Element<'_, Self::Message> {
        let content = self.transition.view();

        Container::new(content)
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
            iced::time::every(Duration::from_secs_f32(0.1)).map(|_tick| Message::Tick)
        }
    }
}

enum Transitions {
    /// fly transition
    Fly(FlyTransition),
    /// slide transition
    Slide(SlideTransition),
    /// fade transition
    Fade(FadeTransition),
}

impl Transitions {
    fn view(&mut self) -> Element<Message> {
        match self {
            Transitions::Fly(ref mut item) => item.view(),
            Transitions::Slide(ref mut item) => item.view(),
            Transitions::Fade(ref mut item) => item.view(),
        }
    }

    fn update(&mut self) {
        match self {
            Transitions::Fly(ref mut item) => item.update(),
            Transitions::Slide(ref mut item) => item.update(),
            Transitions::Fade(ref mut item) => item.update(),
        }
    }

    fn status(&self) -> Status {
        match self {
            Transitions::Fly(ref item) => item.transition.status(),
            Transitions::Slide(ref item) => item.transition.status(),
            Transitions::Fade(ref item) => item.transition.status(),
        }
    }
}

struct FlyTransition {
    btn: button::State,
    transition: fly::Fly,
}

impl FlyTransition {
    fn new() -> Self {
        let transition = fly::Parameters::default()
            .offset(0.0, 300.0)
            .delay(Duration::from_millis(300))
            .duration(Duration::from_secs_f32(2.0))
            .easing(easing::quart_ease().mode(EasingMode::In))
            .fly_in();
        Self {
            btn: Default::default(),
            transition,
        }
    }

    fn view(&mut self) -> Element<Message> {
        let button = Button::new(
            &mut self.btn,
            Text::new("Fly in transition")
                .horizontal_alignment(HorizontalAlignment::Center)
                .vertical_alignment(VerticalAlignment::Center)
                .width(Length::Fill)
                .height(Length::Fill),
        )
        .style(style::Button)
        .on_press(Message::Idle);
        self.transition.view(button)
    }

    fn update(&mut self) {
        let status = self.transition.status();
        match status {
            timeline::Status::Idle => {
                self.transition.begin();
            }
            _ => {}
        }
        self.transition.update();
    }
}

struct SlideTransition {
    btn: button::State,
    transition: slide::Slide,
}

impl SlideTransition {
    fn new() -> Self {
        let transition = slide::Parameters::default()
            .delay(Duration::from_millis(300))
            .duration(Duration::from_secs_f32(2.0))
            .easing(easing::quart_ease().mode(EasingMode::In))
            .slide_in();
        Self {
            btn: Default::default(),
            transition,
        }
    }

    fn view(&mut self) -> Element<Message> {
        let button = Button::new(
            &mut self.btn,
            Text::new("Slide in transition")
                .horizontal_alignment(HorizontalAlignment::Center)
                .vertical_alignment(VerticalAlignment::Center)
                .width(Length::Units(100))
                .height(Length::Units(100)),
        )
        .style(style::Button)
        .on_press(Message::Idle);

        self.transition.view(button).into()
    }

    fn update(&mut self) {
        let status = self.transition.status();
        match status {
            timeline::Status::Idle => {
                self.transition.begin();
            }
            _ => {}
        }
        self.transition.update();
    }
}

struct FadeTransition {
    btn: button::State,
    transition: fade::Fade,
}

impl FadeTransition {
    fn new() -> Self {
        let transition = fade::Parameters::default()
            .opacity(1.0)
            .delay(Duration::from_millis(300))
            .duration(Duration::from_secs_f32(2.0))
            .easing(easing::quart_ease().mode(EasingMode::In))
            .fade_in();
        Self {
            btn: Default::default(),
            transition,
        }
    }

    fn view(&mut self) -> Element<Message> {
        if self.transition.visible() {
            let button = Button::new(
                &mut self.btn,
                Text::new("Fade in transition")
                    .horizontal_alignment(HorizontalAlignment::Center)
                    .vertical_alignment(VerticalAlignment::Center)
                    .width(Length::Units(100))
                    .height(Length::Units(100)),
            )
            .style(style::FadedButton(self.transition.current()))
            .on_press(Message::Idle);

            button.into()
        } else {
            Space::new(Length::Units(0), Length::Units(0)).into()
        }
    }

    fn update(&mut self) {
        let status = self.transition.status();
        match status {
            timeline::Status::Idle => {
                self.transition.begin();
            }
            _ => {}
        }
        self.transition.update();
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

    pub struct FadedButton(pub f32);
    impl button::StyleSheet for FadedButton {
        fn active(&self) -> button::Style {
            button::Style {
                background: Some(
                    Color {
                        a: self.0,
                        ..Color::BLACK
                    }
                    .into(),
                ),
                text_color: Color::WHITE,
                ..Default::default()
            }
        }
    }
}
