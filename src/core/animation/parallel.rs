// anim
//
// An animation library, works nicely with Iced and the others
// Copyright: 2021, Joylei <leingliu@gmail.com>
// License: MIT

use super::{Animation, BaseAnimation};
use std::time::Duration;

/// parallel animations
#[derive(Debug, Clone)]
pub struct Parallel<A, B> {
    first: A,
    second: B,
}

impl<A, B> Parallel<A, B> {
    pub(super) fn new(first: A, second: B) -> Self {
        Self { first, second }
    }
}

impl<A, B> BaseAnimation for Parallel<A, B>
where
    A: Animation,
    B: Animation,
{
    type Item = (A::Item, B::Item);

    fn duration(&self) -> Option<Duration> {
        if let Some(first) = self.first.duration() {
            if let Some(second) = self.second.duration() {
                return Some(first.max(second));
            }
        }
        None
    }
    #[inline]
    fn animate(&self, elapsed: Duration) -> Self::Item {
        let first = self.first.animate(elapsed);
        let second = self.second.animate(elapsed);
        (first, second)
    }
}
