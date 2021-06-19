// anim
//
// A framework independent animation library for rust, works nicely with Iced and the others
// Copyright: 2021, Joylei <leingliu@gmail.com>
// License: MIT

use crate::{
    core::{animation::Primitive, easing, Animatable},
    Animation, Timeline,
};
use std::{fmt, time::Duration};

/// how an [`crate::core::Animation`] repeats its simple duration
#[derive(Debug, Clone, Copy)]
pub enum RepeatBehavior {
    /// specifies the number of times the simple duration of a an [`crate::core::Animation`] plays. default 1.0
    Count(f32),
    /// The [`crate::core::Animation`] repeats indefinitely
    Forever,
}

impl Default for RepeatBehavior {
    #[inline]
    fn default() -> Self {
        RepeatBehavior::Count(1.0)
    }
}

/// options to build an [`crate::core::Animation`]
pub struct Options<T: Animatable> {
    pub(crate) from: T,
    pub(crate) to: T,
    pub(crate) auto_reverse: bool,
    pub(crate) begin_time: std::option::Option<Duration>,
    pub(crate) duration: Duration,
    pub(crate) repeat: RepeatBehavior,
    pub(crate) easing: Box<dyn easing::Function>,
}

impl<T: Animatable> Options<T> {
    /// create new [`Options`]
    #[inline]
    pub fn new(from: T, to: T) -> Self {
        Options {
            from,
            to,
            auto_reverse: false,
            begin_time: None,
            duration: Duration::from_millis(1000),
            repeat: Default::default(),
            easing: Box::new(easing::cubic_ease()),
        }
    }

    /// animation from value
    #[inline]
    pub fn from(mut self, value: T) -> Self {
        self.from = value;
        self
    }

    /// animation to value
    #[inline]
    pub fn to(mut self, value: T) -> Self {
        self.to = value;
        self
    }

    /// auto reverse animation when it reaches the end; default false.
    /// Note: it will not increase the duration or repeat times.
    ///
    /// auto_reverse | effect
    /// ------------- | -------------------
    /// false             | from -> to
    /// true              | from -> to -> from
    ///
    #[inline]
    pub fn auto_reverse(mut self, auto_reverse: bool) -> Self {
        self.auto_reverse = auto_reverse;
        self
    }

    /// animation begin time, same effect as [`crate::core::Animation::skip()`];
    /// take effect only once when the animation loops more than once.
    ///
    /// see [`Options::repeat()`] & [`crate::core::Animation::skip()`]
    #[inline]
    pub fn begin_time(mut self, begin_time: Duration) -> Self {
        self.begin_time = Some(begin_time);
        self
    }

    /// animation simple duration, this animation will last for how long if it plays once. default 1000ms.
    ///
    /// If [`Options::repeat()`] is specified, the animation might play more than once.
    #[inline]
    pub fn duration(mut self, duration: Duration) -> Self {
        self.duration = duration;
        self
    }

    /// repeat behavior
    #[inline]
    pub fn repeat(mut self, behavior: RepeatBehavior) -> Self {
        self.repeat = behavior;
        self
    }

    /// your [`crate::core::Animation`] repeats indefinitely
    ///
    /// see [`Options::repeat()`]
    #[inline]
    pub fn forever(mut self) -> Self {
        self.repeat = RepeatBehavior::Forever;
        self
    }

    /// your [`crate::core::Animation`] repeats for specified times
    ///
    /// see [`Options::repeat()`]
    ///
    /// panics if count<=0
    #[inline]
    pub fn times(mut self, count: f32) -> Self {
        assert!(count >= 0.0);
        self.repeat = RepeatBehavior::Count(count);
        self
    }

    /// set ease function
    #[inline]
    pub fn easing(self, func: impl easing::Function + Clone + 'static) -> Options<T> {
        Options {
            from: self.from,
            to: self.to,
            auto_reverse: self.auto_reverse,
            begin_time: self.begin_time,
            duration: self.duration,
            repeat: self.repeat,
            easing: Box::new(func),
        }
    }

    /// build [`Animation`]
    #[inline(always)]
    pub fn build(self) -> impl Animation<Item = T> + Clone {
        Primitive::new(self)
    }
}

impl<T: Animatable + 'static> Options<T> {
    /// build [`crate::core::Timeline`] and start animation
    #[inline]
    pub fn begin_animation(self) -> Timeline<T> {
        let mut timeline: Timeline<_> = self.into();
        timeline.begin();
        timeline
    }
}

impl<T: Animatable + fmt::Debug> fmt::Debug for Options<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Options")
            .field("from", &self.from)
            .field("to", &self.to)
            .field("auto_reverse", &self.auto_reverse)
            .field("begin_time", &self.begin_time)
            .field("duration", &self.duration)
            .field("repeat", &self.repeat)
            .field("easing", &"[easing function]")
            .finish()
    }
}

impl<T: Animatable> Clone for Options<T> {
    #[inline]
    fn clone(&self) -> Self {
        Self {
            from: self.from.clone(),
            to: self.to.clone(),
            auto_reverse: self.auto_reverse,
            begin_time: self.begin_time,
            duration: self.duration,
            repeat: self.repeat,
            easing: dyn_clone::clone_box(&*self.easing),
        }
    }
}
