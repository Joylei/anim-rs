use super::{Transition, DEFAULT_TRANSITION_DURATION};
use crate::{easing, timeline::Status, Animation, Options, Timeline};
use iced_native::Vector;
use std::time::Duration;

/// fly transition parameters
///
/// see [`Fly`]
#[derive(Debug)]
pub struct Parameters {
    opt: Options<Vector>,
    offset: Vector,
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

    /// transition from/to offset x
    pub fn offset_x(mut self, x: f32) -> Self {
        self.offset = Vector { x, ..self.offset };
        self
    }

    /// transition from/to offset y
    pub fn offset_y(mut self, y: f32) -> Self {
        self.offset = Vector { y, ..self.offset };
        self
    }

    /// transition from/to offset
    pub fn offset(mut self, x: f32, y: f32) -> Self {
        self.offset = Vector { x, y };
        self
    }

    /// animation easing function
    pub fn easing(mut self, func: impl easing::Function + Clone + 'static) -> Self {
        self.opt = self.opt.easing(func);
        self
    }

    /// fly in transition
    pub fn fly_in(self) -> Fly {
        let Parameters { opt, offset } = self;
        let delay = opt.delay.unwrap_or_default();
        let animation = opt
            .from(offset)
            .build()
            .zip(Options::new(false, true).duration(delay).build());
        Fly {
            timeline: animation.to_timeline(),
        }
    }

    /// fly out transition
    pub fn fly_out(self) -> Fly {
        let Parameters { opt, offset } = self;
        let delay = opt.delay.unwrap_or_default();
        let duration = opt.duration;
        let animation = opt
            .to(offset)
            .build()
            .zip(Options::new(true, false).duration(delay + duration).build());
        Fly {
            timeline: animation.to_timeline(),
        }
    }
}

impl Default for Parameters {
    fn default() -> Self {
        let opt = Options::default().duration(DEFAULT_TRANSITION_DURATION);
        Self {
            opt,
            offset: Default::default(),
        }
    }
}

/// fly transition controller
///
/// ## Example
/// - fly in
/// ```rust
/// use std::time::Duration;
/// use anim::{Timeline, easing, transition::fly};
///
/// let transition = fly::Parameters::default()
///     .offset(0.0, 300.0)
///     .delay(Duration::from_millis(200))
///     .duration(Duration::from_secs(2))
///     .easing(easing::quad_ease())
///     .fly_in();
/// ```
/// - fly out
/// ```rust
/// use std::time::Duration;
/// use anim::{Timeline, easing, transition::fly};
///
/// let transition = fly::Parameters::default()
///     .offset(0.0, 300.0)
///     .delay(Duration::from_millis(200))
///     .duration(Duration::from_secs(2))
///     .easing(easing::quad_ease())
///     .fly_out();
/// ```
#[derive(Debug)]
pub struct Fly {
    pub(crate) timeline: Timeline<(Vector, bool)>,
}

impl Fly {
    pub(crate) fn get_value(&self) -> (Vector, bool) {
        let status = self.timeline.status();
        let (offset, visible) = self.timeline.value();
        if status.is_animating() {
            (offset, visible)
        } else {
            (Default::default(), visible)
        }
    }

    /// current offset
    pub fn offset(&self) -> Vector {
        let status = self.timeline.status();
        let (offset, _) = self.timeline.value();
        if status.is_animating() {
            offset
        } else {
            Default::default()
        }
    }
}

impl Transition for Fly {
    fn begin(&mut self) {
        self.timeline.begin();
    }

    fn stop(&mut self) {
        self.timeline.stop();
    }

    fn update(&mut self) {
        self.timeline.update();
    }

    fn status(&self) -> Status {
        self.timeline.status()
    }

    fn visible(&self) -> bool {
        let (_, v) = self.timeline.value();
        v
    }
}
