use super::{Transition, DEFAULT_TRANSITION_DURATION};
use crate::{easing, timeline::Status, Animation, Options, Timeline};
use iced_graphics::{Backend, Defaults, Primitive, Renderer};
use iced_native::{
    mouse::Interaction, Element, Length, Point, Rectangle, Size, Space, Vector, Widget,
};
use std::{hash::Hash, time::Duration};

/// Slide transition parameters
///
/// see [`Slide`]
#[derive(Debug)]
pub struct Parameters {
    opt: Options<f32>,
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

    /// animation easing function
    pub fn easing(mut self, func: impl easing::Function + Clone + 'static) -> Self {
        self.opt = self.opt.easing(func);
        self
    }

    /// slide in transition
    pub fn slide_in(self) -> Slide {
        let Parameters { opt } = self;
        let delay = opt.delay.unwrap_or_default();
        let animation = opt
            .from(0.0)
            .to(1.0)
            .build()
            .zip(Options::new(false, true).duration(delay).build());
        Slide {
            timeline: animation.to_timeline(),
        }
    }

    /// slide out transition
    pub fn slide_out(self) -> Slide {
        let Parameters { opt } = self;
        let delay = opt.delay.unwrap_or_default();
        let duration = opt.duration;
        let animation = opt
            .from(1.0)
            .to(0.0)
            .build()
            .zip(Options::new(true, false).duration(delay + duration).build());
        Slide {
            timeline: animation.to_timeline(),
        }
    }
}

impl Default for Parameters {
    fn default() -> Self {
        let opt = Options::default().duration(DEFAULT_TRANSITION_DURATION);
        Self { opt }
    }
}

/// slide transition controller
///
/// ## Example
/// - slide in
/// ```rust
/// use std::time::Duration;
/// use anim::{Timeline, easing, transition::slide};
///
/// let transition = slide::Parameters::default()
///     .delay(Duration::from_millis(200))
///     .duration(Duration::from_secs(2))
///     .easing(easing::quad_ease())
///     .slide_in();
/// ```
/// - slide out
/// ```rust
/// use std::time::Duration;
/// use anim::{Timeline, easing, transition::slide};
///
/// let transition = slide::Parameters::default()
///     .delay(Duration::from_millis(200))
///     .duration(Duration::from_secs(2))
///     .easing(easing::quad_ease())
///     .slide_out();
/// ```
#[derive(Debug)]
pub struct Slide {
    timeline: Timeline<(f32, bool)>,
}

impl Slide {
    /// current height ratio
    pub fn current(&self) -> f32 {
        let (ratio, _) = self.timeline.value();
        ratio
    }

    /// build view
    pub fn view<'a, Message, B, E>(&self, content: E) -> Element<'a, Message, Renderer<B>>
    where
        E: Into<Element<'a, Message, Renderer<B>>>,
        B: Backend + 'a,
        Message: 'a,
    {
        let (ratio, visible) = self.timeline.value();
        //dbg!(ratio);
        if visible {
            let content = content.into();
            SlideElement::new(ratio, content).into()
        } else {
            Space::new(Length::Units(0), Length::Units(0)).into()
        }
    }
}

impl Transition for Slide {
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

struct SlideElement<'a, Message, B: Backend> {
    height_ratio: f32,
    content: Element<'a, Message, Renderer<B>>,
}

impl<'a, Message, B: Backend> SlideElement<'a, Message, B> {
    fn new<E>(height_ratio: f32, content: E) -> Self
    where
        E: Into<Element<'a, Message, Renderer<B>>>,
        Message: 'a,
    {
        Self {
            height_ratio,
            content: content.into(),
        }
    }
}

impl<'a, Message, B: Backend> Widget<Message, Renderer<B>> for SlideElement<'a, Message, B> {
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
        let node = self.content.layout(renderer, limits);
        if self.height_ratio >= 1.0 {
            node
        } else if self.height_ratio == 0.0 {
            iced_native::layout::Node::default()
        } else {
            let bounds = node.bounds();
            let clip_bounds = Rectangle::new(
                bounds.position(),
                Size::new(bounds.width, self.height_ratio * bounds.height),
            );
            iced_native::layout::Node::with_children(clip_bounds.size(), vec![node])
        }
    }

    fn draw(
        &self,
        renderer: &mut Renderer<B>,
        defaults: &Defaults,
        layout: iced_native::Layout<'_>,
        cursor_position: Point,
        viewport: &Rectangle,
    ) -> (Primitive, Interaction) {
        if self.height_ratio >= 1.0 {
            self.content
                .draw(renderer, defaults, layout, cursor_position, viewport)
        } else if self.height_ratio == 0.0 {
            (Primitive::None, Interaction::Idle)
        } else {
            let bounds = layout.bounds();
            let content_layout = layout.children().next().unwrap();
            let (primitive, interaction) = self.content.draw(
                renderer,
                defaults,
                content_layout,
                cursor_position,
                viewport,
            );
            (
                Primitive::Clip {
                    bounds: bounds,
                    offset: Vector::new(0, 0),
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
        let bounds = layout.bounds();
        if bounds.contains(cursor_position) {
            self.content.on_event(
                event,
                layout,
                cursor_position,
                renderer,
                clipboard,
                messages,
            )
        } else {
            iced_native::event::Status::Ignored
        }
    }

    fn hash_layout(&self, state: &mut iced_native::Hasher) {
        struct Marker;
        std::any::TypeId::of::<Marker>().hash(state);
        self.height_ratio.to_bits().hash(state);
        self.content.hash_layout(state);
    }

    fn overlay(
        &mut self,
        layout: iced_native::Layout<'_>,
    ) -> Option<iced_native::overlay::Element<'_, Message, Renderer<B>>> {
        if self.height_ratio == 0.0 {
            None
        } else if self.height_ratio == 1.0 {
            self.content.overlay(layout)
        } else {
            let content_layout = layout.children().next().unwrap();
            self.content.overlay(content_layout)
        }
    }
}

impl<'a, Message, B> From<SlideElement<'a, Message, B>> for Element<'a, Message, Renderer<B>>
where
    B: Backend + 'a,
    Message: 'a,
{
    fn from(src: SlideElement<'a, Message, B>) -> Self {
        Element::new(src)
    }
}
