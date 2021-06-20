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
    #[inline]
    pub(crate) fn new(opt: Options<T>) -> Self {
        let duration = {
            if opt.duration == DURATION_ZERO {
                Some(DURATION_ZERO)
            } else {
                match opt.repeat {
                    RepeatBehavior::Count(count) => Some(if count > 0.0 {
                        opt.duration.mul_f32(count)
                    } else {
                        DURATION_ZERO
                    }),
                    RepeatBehavior::Forever => None,
                }
            }
        };
        Self { opt, duration }
    }
}

impl<T: Animatable> BaseAnimation for Primitive<T> {
    type Item = T;

    #[inline(always)]
    fn duration(&self) -> Option<Duration> {
        if let Some(mut duration) = self.duration {
            //apply delay
            if let Some(delay) = self.opt.delay {
                duration += delay;
            }
            //apply skip
            if let Some(skip) = self.opt.skip {
                if duration > skip {
                    duration -= skip
                } else {
                    duration = DURATION_ZERO;
                }
            }
            Some(duration)
        } else {
            None
        }
    }

    fn animate(&self, mut elapsed: Duration) -> Self::Item {
        //apply skip
        if let Some(skip) = self.opt.skip {
            elapsed += skip;
        }
        //apply delay
        if let Some(delay) = self.opt.delay {
            if elapsed > delay {
                elapsed -= delay;
            } else {
                elapsed = DURATION_ZERO;
            }
        }

        // TODO: optimize for T:Eq
        if let Some(duration) = self.duration {
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
