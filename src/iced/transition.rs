use iced_graphics::{Backend, Defaults, Primitive, Renderer};
use iced_native::{mouse::Interaction, Color, Length, Point, Rectangle, Widget};

use crate::{easing, timeline::Status};
use std::{marker::PhantomData, rc::Rc, time::Duration};

/// default transition duration
pub const DEFAULT_TRANSITION_DURATION: Duration = Duration::from_millis(400);

/// fly transition
pub mod fly;

pub mod fade;

/// transition controller
pub trait Transition {
    /// update the transition, so the animation can be evaluated to the next state
    fn update(&mut self);
    /// start transition
    fn begin(&mut self);
    /// stop transition
    fn stop(&mut self);
    /// animation status()
    fn status(&self) -> Status;
    /// indicate the visibility of your target element
    fn visible(&self) -> bool;
}

pub struct Wrapper {}

pub struct Fade<F: easing::Function> {
    delay: Duration,
    duration: Duration,
    easing: F,
}
pub struct Blur<F: easing::Function> {
    delay: Duration,
    duration: Duration,
    easing: F,
    opacity: f32,
    amount: u16,
}

pub struct Fly<F: easing::Function> {
    delay: Duration,
    duration: Duration,
    easing: F,
    opacity: f32,
    x: i32,
    y: i32,
}

pub struct Slide<F: easing::Function> {
    delay: Duration,
    duration: Duration,
    easing: F,
}

pub struct Scale<F: easing::Function> {
    delay: Duration,
    duration: Duration,
    easing: F,
    start: f32,
    opacity: f32,
}

#[derive(Debug)]
pub struct State {
    visible: bool,
}

impl State {
    pub fn show(&mut self) {
        self.visible = true;
    }

    pub fn hide(&mut self) {
        self.visible = false;
    }

    pub fn toggle(&mut self) {
        self.visible = !self.visible;
    }
}

pub struct AnimatedElement<'a, Message, B, W>
where
    B: Backend,
    W: Widget<Message, Renderer<B>>,
{
    state: &'a mut State,
    width: Length,
    height: Length,
    widget: W,
    _marker: PhantomData<(Message, B)>,
}

impl<'a, Message, B, W> Widget<Message, Renderer<B>> for AnimatedElement<'a, Message, B, W>
where
    B: Backend,
    W: Widget<Message, Renderer<B>>,
{
    fn width(&self) -> Length {
        if self.state.visible {
            self.width
        } else {
            Length::Units(0)
        }
    }

    fn height(&self) -> Length {
        if self.state.visible {
            self.height
        } else {
            Length::Units(0)
        }
    }

    fn layout(
        &self,
        renderer: &Renderer<B>,
        limits: &iced_native::layout::Limits,
    ) -> iced_native::layout::Node {
        todo!()
    }

    fn draw(
        &self,
        renderer: &mut Renderer<B>,
        defaults: &Defaults,
        layout: iced_native::Layout<'_>,
        cursor_position: Point,
        viewport: &Rectangle,
    ) -> (Primitive, Interaction) {
        todo!()
    }

    fn hash_layout(&self, state: &mut iced_native::Hasher) {
        todo!()
    }
}
