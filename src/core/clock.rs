use std::{
    ops::Sub,
    time::{Duration, Instant},
};

/// [`Clock`] allow you to control the time
pub trait Clock: Default {
    /// represents the time
    type Time: Sub<Output = Duration> + Clone;

    /// current time
    fn now(&self) -> Self::Time;
}

/// a default implementation of [`Clock`]
#[derive(Debug, Default)]
pub struct DefaultClock;

impl Clock for DefaultClock {
    type Time = Instant;
    #[inline]
    fn now(&self) -> Instant {
        Instant::now()
    }
}
