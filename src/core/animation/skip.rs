// anim
//
// An animation library, works nicely with Iced and the others
// Copyright: 2021, Joylei <leingliu@gmail.com>
// License: MIT

use super::{Animation, BaseAnimation};
use crate::core::DURATION_ZERO;
use std::time::Duration;

/// always bypass specified time
#[derive(Debug, Clone)]
pub struct Skip<T: Animation> {
    src: T,
    progress: Duration,
}

impl<T: Animation> Skip<T> {
    pub(super) fn new(src: T, progress: Duration) -> Self {
        if progress < DURATION_ZERO {
            panic!("progress must be >=0");
        }
        Self { src, progress }
    }
}

impl<T: Animation> BaseAnimation for Skip<T> {
    type Item = T::Item;
    #[inline]
    fn duration(&self) -> Option<Duration> {
        debug_assert!(self.progress >= DURATION_ZERO);
        self.src.duration().map(|d| {
            if d > self.progress {
                d - self.progress
            } else {
                DURATION_ZERO
            }
        })
    }

    #[inline]
    fn animate(&self, elapsed: Duration) -> Self::Item {
        debug_assert!(self.progress >= DURATION_ZERO);
        let elapsed = self.progress + elapsed;
        self.src.animate(elapsed)
    }
}
