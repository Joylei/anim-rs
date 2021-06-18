// anim
//
// A framework independent animation library for rust, works nicely with Iced and the others
// Copyright: 2021, Joylei <leingliu@gmail.com>
// License: MIT

use super::{Animation, BaseAnimation};
use crate::{core::RepeatBehavior, core::DURATION_ZERO};
use std::time::Duration;
/// repeat animations
#[derive(Debug, Clone)]
pub struct Repeat<T: Animation> {
    src: T,
    repeat: RepeatBehavior,
}

impl<T: Animation> Repeat<T> {
    #[inline(always)]
    pub(super) fn new(src: T, repeat: RepeatBehavior) -> Self {
        if let RepeatBehavior::Count(limit) = repeat {
            assert!(limit >= 1);
        }
        Self { src, repeat }
    }
}

impl<T: Animation> BaseAnimation for Repeat<T> {
    type Item = T::Item;
    #[inline]
    fn duration(&self) -> Option<Duration> {
        match self.repeat {
            RepeatBehavior::Count(count) => self.src.duration().map(|v| v * count),
            RepeatBehavior::Forever => {
                if let Some(v) = self.src.duration() {
                    if v == DURATION_ZERO {
                        return Some(DURATION_ZERO);
                    }
                }
                None
            }
        }
    }

    #[inline]
    fn animate(&self, elapsed: Duration) -> Self::Item {
        let duration = match self.src.duration() {
            Some(duration) => duration,
            None => {
                return self.src.animate(elapsed);
            }
        };
        let time = elapsed.as_secs_f64() / duration.as_secs_f64();
        let count = time.floor();
        if let RepeatBehavior::Count(limit) = self.repeat {
            if count as u32 >= limit {
                return self.src.animate(duration);
            }
        }
        let time = time - count;
        self.src.animate(duration.mul_f64(time))
    }
}
