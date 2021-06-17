use super::{easing, Animatable, DURATION_ZERO};
use private::*;
use std::{
    cell::RefCell,
    fmt::{self, Debug},
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

/// control your animation
//#[derive(Debug)]
pub struct Timeline<T> {
    id: usize,
    animation: Boxed<T>,
    state: State,
}

impl<T> Timeline<T> {
    /// construct your animation
    #[inline]
    pub fn new<F>(animation: F) -> Self
    where
        F: Into<Boxed<T>>,
    {
        Self {
            id: ID_GEN.fetch_add(1, std::sync::atomic::Ordering::Relaxed),
            animation: animation.into(),
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

/// repeat behavior for your animation
#[derive(Debug, Clone)]
pub enum RepeatBehavior {
    /// repeat limited times, default 1
    Count(u32),
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
    ///
    /// panics if count==0
    #[inline]
    pub fn times(mut self, count: u32) -> Self {
        if count == 0 {
            panic!("count must >=1")
        }
        self.repeat = RepeatBehavior::Count(count);
        self
    }

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

/// build [`Primitive`]
pub trait Builder<T: Animatable> {
    /// build [`Primitive`]
    fn build(self) -> Primitive<T>;
}

impl<T: Animatable> Builder<T> for Options<T> {
    /// into [`Primitive`]
    #[inline]
    fn build(self) -> Primitive<T> {
        Primitive { opt: self }
    }
}

/// your animation, which outputs animated value based on the progressing time.
///
/// Simply, you can think it as an [`Iterator`]. The difference is that an [Animation]
/// always output some values.
pub trait Animation: BaseAnimation {
    /// always delay for specified time when start
    #[inline]
    fn delay(self, delay: Duration) -> Delay<Self>
    where
        Self: Sized,
    {
        if delay < DURATION_ZERO {
            panic!("delay must be >=0");
        }

        Delay { src: self, delay }
    }

    /// always delay for specified time when start
    #[inline]
    fn delay_ms(self, millis: u64) -> Delay<Self>
    where
        Self: Sized,
    {
        Delay {
            src: self,
            delay: Duration::from_millis(millis),
        }
    }

    /// always move forward for specified time when start
    #[inline]
    fn skip(self, progress: Duration) -> Skip<Self>
    where
        Self: Sized,
    {
        if progress < DURATION_ZERO {
            panic!("progress must be >=0");
        }
        Skip {
            src: self,
            progress,
        }
    }

    /// map from one type to another
    #[inline]
    fn map<F, T>(self, f: F) -> Map<Self, F, T>
    where
        Self: Sized,
        F: Fn(Self::Item) -> T,
    {
        Map { src: self, f }
    }

    /// chain two animations
    #[inline]
    fn chain<Other>(self, other: Other) -> Chain<Self::Item>
    where
        Self: Sized + 'static,
        Other: Into<Boxed<Self::Item>>,
    {
        Chain {
            items: vec![Boxed::new(self), other.into()],
        }
    }

    /// parallel animations, run at the same time until the longest one finishes
    #[inline]
    fn parallel<Other>(self, other: Other) -> Parallel<Self, Other, Self::Item>
    where
        Self: Sized,
        Other: Animation<Item = Self::Item>,
    {
        Parallel {
            first: self,
            second: other,
        }
    }

    /// caches animated value, reducing computing while not animating.
    /// you might want to use it at the end of the animation chains
    #[inline]
    fn cached(self) -> Cache<Self>
    where
        Self: Sized,
        Self::Item: Clone,
    {
        Cache {
            src: self,
            cell: Default::default(),
        }
    }

    /// into boxed animation
    #[inline]
    fn boxed(self) -> Boxed<Self::Item>
    where
        Self: Sized + 'static,
    {
        Boxed::new(self)
    }

    // /// into [`Timeline`]
    // #[inline]
    // fn into_timeline(self) -> Timeline<Self::Item>
    // where
    //     Self: Sized + 'static,
    //     Self::Item: 'static,
    // {
    //     Timeline::new(Boxed::new(self))
    // }
}

/// primitive animation which is built from [`Options`]
#[derive(Debug)]
pub struct Primitive<T: Animatable> {
    opt: Options<T>,
}

impl<T: Animatable> BaseAnimation for Primitive<T> {
    type Item = T;

    #[inline]
    fn duration(&self) -> Option<Duration> {
        match self.opt.repeat {
            RepeatBehavior::Count(count) => Some(self.opt.duration * count),
            RepeatBehavior::Forever => None,
        }
    }

    fn animate(&self, elapsed: Duration) -> Self::Item {
        // calc normalized time
        let finished = self.duration().map(|d| elapsed >= d).unwrap_or_default();
        let time = if finished {
            if self.opt.auto_reverse {
                0.0
            } else {
                1.0
            }
        } else {
            let time = elapsed.as_secs_f64() / self.opt.duration.as_secs_f64();
            time - time.floor()
        };

        if self.opt.auto_reverse {
            if time > 0.5 {
                //reverse
                self.opt.to.animate(&self.opt.from, time * 2.0 - 1.0)
            } else {
                self.opt.from.animate(&self.opt.to, time * 2.0)
            }
        } else {
            self.opt.from.animate(&self.opt.to, time)
        }
    }
}

impl<T: Animatable> Animation for Primitive<T> {}

impl<T: Animatable + 'static> From<Primitive<T>> for Boxed<T> {
    #[inline]
    fn from(src: Primitive<T>) -> Self {
        Boxed::new(src)
    }
}

impl<T: Animatable> From<Options<T>> for Primitive<T> {
    #[inline]
    fn from(opt: Options<T>) -> Self {
        Primitive { opt }
    }
}

/// delay your animation for a specified time
pub struct Delay<T: Animation> {
    src: T,
    delay: Duration,
}

impl<T: Animation> BaseAnimation for Delay<T> {
    type Item = T::Item;
    #[inline]
    fn duration(&self) -> Option<Duration> {
        debug_assert!(self.delay >= Duration::from_secs(0));
        self.src.duration().map(|d| self.delay + d)
    }

    #[inline]
    fn animate(&self, elapsed: Duration) -> Self::Item {
        debug_assert!(self.delay >= Duration::from_secs(0));
        let elapsed = if elapsed > self.delay {
            elapsed - self.delay
        } else {
            DURATION_ZERO
        };
        self.src.animate(elapsed)
    }
}

impl<T: Animation> Animation for Delay<T> {}

impl<T: Animation + 'static> From<Delay<T>> for Boxed<T::Item> {
    #[inline]
    fn from(src: Delay<T>) -> Self {
        Boxed::new(src)
    }
}

/// always bypass specified time
pub struct Skip<T: Animation> {
    src: T,
    progress: Duration,
}

impl<T: Animation> BaseAnimation for Skip<T> {
    type Item = T::Item;
    #[inline]
    fn duration(&self) -> Option<Duration> {
        debug_assert!(self.progress >= DURATION_ZERO);
        self.src.duration().map(|d| {
            if d > self.progress {
                d - self.progress
            } else {
                DURATION_ZERO
            }
        })
    }

    #[inline]
    fn animate(&self, elapsed: Duration) -> Self::Item {
        debug_assert!(self.progress >= DURATION_ZERO);
        let elapsed = self.progress + elapsed;
        self.src.animate(elapsed)
    }
}

impl<T: Animation> Animation for Skip<T> {}

impl<T: Animation + 'static> From<Skip<T>> for Boxed<T::Item> {
    #[inline]
    fn from(src: Skip<T>) -> Self {
        Boxed::new(src)
    }
}

/// map from one type to another
pub struct Map<Source, F, T>
where
    Source: Animation,
    F: Fn(Source::Item) -> T,
{
    src: Source,
    f: F,
}

impl<Source, F, T> BaseAnimation for Map<Source, F, T>
where
    Source: Animation,
    F: Fn(Source::Item) -> T,
{
    type Item = T;

    #[inline]
    fn duration(&self) -> Option<Duration> {
        self.src.duration()
    }

    #[inline]
    fn animate(&self, elapsed: Duration) -> Self::Item {
        let v = self.src.animate(elapsed);
        (self.f)(v)
    }
}

impl<Source, F, T> Animation for Map<Source, F, T>
where
    Source: Animation,
    F: Fn(Source::Item) -> T,
{
}

impl<Source, F, T> From<Map<Source, F, T>> for Boxed<T>
where
    Source: Animation + 'static,
    F: Fn(Source::Item) -> T + 'static,
    T: 'static,
{
    #[inline]
    fn from(src: Map<Source, F, T>) -> Self {
        Boxed::new(src)
    }
}

/// chained animations, runs in orders
pub struct Chain<T> {
    items: Vec<Boxed<T>>,
}

impl<T> Chain<T> {
    /// push another one
    #[inline]
    pub fn push<F>(mut self, item: F) -> Self
    where
        F: Into<Boxed<T>>,
    {
        self.items.push(item.into());
        self
    }
}

impl<T> BaseAnimation for Chain<T> {
    type Item = T;

    #[inline]
    fn duration(&self) -> Option<Duration> {
        let mut acc = DURATION_ZERO;
        for item in self.items.iter() {
            match item.duration() {
                Some(d) => acc += d,
                None => return None,
            }
        }
        return Some(acc);
    }

    #[inline]
    fn animate(&self, mut elapsed: Duration) -> Self::Item {
        let count = self.items.len();
        for (i, item) in self.items.iter().enumerate() {
            match item.duration() {
                Some(d) => {
                    if i + 1 < count {
                        //not last one
                        if elapsed >= d {
                            elapsed -= d;
                            continue;
                        }
                    }
                    return item.animate(elapsed);
                }
                None => return item.animate(elapsed), // never go to next
            }
        }

        panic!("empty collection")
    }
}

impl<T> Animation for Chain<T> {}

impl<T: 'static> From<Chain<T>> for Boxed<T> {
    #[inline]
    fn from(src: Chain<T>) -> Self {
        Boxed::new(src)
    }
}

/// caches animated value, reducing computing while not animating.
/// you might want to use it at the end of the animation chains.
pub struct Cache<T>
where
    T: Animation,
    T::Item: Clone,
{
    src: T,
    cell: RefCell<Option<(Duration, T::Item)>>,
}

impl<T> BaseAnimation for Cache<T>
where
    T: Animation,
    T::Item: Clone,
{
    type Item = T::Item;

    #[inline]
    fn duration(&self) -> Option<Duration> {
        self.src.duration()
    }

    #[inline]
    fn animate(&self, mut elapsed: Duration) -> Self::Item {
        if let Some(duration) = self.duration() {
            if elapsed > duration {
                //finished
                elapsed = duration;
            }
        }

        if let Some((time, value)) = &*self.cell.borrow() {
            if time == &elapsed {
                return value.clone();
            }
        }
        let value = self.src.animate(elapsed);
        {
            let cell = &mut *self.cell.borrow_mut();
            *cell = Some((elapsed, value.clone()));
        }
        value
    }
}

impl<T> Animation for Cache<T>
where
    T: Animation,
    T::Item: Clone,
{
}

impl<T> From<Cache<T>> for Boxed<T::Item>
where
    T: Animation + 'static,
    T::Item: Clone,
{
    #[inline]
    fn from(src: Cache<T>) -> Self {
        Boxed::new(src)
    }
}

/// parallel animations
pub struct Parallel<First, Second, T>
where
    First: Animation<Item = T>,
    Second: Animation<Item = T>,
{
    first: First,
    second: Second,
}

impl<First, Second, T> BaseAnimation for Parallel<First, Second, T>
where
    First: Animation<Item = T>,
    Second: Animation<Item = T>,
{
    type Item = (T, T);

    fn duration(&self) -> Option<Duration> {
        let first = if let Some(v) = self.first.duration() {
            v
        } else {
            return None;
        };

        let second = if let Some(v) = self.second.duration() {
            v
        } else {
            return None;
        };

        Some(first.max(second))
    }
    #[inline]
    fn animate(&self, elapsed: Duration) -> Self::Item {
        let first = self.first.animate(elapsed);
        let second = self.second.animate(elapsed);
        (first, second)
    }
}

impl<First, Second, T> Animation for Parallel<First, Second, T>
where
    First: Animation<Item = T>,
    Second: Animation<Item = T>,
{
}

impl<First, Second, T> From<Parallel<First, Second, T>> for Boxed<(T, T)>
where
    First: Animation<Item = T> + 'static,
    Second: Animation<Item = T> + 'static,
    T: 'static,
{
    #[inline]
    fn from(src: Parallel<First, Second, T>) -> Self {
        Boxed::new(src)
    }
}

/// boxed animation
pub type BoxAnimation<T> = Box<dyn Animation<Item = T>>;

impl<F: ?Sized + Animation> BaseAnimation for Box<F> {
    type Item = F::Item;
    #[inline]
    fn duration(&self) -> Option<Duration> {
        (**self).duration()
    }

    #[inline]
    fn animate(&self, elapsed: Duration) -> Self::Item {
        (**self).animate(elapsed)
    }
}

impl<F: ?Sized + Animation> Animation for Box<F> {}

impl<F: Sized + Animation + 'static> From<Box<F>> for Boxed<F::Item> {
    #[inline]
    fn from(src: Box<F>) -> Self {
        Boxed(src)
    }
}

/// wrapper for boxed ['Animation']
pub struct Boxed<T>(Box<dyn Animation<Item = T>>);

impl<T> Boxed<T> {
    /// construct [`Boxed`]
    #[inline]
    pub fn new<F>(src: F) -> Self
    where
        F: Animation<Item = T> + 'static,
    {
        Self(Box::new(src))
    }
}

impl<T> BaseAnimation for Boxed<T> {
    type Item = T;
    #[inline]
    fn duration(&self) -> Option<Duration> {
        self.0.duration()
    }
    #[inline]
    fn animate(&self, elapsed: Duration) -> Self::Item {
        self.0.animate(elapsed)
    }
}

impl<T> Animation for Boxed<T> {}

impl<T: Animatable + 'static> From<Options<T>> for Boxed<T> {
    #[inline]
    fn from(opt: Options<T>) -> Self {
        Boxed::new(opt.build())
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

mod private {
    use std::time::Duration;
    /// your animation, which outputs animated value based on the progressing time
    pub trait BaseAnimation {
        /// animated value
        type Item;

        /// the animation lasts for how long; `None` means it's never finished
        fn duration(&self) -> Option<Duration>;

        /// outputs animated value based on the progressing time
        fn animate(&self, elapsed: Duration) -> Self::Item;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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

        //note: how to handle this situation? it's not continuous.
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
