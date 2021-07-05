use super::Apply;
use crate::trans::fly::Fly;
use iced_graphics::{Backend, Defaults, Primitive, Renderer};
use iced_native::{mouse::Interaction, Element, Length, Point, Rectangle, Space, Vector, Widget};

impl Apply for Fly {
    /// build view
    fn apply<'a, Message, B, E>(&self, content: E) -> Element<'a, Message, Renderer<B>>
    where
        Message: 'a,
        B: Backend + 'a,
        E: Into<Element<'a, Message, Renderer<B>>>,
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
