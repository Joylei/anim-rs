use std::time::Duration;

use crate::{Animatable, DEFAULT_ANIMATION_DURATION, DURATION_ZERO};

use super::BaseAnimation;

#[derive(Debug, Clone, Copy)]
pub enum KeyTime {
    Duration(Duration),
    Percent(f32),
}

impl Default for KeyTime {
    #[inline]
    fn default() -> Self {
        KeyTime::Duration(DURATION_ZERO)
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct KeyFrame<T> {
    pub value: T,
    pub key_time: KeyTime,
}

#[derive(Debug, Clone, Default)]
struct KeyFrameInner<T> {
    value: T,
    key_time: Duration,
}

impl<T> KeyFrameInner<T> {
    fn cvt_from(src: KeyFrame<T>, duration: &Duration) -> Option<Self> {
        match src.key_time {
            KeyTime::Duration(duration) => Some(KeyFrameInner {
                value: src.value,
                key_time: duration,
            }),
            KeyTime::Percent(percent) => {
                // filter out invalid values
                assert!(percent >= 0.0 && percent <= 1.0);
                Some(KeyFrameInner {
                    value: src.value,
                    key_time: duration.mul_f32(percent),
                })
            }
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct KeyFrameAnimation<T> {
    key_frames: Vec<KeyFrameInner<T>>,
    duration: Duration,
}

impl<T: Animatable> KeyFrameAnimation<T> {
    #[inline]
    pub(super) fn builder(key_frames: Vec<KeyFrame<T>>) -> Builder<T> {
        Builder { key_frames }
    }
}

impl<T: Animatable> BaseAnimation for KeyFrameAnimation<T> {
    type Item = T;

    #[inline]
    fn duration(&self) -> Option<Duration> {
        Some(self.duration)
    }

    fn animate(&self, elapsed: Duration) -> Self::Item {
        if elapsed < self.duration {
            for (last, item) in self.key_frames.iter().zip((&self.key_frames[1..]).iter()) {
                if item.key_time <= elapsed {
                    continue;
                }
                let delta = elapsed - last.key_time;
                let total = item.key_time - last.key_time;
                let time = delta.as_secs_f64() / total.as_secs_f64();
                return last.value.animate(&item.value, time);
            }
        }
        let item = self.key_frames.last().unwrap();
        item.value.clone()
    }
}

pub struct Builder<T: Animatable> {
    key_frames: Vec<KeyFrame<T>>,
}

impl<T: Animatable> Builder<T> {
    #[inline]
    pub fn push(mut self, item: KeyFrame<T>) -> Self {
        self.key_frames.push(item);
        self
    }

    pub fn build(self) -> KeyFrameAnimation<T> {
        //find max duration, so we can sort frames later
        let max_duration = self
            .key_frames
            .iter()
            .filter_map(|v| match v.key_time {
                KeyTime::Duration(duration) => Some(duration),
                KeyTime::Percent(_) => None,
            })
            .max()
            .or_else(|| Some(DEFAULT_ANIMATION_DURATION))
            .unwrap();

        //sort key frames
        let mut key_frames: Vec<_> = self
            .key_frames
            .into_iter()
            .filter_map(|frame| KeyFrameInner::cvt_from(frame, &max_duration))
            .collect();
        assert!(key_frames.len() > 0);
        key_frames.sort_by_key(|x| x.key_time);
        KeyFrameAnimation {
            key_frames,
            duration: max_duration,
        }
    }
}
