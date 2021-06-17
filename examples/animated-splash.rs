// https://www.flutterclutter.dev/flutter/tutorials/beautiful-animated-splash-screen/2020/1108/
use anim::{
    local::{self as animator, Timeline},
    timeline::{self, Boxed, Builder, Options, Status},
    Animation,
};
use iced::{
    canvas::{Cursor, Geometry},
    Application, Canvas, Clipboard, Color, Command, Container, Element, HorizontalAlignment,
    Length, Point, Rectangle, Subscription, Text, VerticalAlignment,
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
    timeline: Timeline<Raindrop>,
    painter: HolderPainter,
}

impl Application for State {
    type Executor = iced::executor::Default;
    type Flags = ();
    type Message = self::Message;

    fn new(_flags: ()) -> (Self, Command<Message>) {
        let app = Self {
            timeline: animator::timeline(raindrop()),
            painter: Default::default(),
        };
        (app, Command::none())
    }

    fn title(&self) -> String {
        "Size animation example".to_owned()
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
                animator::update();
                self.painter.model = self.timeline.value();
                self.painter.cache.clear();
            }
            _ => {}
        }
        Command::none()
    }

    fn view(&mut self) -> iced::Element<'_, Self::Message> {
        let status = self.timeline.status();
        let content: Element<Message> = if status == Status::Completed {
            Text::new("Animation completed")
                .horizontal_alignment(HorizontalAlignment::Center)
                .vertical_alignment(VerticalAlignment::Center)
                .width(Length::Fill)
                .height(Length::Fill)
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
        const FPS: f32 = 60.0;
        iced::time::every(Duration::from_secs_f32(1.0 / FPS)).map(|_tick| Message::Tick)
    }
}

#[derive(Default)]
struct HolderPainter {
    model: Raindrop,
    cache: iced::canvas::Cache,
}

impl iced::canvas::Program<Message> for HolderPainter {
    fn draw(&self, bounds: Rectangle, _cursor: Cursor) -> Vec<Geometry> {
        let item = self.cache.draw(bounds.size(), |frame| {
            if self.model.drop_visible {
                //use circle here instead of raindrop
                let center = Point::new(bounds.width / 2.0, bounds.height * self.model.drop_pos);
                let path = iced::canvas::Path::circle(center, self.model.drop_size as f32);
                frame.fill(&path, Color::WHITE);
            } else {
                let center = frame.center();
                let max_radius = frame.width().max(frame.height());
                let radius = max_radius * self.model.hole_size;
                //out circle
                let path = iced::canvas::Path::circle(center, radius);
                frame.fill(
                    &path,
                    Color {
                        a: 0.6,
                        ..Color::WHITE
                    },
                );
                //inner circle
                let path = iced::canvas::Path::circle(center, radius / 2.0);
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

fn raindrop() -> Boxed<Raindrop> {
    const MAX_DROP_SIZE: f32 = 20.0;
    const MAX_DROP_POS: f32 = 0.5;
    const MAX_HOLE_SIZE: f32 = 1.0;
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
        .build()
        .map(move |pos| Raindrop {
            drop_size: MAX_DROP_SIZE,
            drop_visible: true,
            drop_pos: pos,
            hole_size: 0.0,
        });
    let stage3 = Options::new(0.0, MAX_HOLE_SIZE)
        .duration(duration.mul_f64(0.5))
        .build()
        .map(move |size| Raindrop {
            drop_size: MAX_DROP_SIZE,
            drop_visible: false,
            drop_pos: MAX_DROP_POS,
            hole_size: size,
        });
    stage1.chain(stage2).chain(stage3).boxed()
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
