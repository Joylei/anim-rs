// anim
//
// A framework independent animation library for rust, works nicely with Iced and the others
// Copyright: 2021, Joylei <leingliu@gmail.com>
// License: MIT

use super::{Animation, BaseAnimation};
use crate::core::DURATION_ZERO;
use std::time::Duration;

/// seek progress of current animation, only keep the remaining part
#[derive(Clone, Copy)]
pub enum SeekFrom {
    /// from the beginning
    FromBegin(Duration),
    /// from the end
    FromEnd(Duration),
    /// by percentage, negative value means from the end
    Percentage(f32),
}

/// always bypass specified time
#[derive(Debug, Clone)]
pub struct Seek<T: Animation> {
    src: T,
    progress: Duration,
}

impl<T: Animation> Seek<T> {
    pub(super) fn new(src: T, seek: SeekFrom) -> Self {
        let progress = match seek {
            SeekFrom::FromBegin(progress) => progress,
            SeekFrom::FromEnd(progress) => {
                if let Some(duration) = src.duration() {
                    if duration > progress {
                        duration - progress
                    } else {
                        DURATION_ZERO
                    }
                } else {
                    panic!("cannot seek from end for indefinite animation");
                }
            }
            SeekFrom::Percentage(percent) => {
                if let Some(duration) = src.duration() {
                    if percent < 0.0 {
                        duration.mul_f32(1.0 + percent)
                    } else {
                        duration.mul_f32(percent)
                    }
                } else {
                    panic!("cannot seek by percentage for indefinite animation");
                }
            }
        };
        Self { src, progress }
    }
}

impl<T: Animation> BaseAnimation for Seek<T> {
    type Item = T::Item;
    #[inline]
    fn duration(&self) -> Option<Duration> {
        self.src.duration().map(|d| {
            if d > self.progress {
                d - self.progress
            } else {
                DURATION_ZERO
            }
        })
    }

    #[inline(always)]
    fn animate(&self, elapsed: Duration) -> Self::Item {
        let elapsed = self.progress + elapsed;
        self.src.animate(elapsed)
    }
}
