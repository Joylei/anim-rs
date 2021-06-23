// anim
//
// A framework independent animation library for rust, works nicely with Iced and the others
// Copyright: 2021, Joylei <leingliu@gmail.com>
// License: MIT

use super::{
    animation::{Animation, BaseAnimation, Boxed, IsFinished},
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
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        self == &Status::Idle
    }
    /// is animation in progress?
    #[inline(always)]
    pub fn is_animating(&self) -> bool {
        self == &Status::Animating
    }
    /// is animation paused?
    #[inline(always)]
    pub fn is_paused(&self) -> bool {
        self == &Status::Paused
    }
    /// is animation completed?
    #[inline(always)]
    pub fn is_completed(&self) -> bool {
        self == &Status::Completed
    }
}

/// animation state
#[derive(Debug)]
enum State {
    /// animations not yet run
    Idle,
    /// animation is in progress
    Animating {
        /// current animation begin/recovery at
        time: Instant,
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
pub struct Timeline<T> {
    id: usize,
    animation: Boxed<T>, // it's not easy to use if not boxed
    state: State,
}

impl<T> Timeline<T> {
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
        }
    }

    /// the unique id of your animation
    #[inline]
    pub fn id(&self) -> TimelineId {
        TimelineId(self.id)
    }

    /// start your animation; if it's not completed yet, restart it
    #[inline]
    pub fn begin(&mut self) {
        let now = Instant::now();
        self.state = State::Animating {
            time: now,
            elapsed: None,
        }
    }

    /// stop your animation
    #[inline]
    pub fn stop(&mut self) {
        match self.state {
            State::Idle | State::Completed { .. } => {}
            State::Animating { time, elapsed } => {
                let elapsed = if let Some(elapsed) = elapsed {
                    elapsed + time.elapsed()
                } else {
                    time.elapsed()
                };
                self.state = State::Completed {
                    elapsed: Some(elapsed),
                }
            }
            State::Paused { elapsed } => self.state = State::Completed { elapsed },
        }
    }

    /// pause your animation only if it's animating
    #[inline]
    pub fn pause(&mut self) {
        match self.state {
            State::Animating { time, elapsed } => {
                let elapsed = if let Some(elapsed) = elapsed {
                    elapsed + time.elapsed()
                } else {
                    time.elapsed()
                };
                self.state = State::Paused {
                    elapsed: Some(elapsed),
                };
            }
            _ => {}
        }
    }

    /// continue your animation if it was paused, otherwise start new animation
    #[inline]
    pub fn resume(&mut self) {
        match self.state {
            State::Paused { elapsed } => {
                self.state = State::Animating {
                    time: Instant::now(),
                    elapsed,
                };
                return;
            }
            _ => self.begin(),
        }
    }

    /// if animation was stopped, it might keep its progress, you can clear it by this method
    #[inline]
    pub fn reset(&mut self) {
        match self.state {
            State::Completed { .. } => {
                self.state = State::Completed { elapsed: None };
            }
            _ => {}
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
    pub fn value(&self) -> T {
        match self.state {
            State::Idle => self.animation.animate(DURATION_ZERO),
            State::Animating { time, elapsed } => {
                let elapsed = if let Some(elapsed) = elapsed {
                    elapsed + time.elapsed()
                } else {
                    time.elapsed()
                };
                self.animation.animate(elapsed)
            }
            State::Paused { elapsed } => self.animation.animate(elapsed.unwrap_or(DURATION_ZERO)),
            State::Completed { elapsed, .. } => {
                if let Some(elapsed) = elapsed {
                    self.animation.animate(elapsed)
                } else {
                    self.animation.animate(DURATION_ZERO)
                }
            }
        }
    }

    /// update the timeline
    #[inline]
    pub fn update(&mut self) -> Status {
        self.update_with_time(Instant::now())
    }

    /// update the timeline
    pub fn update_with_time(&mut self, now: Instant) -> Status {
        match self.state {
            State::Idle => Status::Idle,
            State::Animating { time, elapsed } => {
                // accumulated time
                let elapsed = if let Some(elapsed) = elapsed {
                    elapsed + (now - time)
                } else {
                    now - time
                };
                if self.animation.is_finished(elapsed) {
                    self.state = State::Completed {
                        elapsed: Some(elapsed),
                    };
                    return Status::Completed;
                }
                Status::Animating
            }
            State::Paused { .. } => Status::Paused,
            State::Completed { .. } => Status::Completed,
        }
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
