// anim
//
// An animation library, works nicely with Iced and the others
// Copyright: 2021, Joylei <leingliu@gmail.com>
// License: MIT

use super::{Animation, BaseAnimation};
use std::time::Duration;

/// chained animations, runs in orders
#[derive(Debug, Clone)]
pub struct Chain<A, B> {
    first: A,
    second: B,
}

impl<A, B> Chain<A, B> {
    #[inline]
    pub(super) fn new(first: A, second: B) -> Self {
        Self { first, second }
    }
}

impl<A, B> BaseAnimation for Chain<A, B>
where
    A: Animation,
    B: Animation<Item = A::Item>,
{
    type Item = A::Item;

    #[inline]
    fn duration(&self) -> Option<Duration> {
        if let Some(first) = self.first.duration() {
            if let Some(second) = self.second.duration() {
                return Some(first + second);
            }
        }
        None
    }

    #[inline]
    fn animate(&self, elapsed: Duration) -> Self::Item {
        if let Some(first) = self.first.duration() {
            if elapsed >= first {
                return self.second.animate(elapsed - first);
            }
        }
        self.first.animate(elapsed)
    }
}
