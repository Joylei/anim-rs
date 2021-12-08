use super::BaseAnimation;
use crate::DURATION_ZERO;
use std::time::Duration;

/// like `Iterator`, but does not consume any element
///
/// built-in types that derives [`Cursor`]
/// - `Vec<T>`
/// - `[T]`
/// - `&[T]`
/// - `Box<T>` where `T:Cursor`
/// - `&T` where `T:Cursor`
pub trait Cursor {
    /// item of the cursor
    type Item;
    /// none means that it's infinite
    fn size(&self) -> Option<usize>;

    /// seek to specified element
    fn index(&self, n: usize) -> Self::Item;
}

impl<T: Clone> Cursor for [T] {
    type Item = T;
    #[inline]
    fn size(&self) -> Option<usize> {
        Some(self.len())
    }
    #[inline]
    fn index(&self, n: usize) -> T {
        self[n].to_owned()
    }
}

impl<T: Clone> Cursor for &[T] {
    type Item = T;
    #[inline]
    fn size(&self) -> Option<usize> {
        Some(self.len())
    }
    #[inline]
    fn index(&self, n: usize) -> T {
        self[n].to_owned()
    }
}

impl<T: Clone> Cursor for Vec<T> {
    type Item = T;
    #[inline]
    fn size(&self) -> Option<usize> {
        Some(self.len())
    }
    #[inline]
    fn index(&self, n: usize) -> T {
        self[n].to_owned()
    }
}

impl<T: Cursor> Cursor for &T {
    type Item = T::Item;
    #[inline]
    fn size(&self) -> Option<usize> {
        (**self).size()
    }
    #[inline]
    fn index(&self, n: usize) -> Self::Item {
        (**self).index(n)
    }
}

impl<T: Cursor> Cursor for Box<T> {
    type Item = T::Item;
    #[inline]
    fn size(&self) -> Option<usize> {
        (**self).size()
    }
    #[inline]
    fn index(&self, n: usize) -> Self::Item {
        (**self).index(n)
    }
}

struct Finite<T> {
    src: T,
}

impl<T> Cursor for Finite<T>
where
    T: ExactSizeIterator + Clone,
{
    type Item = <T as Iterator>::Item;

    #[inline]
    fn size(&self) -> Option<usize> {
        self.src.len().into()
    }

    #[inline]
    fn index(&self, n: usize) -> Self::Item {
        let mut src = self.src.clone();
        src.nth(n).unwrap()
    }
}

impl<T> From<T> for Finite<T>
where
    T: ExactSizeIterator + Clone,
{
    #[inline]
    fn from(src: T) -> Self {
        Finite { src }
    }
}

#[derive(Clone)]
pub struct Infinite<F: Fn(usize) -> T, T> {
    f: F,
}

impl<F, T> Infinite<F, T>
where
    F: Fn(usize) -> T,
{
    #[inline]
    pub(super) fn new(f: F) -> Self {
        Self { f }
    }
}

impl<F, T> Cursor for Infinite<F, T>
where
    F: Fn(usize) -> T,
{
    type Item = T;
    #[inline]
    fn size(&self) -> Option<usize> {
        None
    }
    #[inline]
    fn index(&self, n: usize) -> Self::Item {
        (self.f)(n)
    }
}

#[derive(Debug, Clone)]
pub struct StepAnimation<T: Cursor> {
    src: T,
    interval: Duration,
}

impl<T> StepAnimation<T>
where
    T: Cursor,
{
    /// create animation
    #[inline]
    pub(super) fn new(src: T) -> Self {
        Self {
            src,
            interval: DURATION_ZERO,
        }
    }

    /// set duration of the animation
    #[inline]
    pub fn interval(mut self, interval: Duration) -> Self {
        self.interval = interval;
        self
    }
}

impl<T> BaseAnimation for StepAnimation<T>
where
    T: Cursor,
{
    type Item = <T as Cursor>::Item;

    #[inline]
    fn duration(&self) -> Option<Duration> {
        if self.interval == DURATION_ZERO {
            return Some(DURATION_ZERO);
        }
        self.src
            .size()
            .map(|size| self.interval.mul_f64(size as f64))
    }

    #[inline]
    fn animate(&self, elapsed: Duration) -> Self::Item {
        let n = match self.duration() {
            Some(duration) if duration == DURATION_ZERO => 0,
            Some(duration) if elapsed >= duration => self.src.size().unwrap(),
            _ => {
                let n = elapsed.as_secs_f64() / self.interval.as_secs_f64();
                n as usize
            }
        };
        self.src.index(n)
    }
}
