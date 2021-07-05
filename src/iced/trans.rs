use iced_graphics::{Backend, Renderer};
use iced_native::Element;

/// fly transition
pub(crate) mod fly;
/// slide transition
pub(crate) mod slide;

/// apply transition to iced Element
pub trait Apply {
    /// apply transition to iced Element
    fn apply<'a, Message, B, E>(&self, content: E) -> Element<'a, Message, Renderer<B>>
    where
        Message: 'a,
        B: Backend + 'a,
        E: Into<Element<'a, Message, Renderer<B>>>;
}
