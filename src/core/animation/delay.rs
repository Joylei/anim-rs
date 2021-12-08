// anim
//
// A framework independent animation library for rust, works nicely with Iced and the others
// Copyright: 2021, Joylei <leingliu@gmail.com>
// License: MIT

use super::{Animation, BaseAnimation};
use crate::core::DURATION_ZERO;
use std::time::Duration;

/// delay your animation for a specified time; negative delay has no effect
#[derive(Debug, Clone)]
pub struct Delay<T: Animation> {
    src: T,
    delay: Duration,
}

impl<T: Animation> Delay<T> {
    #[inline]
    pub(super) fn new(src: T, delay: Duration) -> Self {
        Self { src, delay }
    }

    #[inline]
    fn delay(&self) -> Duration {
        if self.delay > DURATION_ZERO {
            self.delay
        } else {
            DURATION_ZERO
        }
    }
}

impl<T: Animation> BaseAnimation for Delay<T> {
    type Item = T::Item;
    #[inline]
    fn duration(&self) -> Option<Duration> {
        self.src.duration().map(|d| self.delay() + d)
    }

    #[inline]
    fn animate(&self, elapsed: Duration) -> Self::Item {
        let delay = self.delay();
        let elapsed = if elapsed > delay {
            elapsed - delay
        } else {
            DURATION_ZERO
        };
        self.src.animate(elapsed)
    }
}
