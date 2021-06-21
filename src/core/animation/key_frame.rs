use crate::{easing, Animatable, DEFAULT_ANIMATION_DURATION, DURATION_ZERO};
use std::fmt;
use std::time::Duration;

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

impl From<Duration> for KeyTime {
    #[inline]
    fn from(duration: Duration) -> Self {
        Self::Duration(duration)
    }
}

impl From<f32> for KeyTime {
    #[inline]
    fn from(percent: f32) -> Self {
        assert!(percent >= 0.0 && percent <= 1.0);
        Self::Percent(percent)
    }
}

pub struct KeyFrame<T> {
    pub value: T,
    pub key_time: KeyTime,
    easing: Box<dyn easing::Function>,
}

impl<T> KeyFrame<T> {
    #[inline]
    pub fn new(value: T) -> Self {
        Self {
            value,
            key_time: DURATION_ZERO.into(),
            easing: Box::new(easing::linear()),
        }
    }

    #[inline]
    pub fn new_with_key_time(value: T, key_time: KeyTime) -> Self {
        Self {
            value,
            key_time,
            easing: Box::new(easing::linear()),
        }
    }

    #[inline]
    pub fn value(mut self, value: T) -> Self {
        self.value = value;
        self
    }

    #[inline]
    pub fn key_time(mut self, key_time: KeyTime) -> Self {
        self.key_time = key_time;
        self
    }

    /// panics if percent<0 or percent>1
    #[inline]
    pub fn by_percentage(mut self, percent: f32) -> Self {
        self.key_time = percent.into();
        self
    }

    #[inline]
    pub fn by_duration(mut self, duration: Duration) -> Self {
        self.key_time = duration.into();
        self
    }

    #[inline]
    pub fn easing(mut self, func: impl easing::Function + Clone + 'static) -> Self {
        self.easing = Box::new(func);
        self
    }
}

impl<T: Default> Default for KeyFrame<T> {
    #[inline]
    fn default() -> Self {
        Self {
            value: Default::default(),
            key_time: Default::default(),
            easing: Box::new(easing::linear()),
        }
    }
}

impl<T: Clone> Clone for KeyFrame<T> {
    fn clone(&self) -> Self {
        Self {
            value: self.value.clone(),
            key_time: self.key_time.clone(),
            easing: dyn_clone::clone_box(&*self.easing),
        }
    }
}

impl<T: fmt::Debug> fmt::Debug for KeyFrame<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("KeyFrame")
            .field("value", &self.value)
            .field("key_time", &self.key_time)
            .field("easing", &"???")
            .finish()
    }
}

struct KeyFrameInner<T> {
    value: T,
    key_time: Duration,
    easing: Box<dyn easing::Function>,
}

impl<T> KeyFrameInner<T> {
    fn cvt_from(src: KeyFrame<T>, duration: &Duration) -> Option<Self> {
        match src.key_time {
            KeyTime::Duration(duration) => Some(KeyFrameInner {
                value: src.value,
                key_time: duration,
                easing: src.easing,
            }),
            KeyTime::Percent(percent) => {
                // filter out invalid values
                assert!(percent >= 0.0 && percent <= 1.0);
                Some(KeyFrameInner {
                    value: src.value,
                    key_time: duration.mul_f32(percent),
                    easing: src.easing,
                })
            }
        }
    }
}

impl<T: Clone> Clone for KeyFrameInner<T> {
    fn clone(&self) -> Self {
        Self {
            value: self.value.clone(),
            key_time: self.key_time.clone(),
            easing: dyn_clone::clone_box(&*self.easing),
        }
    }
}

impl<T: fmt::Debug> fmt::Debug for KeyFrameInner<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("KeyFrame")
            .field("value", &self.value)
            .field("key_time", &self.key_time)
            .field("easing", &"???")
            .finish()
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
            let mut last = None;
            for item in self.key_frames.iter() {
                if item.key_time <= elapsed {
                    last = Some(item);
                    continue;
                }
                if let Some(last) = last {
                    let delta = elapsed - last.key_time;
                    let total = item.key_time - last.key_time;
                    let time = delta.as_secs_f64() / total.as_secs_f64();
                    let time = item.easing.ease(time);
                    return last.value.animate(&item.value, time);
                } else {
                    return item.value.clone();
                }
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
    #[allow(unused)]
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

        dbg!(max_duration);

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
