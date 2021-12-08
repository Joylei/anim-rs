// anim
//
// A framework independent animation library for rust, works nicely with Iced and the others
// Copyright: 2021, Joylei <leingliu@gmail.com>
// License: MIT

use super::{Animation, BaseAnimation};
use std::{fmt, time::Duration};

/// wrapper for boxed [`Animation`]
pub struct Boxed<T>(Box<dyn Animation<Item = T>>);

impl<T> Boxed<T> {
    /// construct [`Boxed`]
    #[inline]
    pub(crate) fn new<F>(src: F) -> Self
    where
        F: Animation<Item = T> + 'static,
    {
        Self(Box::new(src))
    }
}

impl<T> BaseAnimation for Boxed<T> {
    type Item = T;
    #[inline]
    fn duration(&self) -> Option<Duration> {
        self.0.duration()
    }

    #[inline]
    fn animate(&self, elapsed: Duration) -> Self::Item {
        self.0.animate(elapsed)
    }
}

impl<T> fmt::Debug for Boxed<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "BoxedAnimation")
    }
}
