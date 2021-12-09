// anim
//
// A framework independent animation library for rust, works nicely with Iced and the others
// Copyright: 2021, Joylei <leingliu@gmail.com>
// License: MIT

use super::{
    animation::{Animation, BaseAnimation, Boxed, IsFinished},
    clock::*,
    Animatable, Options, DURATION_ZERO,
};
use std::{
    fmt::Debug,
    sync::atomic::AtomicUsize,
    time::{Duration, Instant},
};
/// unique id
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TimelineId(usize);

/// animation status
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Status {
    /// animation not yet run
    Idle,
    /// animation is in progress
    Animating,
    /// animation was paused
    Paused,
    /// animation was completed
    Completed,
}

impl Status {
    /// is animation idle?
    #[inline]
    pub fn is_idle(&self) -> bool {
        self == &Status::Idle
    }
    /// is animation in progress?
    #[inline]
    pub fn is_animating(&self) -> bool {
        self == &Status::Animating
    }
    /// is animation paused?
    #[inline]
    pub fn is_paused(&self) -> bool {
        self == &Status::Paused
    }
    /// is animation completed?
    #[inline]
    pub fn is_completed(&self) -> bool {
        self == &Status::Completed
    }
}

/// animation state
#[derive(Debug)]
enum State<Time> {
    /// animations not yet run
    Idle,
    /// animation is in progress
    Animating {
        /// current animation begin/recovery at
        time: Time,
        /// elapsed time before above time
        elapsed: Option<Duration>,
    },
    /// animation was paused
    Paused { elapsed: Option<Duration> },
    /// animation was completed
    Completed { elapsed: Option<Duration> },
}

static ID_GEN: AtomicUsize = AtomicUsize::new(1);

/// control your animation
#[derive(Debug)]
pub struct Timeline<T, C: Clock = DefaultClock> {
    id: usize,
    animation: Boxed<T>, // it's not easy to use if not boxed
    state: State<C::Time>,
    clock: C,
}

impl<T, C: Clock> Timeline<T, C> {
    /// construct your animation
    #[inline]
    pub fn new<F>(animation: F) -> Self
    where
        F: Animation<Item = T> + 'static,
    {
        Self {
            id: ID_GEN.fetch_add(1, std::sync::atomic::Ordering::Relaxed),
            animation: Boxed::new(animation),
            state: State::Idle,
            clock: Default::default(),
        }
    }

    /// associated clock
    pub fn clock(&self) -> &C {
        &self.clock
    }

    /// associated clock
    pub fn clock_mut(&mut self) -> &mut C {
        &mut self.clock
    }

    /// the unique id of your animation
    #[inline]
    pub fn id(&self) -> TimelineId {
        TimelineId(self.id)
    }

    /// start your animation; if it's not completed yet, restart it
    #[inline]
    pub fn begin(&mut self) {
        let now = self.clock.now();
        self.state = State::Animating {
            time: now,
            elapsed: None,
        }
    }

    /// stop your animation
    #[inline]
    pub fn stop(&mut self) {
        match &mut self.state {
            State::Idle | State::Completed { .. } => {}
            State::Animating { time, elapsed } => {
                let duration = self.clock.now() - time.clone();
                let elapsed = elapsed.unwrap_or(DURATION_ZERO) + duration;
                self.state = State::Completed {
                    elapsed: Some(elapsed),
                }
            }
            State::Paused { elapsed } => {
                self.state = State::Completed {
                    elapsed: elapsed.take(),
                }
            }
        }
    }

    /// pause your animation only if it's animating
    #[inline]
    pub fn pause(&mut self) {
        if let State::Animating { time, elapsed } = &mut self.state {
            let duration = self.clock.now() - time.clone();
            let elapsed = elapsed.unwrap_or_default() + duration;
            self.state = State::Paused {
                elapsed: Some(elapsed),
            };
        }
    }

    /// continue your animation if it was paused, otherwise start new animation
    #[inline]
    pub fn resume(&mut self) {
        match self.state {
            State::Paused { elapsed } => {
                self.state = State::Animating {
                    time: self.clock.now(),
                    elapsed,
                };
            }
            _ => self.begin(),
        }
    }

    /// if animation was stopped, it might keep its progress, you can clear it by this method
    #[inline]
    pub fn reset(&mut self) {
        if let State::Completed { .. } = self.state {
            self.state = State::Completed { elapsed: None };
        }
    }

    /// the status of your animation
    #[inline]
    pub fn status(&self) -> Status {
        match self.state {
            State::Idle => Status::Idle,
            State::Animating { .. } => Status::Animating,
            State::Paused { .. } => Status::Paused,
            State::Completed { .. } => Status::Completed,
        }
    }

    /// the current value of your animation
    #[inline]
    pub fn value(&self) -> T {
        match &self.state {
            State::Idle => self.animation.animate(DURATION_ZERO),
            State::Animating { time, elapsed } => {
                let duration = self.clock.now() - time.clone();
                let elapsed = elapsed.unwrap_or_default() + duration;
                self.animation.animate(elapsed)
            }
            State::Paused { elapsed } => self.animation.animate(elapsed.unwrap_or(DURATION_ZERO)),
            State::Completed { elapsed, .. } => {
                if let Some(elapsed) = elapsed {
                    self.animation.animate(*elapsed)
                } else {
                    self.animation.animate(DURATION_ZERO)
                }
            }
        }
    }

    /// update the status of the timeline
    #[inline]
    pub fn update(&mut self) -> Status {
        match &mut self.state {
            State::Idle => Status::Idle,
            State::Animating { time, elapsed } => {
                let now = self.clock.now();
                // accumulated time
                let duration = elapsed.unwrap_or_default() + (now - time.clone());
                if self.animation.is_finished(duration) {
                    self.state = State::Completed {
                        elapsed: Some(duration),
                    };
                    return Status::Completed;
                }
                Status::Animating
            }
            State::Paused { .. } => Status::Paused,
            State::Completed { .. } => Status::Completed,
        }
    }

    /// update the timeline
    #[deprecated = "will be removed"]
    #[inline]
    pub fn update_with_time(&mut self, _now: Instant) -> Status {
        self.update()
    }
}

impl<T: Animation + 'static> From<T> for Timeline<T::Item> {
    #[inline]
    fn from(src: T) -> Self {
        let src = Boxed::new(src);
        Timeline::new(src)
    }
}

impl<T: Animatable + 'static> From<Options<T>> for Timeline<T> {
    #[inline]
    fn from(opt: Options<T>) -> Self {
        Timeline::new(opt.build())
    }
}
