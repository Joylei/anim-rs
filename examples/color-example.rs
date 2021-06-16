use anim::{
    local::{self as animator, Timeline},
    timeline::{self, Options},
};
use iced::{
    button, Align, Application, Button, Clipboard, Color, Command, Container, HorizontalAlignment,
    Length, Row, Subscription, Text, VerticalAlignment,
};
use std::time::Duration;

fn main() {
    State::run(Default::default()).unwrap();
}

#[derive(Debug, Clone)]
enum Message {
    Idle,
    Tick,
}

struct State {
    btn_color: button::State,
    btn_opacity: button::State,
    timeline: Timeline<(Color, Color)>,
}

impl Application for State {
    type Executor = iced::executor::Default;
    type Flags = ();
    type Message = self::Message;

    fn new(_flags: ()) -> (Self, Command<Message>) {
        let from = (Color::from_rgb8(0, 0, 255), Color::from_rgba8(0, 0, 0, 0.1));
        let to = (
            Color::from_rgb8(255, 0, 255),
            Color::from_rgba8(0, 0, 0, 1.0),
        );
        let app = Self {
            btn_color: Default::default(),
            btn_opacity: Default::default(),
            timeline: Options::new(from, to)
                .duration(Duration::from_secs(2))
                .auto_reverse(true)
                .forever()
                .into(),
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
            }
            _ => {}
        }
        Command::none()
    }

    fn view(&mut self) -> iced::Element<'_, Self::Message> {
        let (btn_color, btn_opacity) = self.timeline.value();
        let btn_color = Button::new(
            &mut self.btn_color,
            Text::new("color changes")
                .horizontal_alignment(HorizontalAlignment::Center)
                .vertical_alignment(VerticalAlignment::Center),
        )
        .style(style::Button(btn_color))
        .padding(20)
        .on_press(Message::Idle);

        let btn_opacity = Button::new(
            &mut self.btn_opacity,
            Text::new("opacity changes")
                .horizontal_alignment(HorizontalAlignment::Center)
                .vertical_alignment(VerticalAlignment::Center),
        )
        .style(style::Button(btn_opacity))
        .padding(20)
        .on_press(Message::Idle);

        let row = Row::new()
            .padding(10)
            .align_items(Align::Center)
            .push(btn_color)
            .push(btn_opacity);

        Container::new(row)
            .align_x(iced::Align::Center)
            .align_y(iced::Align::Center)
            .width(Length::Units(800))
            .height(Length::Units(600))
            .into()
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        const FPS: f32 = 60.0;
        iced::time::every(Duration::from_secs_f32(1.0 / FPS)).map(|_tick| Message::Tick)
    }
}

mod style {
    use iced::{button, Color};

    pub struct Button(pub Color);
    impl button::StyleSheet for Button {
        fn active(&self) -> button::Style {
            button::Style {
                background: Some(self.0.into()),
                text_color: Color::WHITE,
                ..Default::default()
            }
        }
    }
}
