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
    duration: Option<Duration>,
}

impl<T: Animation> Repeat<T> {
    #[inline(always)]
    pub(super) fn new(src: T, repeat: RepeatBehavior) -> Self {
        let duration = src.duration().and_then(|duration| {
            debug_assert!(duration >= DURATION_ZERO);
            if duration == DURATION_ZERO {
                return Some(DURATION_ZERO);
            }
            match repeat {
                RepeatBehavior::Count(count) => Some(duration.mul_f32(count)),
                RepeatBehavior::Forever => None,
            }
        });
        Self {
            src,
            repeat,
            duration,
        }
    }
}

impl<T: Animation> BaseAnimation for Repeat<T> {
    type Item = T::Item;
    #[inline(always)]
    fn duration(&self) -> Option<Duration> {
        self.duration
    }

    #[inline]
    fn animate(&self, mut elapsed: Duration) -> Self::Item {
        let simple_duration = match self.src.duration() {
            Some(duration) => duration,
            None => {
                return self.src.animate(elapsed);
            }
        };

        if let Some(duration) = self.duration {
            if elapsed > duration {
                elapsed = duration;
            }
        }

        let time = elapsed.as_secs_f64() / simple_duration.as_secs_f64();
        let count = time.floor();
        let mut time = time - count;
        if count > 0.0 && time == 0.0 {
            time = 1.0
        };
        self.src.animate(simple_duration.mul_f64(time))
    }
}
