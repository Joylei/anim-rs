// anim
//
// A framework independent animation library for rust, works nicely with Iced and the others
// Copyright: 2021, Joylei <leingliu@gmail.com>
// License: MIT

use super::BaseAnimation;
use crate::core::{Animatable, Options, RepeatBehavior};
use std::time::Duration;

/// primitive animation which is built from [`Options`]
#[derive(Debug, Clone)]
pub struct Primitive<T: Animatable> {
    opt: Options<T>,
}

impl<T: Animatable> Primitive<T> {
    #[inline(always)]
    pub(crate) fn new(opt: Options<T>) -> Self {
        Self { opt }
    }
}

impl<T: Animatable> BaseAnimation for Primitive<T> {
    type Item = T;

    #[inline(always)]
    fn duration(&self) -> Option<Duration> {
        match self.opt.repeat {
            RepeatBehavior::Count(count) => Some(self.opt.duration * count),
            RepeatBehavior::Forever => None,
        }
    }

    fn animate(&self, elapsed: Duration) -> Self::Item {
        // calc normalized time
        let finished = self.duration().map(|d| elapsed >= d).unwrap_or_default();
        let time = if finished {
            if self.opt.auto_reverse {
                0.0
            } else {
                1.0
            }
        } else {
            let time = elapsed.as_secs_f64() / self.opt.duration.as_secs_f64();
            time - time.floor()
        };

        let time = self.opt.easing.ease(time);

        if self.opt.auto_reverse {
            if time > 0.5 {
                //reverse
                self.opt.to.animate(&self.opt.from, time * 2.0 - 1.0)
            } else {
                self.opt.from.animate(&self.opt.to, time * 2.0)
            }
        } else {
            self.opt.from.animate(&self.opt.to, time)
        }
    }
}
