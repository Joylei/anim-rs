mod animator;
mod timeline;

use crate::{timeline::Options, Animatable};
#[doc(inline)]
pub use animator::{timeline, update};
#[doc(inline)]
pub use timeline::Timeline;

impl<T: Animatable + Send + Sync + 'static> From<Options<T>> for Timeline<T> {
    #[inline]
    fn from(opts: Options<T>) -> Self {
        timeline(opts)
    }
}

/// build a thread-local based [`Timeline`]
pub trait Builder<T> {
    /// build a thread-local based [`Timeline`]
    fn build(self) -> Timeline<T>;

    /// build a thread-local based [`Timeline`] and start animation
    fn begin_animation(self) -> Timeline<T>;
}

impl<T: Animatable + Send + Sync + 'static> Builder<T> for Options<T> {
    #[inline]
    fn build(self) -> Timeline<T> {
        timeline(self)
    }

    #[inline]
    fn begin_animation(self) -> Timeline<T> {
        let mut timeline = timeline(self);
        timeline.begin();
        timeline
    }
}
