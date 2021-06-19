// anim
//
// A framework independent animation library for rust, works nicely with Iced and the others
// Copyright: 2021, Joylei <leingliu@gmail.com>
// License: MIT

use super::BaseAnimation;
use crate::{
    core::{Animatable, Options, RepeatBehavior},
    DURATION_ZERO,
};
use std::time::Duration;

/// primitive animation which is built from [`Options`]
#[derive(Debug, Clone)]
pub struct Primitive<T: Animatable> {
    opt: Options<T>,
    duration: Option<Duration>,
}

impl<T: Animatable> Primitive<T> {
    #[inline(always)]
    pub(crate) fn new(opt: Options<T>) -> Self {
        let duration = if opt.duration <= DURATION_ZERO {
            Some(DURATION_ZERO)
        } else {
            match opt.repeat {
                RepeatBehavior::Count(count) => {
                    let duration = if count > 0.0 {
                        opt.duration.mul_f32(count)
                    } else {
                        DURATION_ZERO
                    };
                    Some(duration)
                }
                RepeatBehavior::Forever => None,
            }
        };
        Self { opt, duration }
    }
}

impl<T: Animatable> BaseAnimation for Primitive<T> {
    type Item = T;

    #[inline(always)]
    fn duration(&self) -> Option<Duration> {
        self.duration
    }

    fn animate(&self, mut elapsed: Duration) -> Self::Item {
        if let Some(duration) = self.duration() {
            // opt.duration<=0 || repeat count <=0
            if duration == DURATION_ZERO {
                return self.opt.from.clone();
            }
            //apply repeat limit
            if elapsed > duration {
                elapsed = duration;
            }
        }

        // calc normalized time
        let time = elapsed.as_secs_f64() / self.opt.duration.as_secs_f64();
        let count = time.floor();
        let mut time = time - count;
        if count > 0.0 && time == 0.0 {
            time = 1.0;
        }
        time = self.opt.easing.ease(time);
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
