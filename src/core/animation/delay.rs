// anim
//
// A framework independent animation library for rust, works nicely with Iced and the others
// Copyright: 2021, Joylei <leingliu@gmail.com>
// License: MIT

use super::{Animation, BaseAnimation};
use crate::core::DURATION_ZERO;
use std::time::Duration;

/// delay your animation for a specified time
#[derive(Debug, Clone)]
pub struct Delay<T: Animation> {
    src: T,
    delay: Duration,
}

impl<T: Animation> Delay<T> {
    #[inline(always)]
    pub(super) fn new(src: T, delay: Duration) -> Self {
        assert!(delay >= DURATION_ZERO);
        Self { src, delay }
    }
}

impl<T: Animation> BaseAnimation for Delay<T> {
    type Item = T::Item;
    #[inline(always)]
    fn duration(&self) -> Option<Duration> {
        debug_assert!(self.delay >= Duration::from_secs(0));
        self.src.duration().map(|d| self.delay + d)
    }

    #[inline(always)]
    fn animate(&self, elapsed: Duration) -> Self::Item {
        debug_assert!(self.delay >= Duration::from_secs(0));
        let elapsed = if elapsed > self.delay {
            elapsed - self.delay
        } else {
            DURATION_ZERO
        };
        self.src.animate(elapsed)
    }
}
