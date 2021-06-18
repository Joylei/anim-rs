// anim
//
// A framework independent animation library for rust, works nicely with Iced and the others
// Copyright: 2021, Joylei <leingliu@gmail.com>
// License: MIT

use super::{Animation, BaseAnimation};
use std::time::Duration;

/// map from one type to another
#[derive(Debug, Clone)]
pub struct Map<Source, F, T>
where
    Source: Animation,
    F: Fn(Source::Item) -> T,
{
    src: Source,
    f: F,
}

impl<Source, F, T> Map<Source, F, T>
where
    Source: Animation,
    F: Fn(Source::Item) -> T,
{
    #[inline(always)]
    pub(super) fn new(src: Source, f: F) -> Self {
        Self { src, f }
    }
}

impl<Source, F, T> BaseAnimation for Map<Source, F, T>
where
    Source: Animation,
    F: Fn(Source::Item) -> T,
{
    type Item = T;

    #[inline(always)]
    fn duration(&self) -> Option<Duration> {
        self.src.duration()
    }

    #[inline(always)]
    fn animate(&self, elapsed: Duration) -> Self::Item {
        let v = self.src.animate(elapsed);
        (self.f)(v)
    }
}
