use crate::core::timeline::Status;
use std::marker::PhantomData;

pub(crate) trait TimelineEx<T> {
    fn status(&self) -> Status;
    fn value(&self) -> T;
    fn begin(&self);
    fn stop(&self);
    fn pause(&self);
    fn resume(&self);
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
}

impl<T: 'static> Timeline<T> {
    /// map from one type to another
    #[inline]
    pub fn map<D, F>(self, f: F) -> Timeline<D>
    where
        D: 'static,
        F: Fn(T, Status) -> D + 'static,
    {
        let src = MappedTimeline::new(self, f);
        src.into()
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
}

struct MappedTimeline<S, T, F> {
    src: Timeline<S>,
    f: F,
    _marker: PhantomData<(S, T)>,
}

impl<S, T, F> MappedTimeline<S, T, F>
where
    F: Fn(S, Status) -> T,
{
    #[inline]
    fn new(src: Timeline<S>, f: F) -> Self {
        Self {
            src,
            f,
            _marker: Default::default(),
        }
    }
}

impl<S, T, F> TimelineEx<T> for MappedTimeline<S, T, F>
where
    F: Fn(S, Status) -> T,
{
    #[inline]
    fn status(&self) -> Status {
        self.src.0.status()
    }
    #[inline]
    fn value(&self) -> T {
        let status = self.src.0.status();
        let value = self.src.0.value();
        (self.f)(value, status)
    }
    #[inline]
    fn begin(&self) {
        self.src.0.begin();
    }
    #[inline]
    fn stop(&self) {
        self.src.0.stop();
    }
    #[inline]
    fn pause(&self) {
        self.src.0.pause()
    }

    #[inline]
    fn resume(&self) {
        self.src.0.resume()
    }
}

impl<S, T, F> From<MappedTimeline<S, T, F>> for Timeline<T>
where
    S: 'static,
    T: 'static,
    F: Fn(S, Status) -> T + 'static,
{
    #[inline]
    fn from(src: MappedTimeline<S, T, F>) -> Self {
        Timeline::new(src)
    }
}
