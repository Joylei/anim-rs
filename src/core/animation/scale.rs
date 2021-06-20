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
pub struct Scale<T: Animation> {
    src: T,
    scale: f64,
}

impl<T: Animation> Scale<T> {
    #[inline(always)]
    pub(super) fn new(src: T, scale: f64) -> Self {
        assert!(scale >= 0.0);
        Self { src, scale }
    }
}

impl<T: Animation> BaseAnimation for Scale<T> {
    type Item = T::Item;
    #[inline(always)]
    fn duration(&self) -> Option<Duration> {
        self.src.duration().map(|duration| {
            if duration == DURATION_ZERO || self.scale == 0.0 {
                return DURATION_ZERO;
            }
            duration.div_f64(self.scale)
        })
    }

    #[inline]
    fn animate(&self, elapsed: Duration) -> Self::Item {
        if self.scale == 0.0 {
            return self.src.animate(DURATION_ZERO);
        }
        let elapsed = elapsed.div_f64(self.scale);
        self.src.animate(elapsed)
    }
}
