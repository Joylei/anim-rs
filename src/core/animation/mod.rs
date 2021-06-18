// anim
//
// An animation library, works nicely with Iced and the others
// Copyright: 2021, Joylei <leingliu@gmail.com>
// License: MIT

mod boxed;
mod cache;
mod chain;
mod delay;
mod map;
mod parallel;
mod primitive;
mod skip;

use crate::{easing, Options, Timeline};

pub(crate) use boxed::Boxed;
pub(crate) use cache::Cache;
pub(crate) use chain::Chain;
pub(crate) use delay::Delay;
pub(crate) use map::Map;
pub(crate) use parallel::Parallel;
pub(crate) use primitive::Primitive;
pub(crate) use skip::Skip;
use std::time::Duration;

/// build a linear animation(x=t), with which you can get normalized time between 0-1
///
/// ## Example
/// ```rust,ignore
/// let timeline = linear(Duration::from_millis(2000))
///      .map(|t| if t>0.5 { true } else { false })
///      .begin_animation();
/// ```
#[inline]
pub fn linear(duration: Duration) -> impl Animation<Item = f32> + Clone {
    Options::new(0.0, 1.0)
        .auto_reverse(false)
        .easing(easing::linear())
        .duration(duration)
        .build()
}

/// A crate-private base trait,
pub trait BaseAnimation {
    /// animated value
    type Item;

    /// the animation lasts for how long; `None` means it's never finished
    fn duration(&self) -> Option<Duration>;

    /// outputs animated value based on the progressing time
    fn animate(&self, elapsed: Duration) -> Self::Item;
}

/// your animation, which outputs animated value based on the progressing time.
///
/// Simply, you can think it as an [`Iterator`]. The difference is that an [`Animation`]
/// always output some values.
pub trait Animation: BaseAnimation {
    /// always delay for specified time when start
    #[inline(always)]
    fn delay(self, delay: Duration) -> Delay<Self>
    where
        Self: Sized,
    {
        Delay::new(self, delay)
    }

    /// always delay for specified time when start
    #[inline(always)]
    fn delay_ms(self, millis: u64) -> Delay<Self>
    where
        Self: Sized,
    {
        Delay::new(self, Duration::from_millis(millis))
    }

    /// always move forward for specified time when start
    #[inline(always)]
    fn skip(self, progress: Duration) -> Skip<Self>
    where
        Self: Sized,
    {
        Skip::new(self, progress)
    }

    /// map from one type to another
    #[inline(always)]
    fn map<F, T>(self, f: F) -> Map<Self, F, T>
    where
        Self: Sized,
        F: Fn(Self::Item) -> T,
    {
        Map::new(self, f)
    }

    /// chain two animations
    #[inline(always)]
    fn chain<Other>(self, other: Other) -> Chain<Self, Other>
    where
        Self: Sized + 'static,
        Other: Animation<Item = Self::Item>,
    {
        Chain::new(self, other)
    }

    /// parallel animations, run at the same time until the longest one finishes
    #[inline(always)]
    fn parallel<Other>(self, other: Other) -> Parallel<Self, Other>
    where
        Self: Sized,
        Other: Animation,
    {
        Parallel::new(self, other)
    }

    /// parallel animations, run at the same time until the longest one finishes.
    ///
    /// alias for [`Animation::parallel()`]
    #[inline(always)]
    fn zip<Other>(self, other: Other) -> Parallel<Self, Other>
    where
        Self: Sized,
        Other: Animation,
    {
        Parallel::new(self, other)
    }

    /// caches animated value, reducing computing while not animating.
    /// you might want to use it at the end of the animation chains
    #[inline(always)]
    fn cached(self) -> Cache<Self>
    where
        Self: Sized,
        Self::Item: Clone,
    {
        Cache::new(self)
    }

    /// into boxed animation
    #[inline(always)]
    fn boxed(self) -> Boxed<Self::Item>
    where
        Self: Sized + 'static,
    {
        Boxed::new(self)
    }

    /// build [`Timeline`] and start animation
    #[inline(always)]
    fn begin_animation(self) -> Timeline<Self::Item>
    where
        Self: Sized + 'static,
        Self::Item: 'static,
    {
        let mut timeline = Timeline::new(self);
        timeline.begin();
        timeline
    }
}

impl<T: BaseAnimation> Animation for T {}

pub trait AnimationClone: Animation + Clone {}

impl<T: Animation + Clone> AnimationClone for T {}

// ----- private  -----

// helper
pub(crate) trait IsFinished {
    fn is_finished(&self, elapsed: Duration) -> bool;
}

impl<T: Animation> IsFinished for T {
    #[inline(always)]
    fn is_finished(&self, elapsed: Duration) -> bool {
        self.duration().map(|d| elapsed >= d).unwrap_or_default()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::core::{easing, Options, DURATION_ZERO};

    #[test]
    fn test_primitive() {
        let animation = Options::new(0.0, 1.0)
            .easing(easing::custom(|t| t))
            .duration(Duration::from_millis(1000))
            .auto_reverse(false)
            .build();

        let v = animation.animate(DURATION_ZERO);
        assert_eq!(v, 0.0);

        let v = animation.animate(Duration::from_millis(500));
        assert_eq!(v, 0.5);

        let v = animation.animate(Duration::from_millis(1000));
        assert_eq!(v, 1.0);

        let v = animation.animate(Duration::from_millis(1100));
        assert_eq!(v, 1.0);
    }

    #[test]
    fn test_reverse() {
        let animation = Options::new(0.0, 1.0)
            .easing(easing::custom(|t| t))
            .duration(Duration::from_millis(1000))
            .auto_reverse(true)
            .build();
        let v = animation.animate(DURATION_ZERO);
        assert_eq!(v, 0.0);

        let v = animation.animate(Duration::from_millis(250));
        assert_eq!(v, 0.5);

        let v = animation.animate(Duration::from_millis(500));
        assert_eq!(v, 1.0);

        let v = animation.animate(Duration::from_millis(750));
        assert_eq!(v, 0.5);

        let v = animation.animate(Duration::from_millis(1000));
        assert_eq!(v, 0.0);

        let v = animation.animate(Duration::from_millis(1100));
        assert_eq!(v, 0.0);
    }

    #[test]
    fn test_map() {
        let animation = Options::new(0.0, 1.0)
            .easing(easing::custom(|t| t))
            .duration(Duration::from_millis(1000))
            .auto_reverse(false)
            .build()
            .map(|v| v * 2.0);

        let v = animation.animate(DURATION_ZERO);
        assert_eq!(v, 0.0);

        let v = animation.animate(Duration::from_millis(500));
        assert_eq!(v, 1.0);

        let v = animation.animate(Duration::from_millis(1000));
        assert_eq!(v, 2.0);

        let v = animation.animate(Duration::from_millis(1100));
        assert_eq!(v, 2.0);
    }

    #[test]
    fn test_skip() {
        let animation = Options::new(0.0, 1.0)
            .easing(easing::custom(|t| t))
            .duration(Duration::from_millis(1000))
            .auto_reverse(false)
            .build()
            .skip(Duration::from_millis(500));

        let v = animation.animate(DURATION_ZERO);
        assert_eq!(v, 0.5);

        let v = animation.animate(Duration::from_millis(250));
        assert_eq!(v, 0.75);

        let v = animation.animate(Duration::from_millis(500));
        assert_eq!(v, 1.0);

        let v = animation.animate(Duration::from_millis(1000));
        assert_eq!(v, 1.0);
    }

    #[test]
    fn test_delay() {
        let animation = Options::new(0.0, 1.0)
            .easing(easing::custom(|t| t))
            .duration(Duration::from_millis(1000))
            .auto_reverse(false)
            .build()
            .delay(Duration::from_millis(500));

        let v = animation.animate(DURATION_ZERO);
        assert_eq!(v, 0.0);

        let v = animation.animate(Duration::from_millis(250));
        assert_eq!(v, 0.0);

        let v = animation.animate(Duration::from_millis(500));
        assert_eq!(v, 0.0);

        let v = animation.animate(Duration::from_millis(1000));
        assert_eq!(v, 0.5);

        let v = animation.animate(Duration::from_millis(1500));
        assert_eq!(v, 1.0);

        let v = animation.animate(Duration::from_millis(1600));
        assert_eq!(v, 1.0);
    }

    #[test]
    fn test_chain() {
        let animation = Options::new(0.0, 1.0)
            .easing(easing::custom(|t| t))
            .duration(Duration::from_millis(1000))
            .auto_reverse(false)
            .build()
            .chain(
                Options::new(0.0, 1.0)
                    .easing(easing::custom(|t| t))
                    .duration(Duration::from_millis(1000))
                    .auto_reverse(false)
                    .build(),
            );

        let v = animation.animate(DURATION_ZERO);
        assert_eq!(v, 0.0);

        let v = animation.animate(Duration::from_millis(250));
        assert_eq!(v, 0.25);

        let v = animation.animate(Duration::from_millis(500));
        assert_eq!(v, 0.5);

        //note: it's not continuous.
        // previous animation ended with value 1.0
        // next animation started with value 0.0
        let v = animation.animate(Duration::from_millis(1000));
        assert_eq!(v, 0.0);

        let v = animation.animate(Duration::from_millis(1500));
        assert_eq!(v, 0.5);

        let v = animation.animate(Duration::from_millis(2000));
        assert_eq!(v, 1.0);

        let v = animation.animate(Duration::from_millis(2100));
        assert_eq!(v, 1.0);
    }

    #[test]
    fn test_parallel() {
        let animation = Options::new(0.0, 1.0)
            .easing(easing::custom(|t| t))
            .duration(Duration::from_millis(1000))
            .auto_reverse(false)
            .build()
            .parallel(
                Options::new(0.0, 1.0)
                    .easing(easing::custom(|t| t))
                    .duration(Duration::from_millis(2000))
                    .auto_reverse(false)
                    .build(),
            );

        let v = animation.animate(Duration::from_millis(0));
        assert_eq!(v, (0.0, 0.0));

        let v = animation.animate(Duration::from_millis(500));
        assert_eq!(v, (0.5, 0.25));

        let v = animation.animate(Duration::from_millis(1000));
        assert_eq!(v, (1.0, 0.5));

        let v = animation.animate(Duration::from_millis(1500));
        assert_eq!(v, (1.0, 0.75));

        let v = animation.animate(Duration::from_millis(2000));
        assert_eq!(v, (1.0, 1.0));

        let v = animation.animate(Duration::from_millis(2300));
        assert_eq!(v, (1.0, 1.0));
    }
}
