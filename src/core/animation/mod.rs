// anim
//
// A framework independent animation library for rust, works nicely with Iced and the others
// Copyright: 2021, Joylei <leingliu@gmail.com>
// License: MIT

mod boxed;
mod cache;
mod chain;
mod delay;
mod key_frame;
mod map;
mod parallel;
mod primitive;
mod repeat;
mod scale;
mod seek;
mod step;
mod take;

use crate::{easing, Animatable, Options, RepeatBehavior, Timeline};

pub use self::key_frame::{KeyFrame, KeyTime};
pub use self::seek::SeekFrom;
pub use self::step::Cursor;
pub use self::step::StepAnimation;
use self::{scale::Scale, step::Infinite};
pub(crate) use boxed::Boxed;
pub(crate) use cache::Cache;
pub(crate) use chain::Chain;
pub(crate) use delay::Delay;
pub(crate) use key_frame::KeyFrameAnimation;
pub(crate) use map::Map;
pub(crate) use parallel::Parallel;
pub(crate) use primitive::Primitive;
pub(crate) use repeat::Repeat;
pub(crate) use seek::Seek;
use std::time::Duration;
pub(crate) use take::Take;

/// build a linear animation(x=t), with which you can get normalized time between 0-1
///
/// ## Example
/// ```rust
/// use std::time::Duration;
/// use anim::{Animation,builder::linear};
///
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

/// build a constant animation, which will output constant values
#[inline]
pub fn constant<T: Clone>(value: T, duration: Duration) -> impl Animation<Item = T> + Clone {
    Options::new(true, true)
        .duration(duration)
        .build()
        .map(move |_| value.clone())
}

/// build key frames animation
///
/// - requires at least one frame
/// - default duration is one second if not specified in any of the frames
#[inline]
pub fn key_frames<T: Animatable>(
    frames: impl Into<Vec<KeyFrame<T>>>,
) -> impl Animation<Item = T> + Clone {
    KeyFrameAnimation::builder(frames.into()).build()
}

/// infinite or finite steps
///
/// see [`Cursor`]
#[inline]
pub fn steps<T: Cursor>(src: T, interval: Duration) -> StepAnimation<T> {
    StepAnimation::new(src).interval(interval)
}

/// infinite steps
///
/// ## Example
/// ```rust
/// use std::time::Duration;
/// use anim::{Animation, builder::steps_infinite};
///
/// #[derive(Debug)]
/// enum Action {
///     Stand,
///     Step1,
///     Step2,
///     Run,   
/// }
///
/// let steps = steps_infinite(|i| {
///     if i == 0 {
///         return Action::Stand;
///      }
///      match (i-1) % 3 {
///           0 => Action::Step1,
///           1 => Action::Step2,
///            _ => Action::Run,
///       }
/// },Duration::from_millis(40));
/// let timeline = steps.begin_animation();
/// //...
/// ```
#[inline]
pub fn steps_infinite<F: Fn(usize) -> T, T>(
    f: F,
    interval: Duration,
) -> StepAnimation<Infinite<F, T>> {
    let src = Infinite::new(f);
    StepAnimation::new(src).interval(interval)
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
    /// always delay for specified time when play current animation; negative delay has no effect
    #[inline]
    fn delay(self, delay: Duration) -> Delay<Self>
    where
        Self: Sized,
    {
        Delay::new(self, delay)
    }

    /// always delay for specified time when play current animation
    #[inline]
    fn delay_ms(self, millis: u64) -> Delay<Self>
    where
        Self: Sized,
    {
        Delay::new(self, Duration::from_millis(millis))
    }

    /// always move forward for specified time when play current animation
    ///
    /// just a simple wrap on [`Animation::seek`]
    #[inline]
    fn skip(self, progress: Duration) -> Seek<Self>
    where
        Self: Sized,
    {
        Seek::new(self, SeekFrom::Begin(progress))
    }

    /// always move forward for specified time when play current animation
    ///
    /// ## panic
    /// - panics if percent < -1.0 or percent > 1.0
    /// - panics if current animation lasts indefinitely while seeking from end or by percent
    #[inline]
    fn seek(self, seek: SeekFrom) -> Seek<Self>
    where
        Self: Sized,
    {
        Seek::new(self, seek)
    }

    /// always move forward for specified time when play current animation
    ///
    /// just a simple wrap on [`Animation::seek`]
    ///
    /// ## panic
    /// - panics if percent < -1.0 or percent > 1.0
    /// - panics if current animation lasts indefinitely
    #[inline]
    fn seek_by(self, percent: f32) -> Seek<Self>
    where
        Self: Sized,
    {
        Seek::new(self, SeekFrom::Percent(percent))
    }

    /// map from one type to another
    #[inline]
    fn map<F, T>(self, f: F) -> Map<Self, F, T>
    where
        Self: Sized,
        F: Fn(Self::Item) -> T,
    {
        Map::new(self, f)
    }

    /// chain two animations, play in the chained order
    #[inline]
    fn chain<Other>(self, other: Other) -> Chain<Self, Other>
    where
        Self: Sized,
        Other: Animation<Item = Self::Item>,
    {
        Chain::new(self, other)
    }

    /// take specified duration
    #[inline]
    fn take(self, duration: Duration) -> Take<Self>
    where
        Self: Sized,
    {
        Take::new(self, duration)
    }

    /// speed up or slow down you animation
    ///
    /// scale | effect
    /// ------|-------
    /// =0.0 | your animation's duration becomes zero
    /// <1.0 | speed up your animation
    /// >1.0 | slow down your animation
    /// <0.0 | panics
    ///
    /// see [`Animation::speed_up`]
    #[inline]
    fn scale(self, scale: f32) -> Scale<Self>
    where
        Self: Sized,
    {
        Scale::new(self, scale)
    }

    /// speed up or slow down you animation
    ///
    /// ratio | effect
    /// -----|--------
    /// >1.0 | speed up your animation
    /// <1.0 | slow down your animation
    /// <=0.0 | panics
    ///
    /// see [`Animation::scale`]
    #[inline]
    fn speed_up(self, ratio: f32) -> Scale<Self>
    where
        Self: Sized,
    {
        assert!(ratio > 0.0);
        let scale = 1.0 / ratio;
        Scale::new(self, scale)
    }

    /// repeat animations with specified strategies
    ///
    /// panics if count<0
    #[inline]
    fn repeat(self, repeat: RepeatBehavior) -> Repeat<Self>
    where
        Self: Sized,
    {
        Repeat::new(self, repeat)
    }

    /// repeat your animation for specified times
    ///
    /// see [`Animation::repeat`]
    ///
    /// ## panic
    /// panics if count<0
    #[inline]
    fn times(self, count: f32) -> Repeat<Self>
    where
        Self: Sized,
    {
        Repeat::new(self, RepeatBehavior::Count(count))
    }

    // repeat your animation indefinitely
    ///
    /// see [`Animation::repeat`]
    #[inline]
    fn forever(self) -> Repeat<Self>
    where
        Self: Sized,
    {
        self.cycle()
    }

    // repeat your animation indefinitely
    ///
    /// see [`Animation::repeat`]
    #[inline]
    fn cycle(self) -> Repeat<Self>
    where
        Self: Sized,
    {
        Repeat::new(self, RepeatBehavior::Forever)
    }

    /// parallel animations, play at the same time until the longest one finishes
    #[inline]
    fn parallel<Other>(self, other: Other) -> Parallel<Self, Other>
    where
        Self: Sized,
        Other: Animation,
    {
        Parallel::new(self, other)
    }

    /// parallel animations, play at the same time until the longest one finishes.
    ///
    /// alias for [`Animation::parallel()`]
    #[inline]
    fn zip<Other>(self, other: Other) -> Parallel<Self, Other>
    where
        Self: Sized,
        Other: Animation,
    {
        Parallel::new(self, other)
    }

    /// caches animated value, reducing computing while not animating.
    /// you might want to use it at the end of the animation chains
    #[inline]
    fn cached(self) -> Cache<Self>
    where
        Self: Sized,
        Self::Item: Clone,
    {
        Cache::new(self)
    }

    /// into boxed animation
    #[inline]
    fn boxed(self) -> Boxed<Self::Item>
    where
        Self: Sized + 'static,
    {
        Boxed::new(self)
    }

    /// build [`Timeline`]
    #[inline]
    fn to_timeline(self) -> Timeline<Self::Item>
    where
        Self: Sized + 'static,
        Self::Item: 'static,
    {
        Timeline::new(self)
    }

    /// build [`Timeline`] and start to play the animation
    #[inline]
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
    #[inline]
    fn is_finished(&self, elapsed: Duration) -> bool {
        self.duration().map(|d| elapsed >= d).unwrap_or_default()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::core::{easing, Options, DURATION_ZERO};

    #[test]
    fn test_constant() {
        let animation = constant(1.0, Duration::from_millis(200));
        let v = animation.animate(DURATION_ZERO);
        assert_eq!(v, 1.0);
        let v = animation.animate(Duration::from_secs(10));
        assert_eq!(v, 1.0);
    }

    #[test]
    fn test_primitive() {
        let animation = Options::new(0.0, 1.0)
            .easing(easing::linear())
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
    fn test_primitive_const() {
        let animation = Options::new(1.0, 1.0)
            .easing(easing::linear())
            .duration(Duration::from_millis(1000))
            .auto_reverse(false)
            .build();

        let v = animation.animate(DURATION_ZERO);
        assert_eq!(v, 1.0);

        let v = animation.animate(Duration::from_millis(500));
        assert_eq!(v, 1.0);

        let v = animation.animate(Duration::from_millis(1000));
        assert_eq!(v, 1.0);

        let v = animation.animate(Duration::from_millis(1100));
        assert_eq!(v, 1.0);
    }

    #[test]
    fn test_primitive_duration_zero() {
        let animation = Options::new(1.0, 2.0)
            .easing(easing::linear())
            .duration(DURATION_ZERO)
            .auto_reverse(false)
            .build();

        let v = animation.animate(DURATION_ZERO);
        assert_eq!(v, 1.0);

        let v = animation.animate(Duration::from_millis(500));
        assert_eq!(v, 1.0);

        let v = animation.animate(Duration::from_millis(1000));
        assert_eq!(v, 1.0);

        let v = animation.animate(Duration::from_millis(1100));
        assert_eq!(v, 1.0);
    }

    #[test]
    fn test_primitive_reverse() {
        let animation = Options::new(0.0, 1.0)
            .easing(easing::linear())
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
    fn test_primitive_repeat() {
        let animation = Options::new(0.0, 1.0)
            .easing(easing::linear())
            .duration(Duration::from_millis(1000))
            .times(2.0)
            .auto_reverse(false)
            .build();

        let v = animation.animate(DURATION_ZERO);
        assert_eq!(v, 0.0);

        let v = animation.animate(Duration::from_millis(500));
        assert_eq!(v, 0.5);

        let v = animation.animate(Duration::from_millis(1000));
        assert_eq!(v, 1.0);

        let v = animation.animate(Duration::from_millis(1500));
        assert_eq!(v, 0.5);

        let v = animation.animate(Duration::from_millis(2000));
        assert_eq!(v, 1.0);

        let v = animation.animate(Duration::from_millis(2100));
        assert_eq!(v, 1.0);
    }

    #[test]
    fn test_primitive_skip() {
        let animation = Options::new(0.0, 1.0)
            .easing(easing::linear())
            .duration(Duration::from_millis(1000))
            .auto_reverse(false)
            .skip(Duration::from_millis(500))
            .build();

        let v = animation.animate(DURATION_ZERO);
        assert_eq!(v, 0.5);

        let v = animation.animate(Duration::from_millis(250));
        assert_eq!(v, 0.75);

        let v = animation.animate(Duration::from_millis(500));
        assert_eq!(v, 1.0);

        let v = animation.animate(Duration::from_millis(1100));
        assert_eq!(v, 1.0);
    }

    #[test]
    fn test_primitive_delay() {
        let animation = Options::new(0.0, 1.0)
            .easing(easing::linear())
            .duration(Duration::from_millis(1000))
            .auto_reverse(false)
            .delay(Duration::from_millis(500))
            .build();

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

        let v = animation.animate(Duration::from_millis(1700));
        assert_eq!(v, 1.0);
    }

    #[test]
    fn test_map() {
        let animation = Options::new(0.0, 1.0)
            .easing(easing::linear())
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
            .easing(easing::linear())
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
    fn test_seek_from_end() {
        let animation = Options::new(0.0, 1.0)
            .easing(easing::linear())
            .duration(Duration::from_millis(1000))
            .auto_reverse(false)
            .build()
            .seek(SeekFrom::End(Duration::from_millis(500)));

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
    fn test_seek_by() {
        let animation = Options::new(0.0, 1.0)
            .easing(easing::linear())
            .duration(Duration::from_millis(1000))
            .auto_reverse(false)
            .build()
            .seek(SeekFrom::Percent(0.5));

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
    fn test_seek_by_negative() {
        let animation = Options::new(0.0, 1.0)
            .easing(easing::linear())
            .duration(Duration::from_millis(1000))
            .auto_reverse(false)
            .build()
            .seek(SeekFrom::Percent(-0.5));

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
            .easing(easing::linear())
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
            .easing(easing::linear())
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
            .easing(easing::linear())
            .duration(Duration::from_millis(1000))
            .auto_reverse(false)
            .build()
            .parallel(
                Options::new(0.0, 1.0)
                    .easing(easing::linear())
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

    #[test]
    fn test_repeat() {
        let animation = Options::new(0.0, 1.0)
            .easing(easing::linear())
            .duration(Duration::from_millis(1000))
            .auto_reverse(false)
            .build()
            .times(1.5);

        let v = animation.animate(DURATION_ZERO);
        assert_eq!(v, 0.0);

        let v = animation.animate(Duration::from_millis(500));
        assert_eq!(v, 0.5);

        let v = animation.animate(Duration::from_millis(1000));
        assert_eq!(v, 1.0);

        let v = animation.animate(Duration::from_millis(1500));
        assert_eq!(v, 0.5);

        let v = animation.animate(Duration::from_millis(2000));
        assert_eq!(v, 0.5);

        let v = animation.animate(Duration::from_millis(2100));
        assert_eq!(v, 0.5);
    }

    #[test]
    fn test_scale_up() {
        let animation = Options::new(0.0, 1.0)
            .easing(easing::linear())
            .duration(Duration::from_millis(1000))
            .auto_reverse(false)
            .build()
            .scale(2.0);

        let v = animation.animate(DURATION_ZERO);
        assert_eq!(v, 0.0);

        let v = animation.animate(Duration::from_millis(500));
        assert_eq!(v, 0.25);

        let v = animation.animate(Duration::from_millis(1000));
        assert_eq!(v, 0.5);

        let v = animation.animate(Duration::from_millis(2000));
        assert_eq!(v, 1.0);

        let v = animation.animate(Duration::from_millis(2100));
        assert_eq!(v, 1.0);
    }

    #[test]
    fn test_scale_down() {
        let animation = Options::new(0.0, 1.0)
            .easing(easing::linear())
            .duration(Duration::from_millis(2000))
            .auto_reverse(false)
            .build()
            .scale(0.5);

        let v = animation.animate(DURATION_ZERO);
        assert_eq!(v, 0.0);

        let v = animation.animate(Duration::from_millis(500));
        assert_eq!(v, 0.5);

        let v = animation.animate(Duration::from_millis(1000));
        assert_eq!(v, 1.0);

        let v = animation.animate(Duration::from_millis(1200));
        assert_eq!(v, 1.0);

        let v = animation.animate(Duration::from_millis(2100));
        assert_eq!(v, 1.0);
    }

    #[test]
    fn test_speed_up() {
        let animation = Options::new(0.0, 1.0)
            .easing(easing::linear())
            .duration(Duration::from_millis(2000))
            .auto_reverse(false)
            .build()
            .speed_up(2.0);

        let v = animation.animate(DURATION_ZERO);
        assert_eq!(v, 0.0);

        let v = animation.animate(Duration::from_millis(500));
        assert_eq!(v, 0.5);

        let v = animation.animate(Duration::from_millis(1000));
        assert_eq!(v, 1.0);

        let v = animation.animate(Duration::from_millis(1200));
        assert_eq!(v, 1.0);

        let v = animation.animate(Duration::from_millis(2100));
        assert_eq!(v, 1.0);
    }

    #[test]
    fn test_key_frames() {
        let key_frames = key_frames(vec![
            KeyFrame::new(0.5).by_percent(0.5),
            KeyFrame::new(1.0).by_duration(Duration::from_millis(2000)),
        ]);

        let v = key_frames.animate(Duration::from_millis(0));
        assert_eq!(v, 0.5);

        let v = key_frames.animate(Duration::from_millis(500));
        assert_eq!(v, 0.5);

        let v = key_frames.animate(Duration::from_millis(1000));
        assert_eq!(v, 0.5);

        let v = key_frames.animate(Duration::from_millis(1500));
        assert_eq!(v, 0.75);

        let v = key_frames.animate(Duration::from_millis(2000));
        assert_eq!(v, 1.0);

        let v = key_frames.animate(Duration::from_millis(2100));
        assert_eq!(v, 1.0);
    }

    #[test]
    fn test_steps_infinite() {
        let steps = steps_infinite(
            |i| {
                if i == 0 {
                    return Action::Stand;
                }
                match (i - 1) % 3 {
                    0 => Action::Step1,
                    1 => Action::Step2,
                    _ => Action::Run,
                }
            },
            Duration::from_millis(100),
        );
        let v = steps.animate(DURATION_ZERO);
        assert_eq!(v, Action::Stand);

        let v = steps.animate(Duration::from_millis(100));
        assert_eq!(v, Action::Step1);

        let v = steps.animate(Duration::from_millis(199));
        assert_eq!(v, Action::Step1);

        let v = steps.animate(Duration::from_millis(900));
        assert_eq!(v, Action::Run);

        let v = steps.animate(Duration::from_millis(999));
        assert_eq!(v, Action::Run);
    }

    #[test]
    fn test_take_in_range() {
        let animation = Options::new(0.0, 1.0)
            .easing(easing::linear())
            .duration(Duration::from_millis(2000))
            .auto_reverse(false)
            .build()
            .take(Duration::from_millis(1000));

        let v = animation.animate(DURATION_ZERO);
        assert_eq!(v, 0.0);

        let v = animation.animate(Duration::from_millis(500));
        assert_eq!(v, 0.25);

        let v = animation.animate(Duration::from_millis(1000));
        assert_eq!(v, 0.5);

        let v = animation.animate(Duration::from_millis(1500));
        assert_eq!(v, 0.5);
    }

    #[test]
    fn test_take_out_range() {
        let animation = Options::new(0.0, 1.0)
            .easing(easing::linear())
            .duration(Duration::from_millis(2000))
            .auto_reverse(false)
            .build()
            .skip(Duration::from_millis(1000))
            .take(Duration::from_millis(2000));

        let v = animation.animate(DURATION_ZERO);
        assert_eq!(v, 0.5);

        let v = animation.animate(Duration::from_millis(500));
        assert_eq!(v, 0.75);

        let v = animation.animate(Duration::from_millis(1000));
        assert_eq!(v, 1.0);

        let v = animation.animate(Duration::from_millis(1500));
        assert_eq!(v, 1.0);

        let v = animation.animate(Duration::from_millis(2111));
        assert_eq!(v, 1.0);
    }

    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    enum Action {
        Stand,
        Step1,
        Step2,
        Run,
    }
}
