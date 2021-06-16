use super::easing;
use super::Animatable;
use std::fmt;
use std::fmt::Debug;
use std::sync::atomic::AtomicUsize;
use std::time::{Duration, Instant};

/// unique id
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TimelineId(usize);

/// repeat behavior for your animation
#[derive(Debug, Clone)]
pub enum RepeatBehavior {
    /// repeat limited times, default 1
    Count(usize),
    /// never end
    Forever,
}

impl Default for RepeatBehavior {
    #[inline]
    fn default() -> Self {
        RepeatBehavior::Count(1)
    }
}

/// Timeline options
pub struct Options<T: Animatable> {
    from: T,
    to: T,
    auto_reverse: bool,
    begin_time: std::option::Option<Duration>,
    //deceleration_ratio: Option<f32>,
    duration: Duration,
    repeat: RepeatBehavior,
    easing_func: Box<dyn easing::Function>,
}

impl<T: Animatable> Options<T> {
    /// create Options
    #[inline]
    pub fn new(from: T, to: T) -> Self {
        Self {
            from,
            to,
            auto_reverse: false,
            begin_time: None,
            //deceleration_ratio: None,
            duration: Duration::from_millis(1000),
            repeat: RepeatBehavior::Count(1),
            easing_func: Box::new(easing::cubic_ease()),
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

    /// auto reverse animation when completed
    #[inline]
    pub fn auto_reverse(mut self, auto_reverse: bool) -> Self {
        self.auto_reverse = auto_reverse;
        self
    }

    /// animation begin time
    #[inline]
    pub fn begin_time(mut self, begin_time: Duration) -> Self {
        self.begin_time = Some(begin_time);
        self
    }

    // #[inline]
    // pub fn deceleration_ratio(mut self, ratio: f32) -> Self {
    //     self.deceleration_ratio = Some(ratio);
    //     self
    // }

    /// animation duration
    #[inline]
    pub fn duration(mut self, duration: Duration) -> Self {
        self.duration = duration;
        self
    }

    /// repeat policy
    #[inline]
    pub fn repeat(mut self, behavior: RepeatBehavior) -> Self {
        self.repeat = behavior;
        self
    }

    /// loops forever
    /// see [`Options::repeat()`]
    #[inline]
    pub fn forever(mut self) -> Self {
        self.repeat = RepeatBehavior::Forever;
        self
    }

    /// loops for specified times
    ///
    /// see [`Options::repeat()`]
    #[inline]
    pub fn times(mut self, count: usize) -> Self {
        if count > 0 {
            self.repeat = RepeatBehavior::Count(count);
        }
        self
    }

    /// loops until specified time
    /// see [`Options::repeat()`]
    // #[inline]
    // pub fn until(mut self, when: Instant) -> Self {
    //     if when > Instant::now() {
    //         self.repeat = RepeatBehavior::Until(when);
    //     }
    //     self
    // }

    /// set ease function
    #[inline]
    pub fn easing(mut self, func: impl easing::Function + 'static) -> Self {
        self.easing_func = Box::new(func);
        self
    }
}

impl<T: Animatable + Default> Default for Options<T> {
    #[inline]
    fn default() -> Self {
        Self::new(Default::default(), Default::default())
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
            .field("easing", &"..")
            .finish()
    }
}

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

/// animation state
#[derive(Debug)]
pub(crate) enum State {
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

/// represents your animation
#[derive(Debug)]
pub struct Timeline<T: Animatable> {
    id: usize,
    opt: Options<T>,
    state: State,
}

impl<T: Animatable> Timeline<T> {
    /// construct your animation
    #[inline]
    pub fn new(opt: Options<T>) -> Self {
        Self {
            id: ID_GEN.fetch_add(1, std::sync::atomic::Ordering::AcqRel),
            opt,
            state: State::Idle,
        }
    }

    /// the unique id of your animation
    #[inline]
    pub fn id(&self) -> TimelineId {
        TimelineId(self.id)
    }

    // #[inline]
    // pub fn schedule_at(&mut self, when: Instant) {
    //     self.state = State::Scheduled { time: when };
    // }

    /// start your animation
    #[inline]
    pub fn begin(&mut self) {
        let now = Instant::now();
        match self.state {
            State::Paused { elapsed } => {
                //recovery
                self.state = State::Animating { time: now, elapsed }
            }

            _ => {
                // restart
                self.state = State::Animating {
                    time: now,
                    elapsed: self.opt.begin_time,
                }
            }
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
            State::Idle | State::Paused { .. } | State::Completed { .. } => {}
            State::Animating { time, elapsed } => {
                //TODO: check limit count
                let elapsed = if let Some(elapsed) = elapsed {
                    elapsed + time.elapsed()
                } else {
                    time.elapsed()
                };
                self.state = State::Paused {
                    elapsed: Some(elapsed),
                };
            }
        }
    }

    /// continue your animation if it was paused, otherwise start new animation
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

    /// apply loop count limit
    fn limit_count(&self, time: f64, auto_reverse: bool) -> f64 {
        match self.opt.repeat {
            RepeatBehavior::Count(limit) => {
                let loop_count = if auto_reverse { time / 2.0 } else { time };
                if loop_count >= limit as f64 {
                    return 0.0;
                }
            }
            RepeatBehavior::Forever => {}
        }
        if auto_reverse {
            time % 2.0
        } else {
            time - time.floor()
        }
    }

    /// generates output values based on its timing progress
    fn animate(&self, elapsed: Duration) -> T {
        let time = elapsed.as_secs_f64() / (self.opt.duration.as_secs_f64() + f64::EPSILON);
        if self.opt.auto_reverse {
            let time = self.limit_count(time, true);
            if time > 1.0 {
                //reverse
                self.opt.to.animate(&self.opt.from, time - 1.0)
            } else {
                self.opt.from.animate(&self.opt.to, time)
            }
        } else {
            let time = self.limit_count(time, false);
            self.opt.from.animate(&self.opt.to, time)
        }
    }

    /// the status of your animation
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
            State::Idle => self.opt.from.clone(),
            State::Animating { time, elapsed } => {
                let elapsed = if let Some(elapsed) = elapsed {
                    elapsed + time.elapsed()
                } else {
                    time.elapsed()
                };
                self.animate(elapsed)
            }
            State::Paused { elapsed } => self.animate(elapsed.unwrap_or(Duration::from_secs(0))),
            State::Completed { elapsed, .. } => {
                if let Some(elapsed) = elapsed {
                    self.animate(elapsed)
                } else {
                    if self.opt.auto_reverse {
                        self.opt.from.clone()
                    } else {
                        self.opt.to.clone()
                    }
                }
            }
        }
    }

    /// update the timeline
    pub fn update(&mut self, now: Instant) -> Status {
        self.on_tick(now);
        self.status()
    }

    fn on_tick(&mut self, now: Instant) {
        match self.state {
            State::Idle => {}
            State::Animating { time, elapsed } => {
                // accumulated time
                let elapsed = if let Some(elapsed) = elapsed {
                    elapsed + (now - time)
                } else {
                    now - time
                };
                let time = elapsed.as_secs_f64() / (self.opt.duration.as_secs_f64() + f64::EPSILON);
                match self.opt.repeat {
                    //check count limit
                    RepeatBehavior::Count(limit) => {
                        let loop_count = if self.opt.auto_reverse {
                            time / 2.0
                        } else {
                            time
                        };
                        if loop_count >= limit as f64 {
                            self.state = State::Completed {
                                elapsed: Some(elapsed),
                            };
                        }
                    }
                    RepeatBehavior::Forever => {}
                }
            }
            State::Paused { .. } => {}
            State::Completed { .. } => {}
        }
    }
}

/// build [`Timeline`]
pub trait Builder<T>
where
    T: Animatable,
{
    /// build [`Timeline`]
    fn build(self) -> Timeline<T>;
}

impl<T> Builder<T> for Options<T>
where
    T: Animatable,
{
    #[inline]
    fn build(self) -> Timeline<T> {
        Timeline::new(self)
    }
}

impl<T: Animatable> From<Options<T>> for Timeline<T> {
    #[inline]
    fn from(opt: Options<T>) -> Self {
        Timeline::new(opt)
    }
}
