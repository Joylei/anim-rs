// anim
//
// A framework independent animation library for rust, works nicely with Iced and the others
// Copyright: 2021, Joylei <leingliu@gmail.com>
// License: MIT

// https://www.flutterclutter.dev/flutter/tutorials/beautiful-animated-splash-screen/2020/1108/
use anim::{easing, timeline::Status, Animatable, Animation, Options, Timeline};
use iced::{
    canvas::{self, Cursor, Geometry},
    Align, Application, Button, Canvas, Clipboard, Color, Column, Command, Container, Element,
    HorizontalAlignment, Length, Point, Rectangle, Subscription, Text, VerticalAlignment,
};
use iced_native::button;
use std::time::Duration;

fn main() {
    State::run(Default::default()).unwrap();
}

#[derive(Debug, Clone)]
enum Message {
    /// animation frame
    Tick,
    Click1,
    Click2,
    Click3,
}

struct State {
    timeline: Timeline<Raindrop>,
    painter: AnimationPainter,
    btn_run1: button::State,
    btn_run2: button::State,
    btn_run3: button::State,
}

impl Application for State {
    type Executor = iced::executor::Default;
    type Flags = ();
    type Message = self::Message;

    fn new(_flags: ()) -> (Self, Command<Message>) {
        let app = Self {
            timeline: raindrop_animation2().begin_animation(),
            painter: Default::default(),
            btn_run1: Default::default(),
            btn_run2: Default::default(),
            btn_run3: Default::default(),
        };
        (app, Command::none())
    }

    fn title(&self) -> String {
        "Raindrop splash example".to_owned()
    }

    fn update(&mut self, message: Self::Message, _clipboard: &mut Clipboard) -> Command<Message> {
        match message {
            Message::Tick => {
                self.timeline.update();
                self.painter.model = self.timeline.value();
                self.painter.cache.clear();
            }
            Message::Click1 => {
                self.timeline = raindrop_animation().begin_animation();
            }
            Message::Click2 => {
                self.timeline = raindrop_animation2().begin_animation();
            }
            Message::Click3 => {
                self.timeline = raindrop_animation3().begin_animation();
            }
        }
        Command::none()
    }

    fn view(&mut self) -> iced::Element<'_, Self::Message> {
        let status = self.timeline.status();
        let content: Element<Message> = if status == Status::Completed {
            Column::new()
                .spacing(10)
                .width(Length::Shrink)
                .height(Length::Shrink)
                .align_items(Align::Center)
                .push(
                    Text::new("Animation completed")
                        .horizontal_alignment(HorizontalAlignment::Center)
                        .vertical_alignment(VerticalAlignment::Center)
                        .width(Length::Shrink)
                        .height(Length::Shrink),
                )
                .push(
                    Button::new(
                        &mut self.btn_run1,
                        Text::new("Run Again with method 1?")
                            .horizontal_alignment(HorizontalAlignment::Center)
                            .vertical_alignment(VerticalAlignment::Center),
                    )
                    .on_press(Message::Click1),
                )
                .push(
                    Button::new(
                        &mut self.btn_run2,
                        Text::new("Run Again with method 2?")
                            .horizontal_alignment(HorizontalAlignment::Center)
                            .vertical_alignment(VerticalAlignment::Center),
                    )
                    .on_press(Message::Click2),
                )
                .push(
                    Button::new(
                        &mut self.btn_run3,
                        Text::new("Run Again with method 3?")
                            .horizontal_alignment(HorizontalAlignment::Center)
                            .vertical_alignment(VerticalAlignment::Center),
                    )
                    .on_press(Message::Click3),
                )
                .into()
        } else {
            Canvas::new(&mut self.painter)
                .width(Length::Fill)
                .height(Length::Fill)
                .into()
        };
        Container::new(content)
            .align_x(iced::Align::Center)
            .align_y(iced::Align::Center)
            .width(Length::Fill)
            .height(Length::Fill)
            .style(style::Container)
            .into()
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        let status = self.timeline.status();
        if status.is_animating() {
            const FPS: f32 = 60.0;
            iced::time::every(Duration::from_secs_f32(1.0 / FPS)).map(|_tick| Message::Tick)
        } else {
            iced::Subscription::none()
        }
    }
}

#[derive(Default)]
struct AnimationPainter {
    model: Raindrop,
    cache: canvas::Cache,
}

impl canvas::Program<Message> for AnimationPainter {
    fn draw(&self, bounds: Rectangle, _cursor: Cursor) -> Vec<Geometry> {
        let item = self.cache.draw(bounds.size(), |frame| {
            if self.model.drop_visible {
                //use circle here instead of raindrop
                let center = Point::new(bounds.width / 2.0, bounds.height * self.model.drop_pos);
                let path = canvas::Path::circle(center, self.model.drop_size as f32);
                frame.fill(&path, Color::WHITE);
            } else {
                let center = frame.center();
                let max_radius = frame.width().max(frame.height());
                let radius = max_radius * self.model.hole_size;
                //out circle
                let path = canvas::Path::circle(center, radius);
                frame.fill(
                    &path,
                    Color {
                        a: 0.6,
                        ..Color::WHITE
                    },
                );
                //inner circle
                let path = canvas::Path::circle(center, radius / 2.0);
                frame.fill(
                    &path,
                    Color {
                        a: 0.1,
                        ..Color::WHITE
                    },
                );
            }
        });

        vec![item]
    }
}

#[derive(Clone)]
struct Raindrop {
    drop_size: f32,
    drop_pos: f32,
    drop_visible: bool,
    hole_size: f32,
}

impl Default for Raindrop {
    fn default() -> Self {
        Self {
            drop_size: 0.0,
            drop_pos: 0.0,
            drop_visible: true,
            hole_size: 0.0,
        }
    }
}

const MAX_DROP_SIZE: f32 = 20.0;
const MAX_DROP_POS: f32 = 0.5;
const MAX_HOLE_SIZE: f32 = 1.0;

// staged animations, demo of chained animations
fn raindrop_animation() -> impl Animation<Item = Raindrop> {
    let duration = Duration::from_millis(3000);
    let stage1 = Options::new(0.0, MAX_DROP_SIZE)
        .duration(duration.mul_f64(0.2))
        .build()
        .map(|size| Raindrop {
            drop_size: size,
            drop_visible: true,
            drop_pos: 0.0,
            hole_size: 0.0,
        });
    let stage2 = Options::new(0.0, MAX_DROP_POS)
        .duration(duration.mul_f64(0.3))
        .easing(easing::quad_ease())
        .build()
        .map(move |pos| Raindrop {
            drop_size: MAX_DROP_SIZE,
            drop_visible: true,
            drop_pos: pos,
            hole_size: 0.0,
        });
    let stage3 = Options::new(0.0, MAX_HOLE_SIZE)
        .duration(duration.mul_f64(0.5))
        .easing(easing::quad_ease())
        .build()
        .map(move |size| Raindrop {
            drop_size: MAX_DROP_SIZE,
            drop_visible: false,
            drop_pos: MAX_DROP_POS,
            hole_size: size,
        });
    stage1.chain(stage2).chain(stage3)
}

//demo of delay and parallel
fn raindrop_animation2() -> impl Animation<Item = Raindrop> {
    let duration = Duration::from_millis(3000);
    let drop_size = Options::new(0.0, MAX_DROP_SIZE)
        .duration(duration.mul_f64(0.2))
        .build();

    let drop_pos = Options::new(0.0, MAX_DROP_POS)
        .duration(duration.mul_f64(0.3))
        .easing(easing::quad_ease())
        .build()
        .delay(duration.mul_f64(0.2));

    //linear
    let drop_visible = anim::builder::linear(duration).map(|t| if t <= 0.5 { true } else { false });

    let hole_size = Options::new(0.0, MAX_HOLE_SIZE)
        .duration(duration.mul_f64(0.5))
        .easing(easing::quad_ease())
        .build()
        .delay(duration.mul_f64(0.5));

    drop_size
        .zip(drop_pos)
        .zip(drop_visible)
        .zip(hole_size)
        .map(
            |(((drop_size, drop_pos), drop_visible), hole_size)| Raindrop {
                drop_size,
                drop_pos,
                drop_visible,
                hole_size,
            },
        )
}

/// demo key-frames, requires Raindrop animatable
impl Animatable for Raindrop {
    fn animate(&self, to: &Self, time: f64) -> Self {
        let drop_size = self.drop_size.animate(&to.drop_size, time);
        let drop_pos = self.drop_pos.animate(&to.drop_pos, time);
        let drop_visible = self.drop_visible.animate(&to.drop_visible, time);
        let hole_size = self.hole_size.animate(&to.hole_size, time);
        Self {
            drop_size,
            drop_pos,
            drop_visible,
            hole_size,
        }
    }
}
fn raindrop_animation3() -> impl Animation<Item = Raindrop> {
    use anim::KeyFrame;
    let duration = Duration::from_millis(3000);
    anim::builder::key_frames(vec![
        KeyFrame::default(),
        KeyFrame::new(Raindrop {
            drop_size: MAX_DROP_SIZE,
            ..Default::default()
        })
        .by_percent(0.2),
        KeyFrame::new(Raindrop {
            drop_size: MAX_DROP_SIZE,
            drop_pos: MAX_DROP_POS,
            drop_visible: false,
            ..Default::default()
        })
        .by_percent(0.5)
        .easing(easing::quad_ease()),
        KeyFrame::new(Raindrop {
            drop_size: MAX_DROP_SIZE,
            drop_pos: MAX_DROP_POS,
            drop_visible: false,
            hole_size: MAX_HOLE_SIZE,
        })
        .by_duration(duration)
        .easing(easing::quad_ease()),
    ])
}

mod style {
    use iced::{container, Color};

    pub struct Container;
    impl container::StyleSheet for Container {
        fn style(&self) -> container::Style {
            container::Style {
                background: Color::from_rgb8(255, 0, 0).into(),
                text_color: Color::WHITE.into(),
                ..Default::default()
            }
        }
    }
}
