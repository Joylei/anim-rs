use iced_graphics::{Backend, Defaults, Primitive, Renderer};
use iced_native::{mouse::Interaction, Element, Length, Point, Rectangle, Space, Vector, Widget};

use super::{Transition, DEFAULT_TRANSITION_DURATION};
use crate::{animation::constant, easing, timeline::Status, Animation, Options, Timeline};
use std::{fmt, marker::PhantomData, time::Duration};

/// fly transition parameters
///
/// see [`Fly`]
#[derive(Debug)]
pub struct Parameters {
    opt: Options<Vector>,
    offset: Vector,
}

impl Parameters {
    /// delay of animation
    pub fn delay(mut self, delay: Duration) -> Self {
        self.opt = self.opt.delay(delay);
        self
    }

    /// duration of animation
    pub fn duration(mut self, duration: Duration) -> Self {
        self.opt = self.opt.duration(duration);
        self
    }

    /// transition from/to offset x
    pub fn offset_x(mut self, x: f32) -> Self {
        self.offset = Vector { x, ..self.offset };
        self
    }

    /// transition from/to offset y
    pub fn offset_y(mut self, y: f32) -> Self {
        self.offset = Vector { y, ..self.offset };
        self
    }

    /// transition from/to offset
    pub fn offset(mut self, x: f32, y: f32) -> Self {
        self.offset = Vector { x, y };
        self
    }

    /// animation easing function
    pub fn easing(mut self, func: impl easing::Function + Clone + 'static) -> Self {
        self.opt = self.opt.easing(func);
        self
    }

    /// fly in transition
    pub fn fly_in(self) -> Fly {
        let Parameters { opt, offset } = self;
        let delay = opt.delay.unwrap_or_default();
        let animation = opt
            .from(offset)
            .build()
            .zip(Options::new(false, true).duration(delay).build());
        Fly {
            timeline: animation.to_timeline(),
        }
    }

    /// fly out transition
    pub fn fly_out(self) -> Fly {
        let Parameters { opt, offset } = self;
        let delay = opt.delay.unwrap_or_default();
        let duration = opt.duration;
        let animation = opt
            .to(offset)
            .build()
            .zip(Options::new(true, false).duration(delay + duration).build());
        Fly {
            timeline: animation.to_timeline(),
        }
    }
}

impl Default for Parameters {
    fn default() -> Self {
        let opt = Options::default().duration(DEFAULT_TRANSITION_DURATION);
        Self {
            opt,
            offset: Default::default(),
        }
    }
}

/// fly transition controller
///
/// ## Example
/// - fly in
/// ```rust
/// use std::time::Duration;
/// use anim::{Timeline, easing, transition::fly};
///
/// let transition = fly::Parameters::default()
///     .offset(0.0, 300.0)
///     .delay(Duration::from_millis(200))
///     .duration(Duration::from_secs(2))
///     .easing(easing::quad_ease())
///     .fly_in();
/// ```
/// - fly out
/// ```rust
/// use std::time::Duration;
/// use anim::{Timeline, easing, transition::fly};
///
/// let transition = fly::Parameters::default()
///     .offset(0.0, 300.0)
///     .delay(Duration::from_millis(200))
///     .duration(Duration::from_secs(2))
///     .easing(easing::quad_ease())
///     .fly_out();
/// ```
#[derive(Debug)]
pub struct Fly {
    timeline: Timeline<(Vector, bool)>,
}

impl Fly {
    fn get_value(&self) -> (Vector, bool) {
        let status = self.timeline.status();
        let (offset, visible) = self.timeline.value();
        if status.is_animating() {
            (offset, visible)
        } else {
            (Default::default(), visible)
        }
    }

    /// current offset
    pub fn current(&self) -> Vector {
        let status = self.timeline.status();
        let (offset, _) = self.timeline.value();
        if status.is_animating() {
            offset
        } else {
            Default::default()
        }
    }

    /// build view
    pub fn view<'a, Message, B, E>(&self, content: E) -> Element<'a, Message, Renderer<B>>
    where
        E: Into<Element<'a, Message, Renderer<B>>>,
        B: Backend + 'a,
        Message: 'a,
    {
        let (offset, visible) = self.get_value();
        if visible {
            let content = content.into();
            FlyElement::new(offset, content).into()
        } else {
            Space::new(Length::Units(0), Length::Units(0)).into()
        }
    }
}

impl Transition for Fly {
    fn begin(&mut self) {
        self.timeline.begin();
    }

    fn stop(&mut self) {
        self.timeline.stop();
    }

    fn update(&mut self) {
        self.timeline.update();
    }

    fn status(&self) -> Status {
        self.timeline.status()
    }

    fn visible(&self) -> bool {
        let (_, v) = self.timeline.value();
        v
    }
}

struct FlyElement<'a, Message, B: Backend> {
    offset: Vector,
    content: Element<'a, Message, Renderer<B>>,
}

impl<'a, Message, B: Backend> FlyElement<'a, Message, B> {
    fn new<E>(offset: Vector, content: E) -> Self
    where
        E: Into<Element<'a, Message, Renderer<B>>>,
        Message: 'a,
    {
        Self {
            offset,
            content: content.into(),
        }
    }
}

impl<'a, Message, B: Backend> Widget<Message, Renderer<B>> for FlyElement<'a, Message, B> {
    fn width(&self) -> Length {
        self.content.width()
    }

    fn height(&self) -> Length {
        self.content.height()
    }

    fn layout(
        &self,
        renderer: &Renderer<B>,
        limits: &iced_native::layout::Limits,
    ) -> iced_native::layout::Node {
        self.content.layout(renderer, limits)
    }

    fn draw(
        &self,
        renderer: &mut Renderer<B>,
        defaults: &Defaults,
        layout: iced_native::Layout<'_>,
        cursor_position: Point,
        viewport: &Rectangle,
    ) -> (Primitive, Interaction) {
        if self.offset.x == 0.0 && self.offset.y == 0.0 {
            self.content
                .draw(renderer, defaults, layout, cursor_position, viewport)
        } else {
            let (primitive, interaction) =
                self.content
                    .draw(renderer, defaults, layout, cursor_position, viewport);
            (
                Primitive::Translate {
                    translation: self.offset,
                    content: primitive.into(),
                },
                interaction,
            )
        }
    }

    fn on_event(
        &mut self,
        event: iced_native::Event,
        layout: iced_native::Layout<'_>,
        cursor_position: Point,
        renderer: &Renderer<B>,
        clipboard: &mut dyn iced_native::Clipboard,
        messages: &mut Vec<Message>,
    ) -> iced_native::event::Status {
        self.content.on_event(
            event,
            layout,
            cursor_position,
            renderer,
            clipboard,
            messages,
        )
    }

    fn hash_layout(&self, state: &mut iced_native::Hasher) {
        self.content.hash_layout(state)
    }

    fn overlay(
        &mut self,
        layout: iced_native::Layout<'_>,
    ) -> Option<iced_native::overlay::Element<'_, Message, Renderer<B>>> {
        self.content.overlay(layout)
    }
}

impl<'a, Message, B> From<FlyElement<'a, Message, B>> for Element<'a, Message, Renderer<B>>
where
    B: Backend + 'a,
    Message: 'a,
{
    fn from(src: FlyElement<'a, Message, B>) -> Self {
        Element::new(src)
    }
}
