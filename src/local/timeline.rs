// anim
//
// An animation library, works nicely with Iced and the others
// Copyright: 2021, Joylei <leingliu@gmail.com>
// License: MIT

use crate::core::timeline::Status;

pub(crate) trait TimelineEx<T> {
    fn status(&self) -> Status;
    fn value(&self) -> T;
    fn begin(&self);
    fn stop(&self);
    fn pause(&self);
    fn resume(&self);
    fn reset(&self);
}

/// thread local specialized timeline
pub struct Timeline<T>(Box<dyn TimelineEx<T>>);

impl<T> Timeline<T> {
    #[inline]
    pub(crate) fn new<E: TimelineEx<T> + 'static>(e: E) -> Self {
        Self(Box::new(e))
    }

    /// timeline status
    ///
    /// see [`crate::timeline::Status`]
    #[inline]
    pub fn status(&self) -> Status {
        self.0.status()
    }
    /// current animated value
    #[inline]
    pub fn value(&self) -> T {
        self.0.value()
    }

    /// start the timeline
    #[inline]
    pub fn begin(&mut self) {
        self.0.begin()
    }

    /// stop the timeline
    #[inline]
    pub fn stop(&mut self) {
        self.0.stop()
    }

    /// pause the timeline
    #[inline]
    pub fn pause(&mut self) {
        self.0.pause()
    }

    /// pause the timeline
    #[inline]
    pub fn resume(&mut self) {
        self.0.resume()
    }

    /// reset your animation if it's completed
    #[inline]
    pub fn reset(&mut self) {
        self.0.resume()
    }
}

impl<T> TimelineEx<T> for Timeline<T> {
    #[inline]
    fn status(&self) -> Status {
        self.0.status()
    }

    #[inline]
    fn value(&self) -> T {
        self.0.value()
    }

    #[inline]
    fn begin(&self) {
        self.0.begin()
    }

    #[inline]
    fn stop(&self) {
        self.0.stop()
    }

    #[inline]
    fn pause(&self) {
        self.0.pause()
    }

    #[inline]
    fn resume(&self) {
        self.0.resume()
    }

    #[inline]
    fn reset(&self) {
        self.0.resume()
    }
}
