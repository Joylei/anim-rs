// anim
//
// A framework independent animation library for rust, works nicely with Iced and the others
// Copyright: 2021, Joylei <leingliu@gmail.com>
// License: MIT

use super::{Animation, BaseAnimation};
use crate::core::DURATION_ZERO;
use std::time::Duration;
/// repeat animations
#[derive(Debug, Clone)]
pub struct Take<T: Animation> {
    src: T,
    duration: Duration,
}

impl<T: Animation> Take<T> {
    #[inline(always)]
    pub(super) fn new(src: T, duration: Duration) -> Self {
        Take { src, duration }
    }
}

impl<T: Animation> BaseAnimation for Take<T> {
    type Item = T::Item;
    #[inline(always)]
    fn duration(&self) -> Option<Duration> {
        if self.duration > DURATION_ZERO {
            if let Some(duration) = self.src.duration() {
                if self.duration >= duration {
                    return Some(duration);
                }
            }
        };

        Some(self.duration)
    }

    #[inline]
    fn animate(&self, elapsed: Duration) -> Self::Item {
        let duration = self.duration().unwrap();
        if elapsed > duration {
            self.src.animate(duration)
        } else {
            self.src.animate(elapsed)
        }
    }
}
