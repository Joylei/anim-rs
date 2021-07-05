use super::{Transition, DEFAULT_TRANSITION_DURATION};
use crate::{easing, timeline::Status, Animation, Options, Timeline};
use std::time::Duration;

/// Slide transition parameters
///
/// see [`Slide`]
#[derive(Debug)]
pub struct Parameters {
    opt: Options<f32>,
}

impl Parameters {
    /// delay of animation
    pub fn delay(mut self, delay: Duration) -> Self {
        self.opt = self.opt.delay(delay);
        self
    }

    /// duration of animation
    pub fn duration(mut self, duration: Duration) -> Self {
        self.opt = self.opt.duration(duration);
        self
    }

    /// animation easing function
    pub fn easing(mut self, func: impl easing::Function + Clone + 'static) -> Self {
        self.opt = self.opt.easing(func);
        self
    }

    /// slide in transition
    pub fn slide_in(self) -> Slide {
        let Parameters { opt } = self;
        let delay = opt.delay.unwrap_or_default();
        let animation = opt
            .from(0.0)
            .to(1.0)
            .build()
            .zip(Options::new(false, true).duration(delay).build());

        Slide {
            timeline: animation.to_timeline(),
        }
    }

    /// slide out transition
    pub fn slide_out(self) -> Slide {
        let Parameters { opt } = self;
        let delay = opt.delay.unwrap_or_default();
        let duration = opt.duration;
        let animation = opt
            .from(1.0)
            .to(0.0)
            .build()
            .zip(Options::new(true, false).duration(delay + duration).build());
        Slide {
            timeline: animation.to_timeline(),
        }
    }
}

impl Default for Parameters {
    fn default() -> Self {
        let opt = Options::default().duration(DEFAULT_TRANSITION_DURATION);
        Self { opt }
    }
}

/// slide transition controller
///
/// ## Example
/// - slide in
/// ```rust
/// use std::time::Duration;
/// use anim::{Timeline, easing, transition::slide};
///
/// let transition = slide::Parameters::default()
///     .delay(Duration::from_millis(200))
///     .duration(Duration::from_secs(2))
///     .easing(easing::quad_ease())
///     .slide_in();
/// ```
/// - slide out
/// ```rust
/// use std::time::Duration;
/// use anim::{Timeline, easing, transition::slide};
///
/// let transition = slide::Parameters::default()
///     .delay(Duration::from_millis(200))
///     .duration(Duration::from_secs(2))
///     .easing(easing::quad_ease())
///     .slide_out();
/// ```
#[derive(Debug)]
pub struct Slide {
    pub(crate) timeline: Timeline<(f32, bool)>,
}

impl Slide {
    /// current height ratio
    pub fn height_ratio(&self) -> f32 {
        let (ratio, _) = self.timeline.value();
        ratio
    }
}

impl Transition for Slide {
    fn update(&mut self) {
        self.timeline.update();
    }

    fn begin(&mut self) {
        self.timeline.begin();
    }

    fn stop(&mut self) {
        self.timeline.stop();
    }

    fn status(&self) -> Status {
        self.timeline.status()
    }

    fn visible(&self) -> bool {
        let (_, v) = self.timeline.value();
        v
    }
}
