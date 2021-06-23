use iced_graphics::{Backend, Defaults, Primitive, Renderer};
use iced_native::{mouse::Interaction, Color, Element, Length, Point, Rectangle, Vector, Widget};

use super::{Transition, DEFAULT_TRANSITION_DURATION};
use crate::{easing, timeline::Status, Animation, Options, Timeline};
use std::{fmt, marker::PhantomData, time::Duration};

/// fade transition parameters
///
/// see [`Fade`]
#[derive(Debug)]
pub struct Parameters {
    opt: Options<f32>,
    opacity: f32,
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

    /// opacity for in/out
    pub fn opacity(mut self, opacity: f32) -> Self {
        assert!(opacity >= 0.0 && opacity <= 1.0);
        self.opacity = opacity;
        self
    }

    /// fade in transition
    pub fn fade_in(self) -> Fade {
        let Parameters { opt, opacity } = self;
        let delay = opt.delay.unwrap_or_default();
        let timeline = opt
            .to(opacity)
            .build()
            .zip(Options::new(false, true).duration(delay).build())
            .to_timeline();
        Fade { timeline }
    }

    /// fade out transition
    pub fn fade_out(self) -> Fade {
        let Parameters { opt, opacity } = self;
        let delay = opt.delay.unwrap_or_default();
        let duration = opt.duration;
        let timeline = opt
            .from(opacity)
            .build()
            .zip(Options::new(true, false).duration(delay + duration).build())
            .to_timeline();
        Fade { timeline }
    }
}

impl Default for Parameters {
    fn default() -> Self {
        let opt = Options::default().duration(DEFAULT_TRANSITION_DURATION);
        Self { opt, opacity: 0.0 }
    }
}

/// transition controller
#[derive(Debug)]
pub struct Fade {
    timeline: Timeline<(f32, bool)>,
}

impl Fade {
    /// current opacity
    pub fn current(&self) -> f32 {
        let status = self.timeline.status();
        if status.is_animating() {
            let (v, _) = self.timeline.value();
            v
        } else {
            Default::default()
        }
    }
}

impl Transition for Fade {
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
