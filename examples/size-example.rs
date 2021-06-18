// anim
//
// A framework independent animation library for rust, works nicely with Iced and the others
// Copyright: 2021, Joylei <leingliu@gmail.com>
// License: MIT

use anim::{easing, timeline::Status, Options, Timeline};
use iced::{
    button, Align, Application, Button, Clipboard, Column, Command, Container, HorizontalAlignment,
    Length, Row, Size, Subscription, Text, VerticalAlignment,
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
    Start,
    Pause,
    Stop,
}

struct State {
    btn_start: button::State,
    btn_pause: button::State,
    btn_stop: button::State,
    btn_test: button::State,
    timeline: Timeline<Size>,
}

impl Application for State {
    type Executor = iced::executor::Default;
    type Flags = ();
    type Message = self::Message;

    fn new(_flags: ()) -> (Self, Command<Message>) {
        let mut timeline: Timeline<_> =
            Options::new(Size::new(130.0, 30.0), Size::new(500.0, 200.0))
                .duration(Duration::from_secs(2))
                .auto_reverse(true)
                .easing(easing::bounce_ease())
                .forever()
                .into();
        timeline.begin();
        let app = Self {
            btn_start: Default::default(),
            btn_pause: Default::default(),
            btn_stop: Default::default(),
            btn_test: Default::default(),
            timeline,
        };
        (app, Command::none())
    }

    fn title(&self) -> String {
        "Size animation example".to_owned()
    }

    fn update(&mut self, message: Self::Message, _clipboard: &mut Clipboard) -> Command<Message> {
        match message {
            Message::Tick => {
                self.timeline.update();
            }
            Message::Start => {
                let status = self.timeline.status();
                if status == Status::Paused {
                    self.timeline.resume();
                } else {
                    self.timeline.begin();
                }
            }
            Message::Pause => {
                self.timeline.pause();
            }
            Message::Stop => {
                self.timeline.stop();
            }
            _ => {}
        }
        Command::none()
    }

    fn view(&mut self) -> iced::Element<'_, Self::Message> {
        let status = self.timeline.status();
        let size = self.timeline.value();
        //eprintln!("{:?}", status);
        let controls = Row::new()
            .spacing(10)
            .push(button_optional(
                &mut self.btn_start,
                "Start",
                if status != Status::Animating {
                    Message::Start.into()
                } else {
                    None
                },
            ))
            .push(button_optional(
                &mut self.btn_pause,
                "Pause",
                if status == Status::Animating {
                    Message::Pause.into()
                } else {
                    None
                },
            ))
            .push(button_optional(
                &mut self.btn_stop,
                "Stop",
                if status == Status::Animating || status == Status::Paused {
                    Message::Stop.into()
                } else {
                    None
                },
            ));

        let content = Column::new()
            .spacing(20)
            .align_items(Align::Start)
            .width(Length::Fill)
            .height(Length::Fill)
            .push(controls)
            .push(
                Container::new(
                    Button::new(
                        &mut self.btn_test,
                        Text::new("size changes")
                            .horizontal_alignment(HorizontalAlignment::Center)
                            .vertical_alignment(VerticalAlignment::Center)
                            .width(Length::Fill)
                            .height(Length::Fill),
                    )
                    .style(style::Button)
                    .width(Length::Units(size.width as u16))
                    .height(Length::Units(size.height as u16))
                    .on_press(Message::Idle),
                )
                .align_x(Align::Center)
                .align_y(Align::Center)
                .width(Length::Fill)
                .height(Length::Fill),
            );

        Container::new(content)
            .align_x(iced::Align::Center)
            .align_y(iced::Align::Start)
            .padding(10)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        const FPS: f32 = 60.0;
        iced::time::every(Duration::from_secs_f32(1.0 / FPS)).map(|_tick| Message::Tick)
    }
}

fn button_optional<'a, Message: Clone>(
    state: &'a mut button::State,
    label: &str,
    on_press: Option<Message>,
) -> Button<'a, Message> {
    let mut btn = Button::new(state, Text::new(label));
    if let Some(on_press) = on_press {
        btn = btn.on_press(on_press);
    }
    btn
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
