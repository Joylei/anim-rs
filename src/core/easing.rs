// anim
//
// An animation library, works nicely with Iced and the others
// Copyright: 2021, Joylei <leingliu@gmail.com>
// License: MIT

use dyn_clone::DynClone;
pub use functions::*;

/// easing function
pub trait Function: DynClone {
    /// output time based on normalized time, which is between 0-1
    fn ease(&self, normalized_time: f64) -> f64;
}

impl<F: Function + Clone> Function for Box<F> {
    #[inline(always)]
    fn ease(&self, normalized_time: f64) -> f64 {
        (**self).ease(normalized_time)
    }
}

#[doc(hidden)]
#[allow(missing_docs)]
pub trait FunctionClone: Function + Clone {}

impl<F: Function + Clone> FunctionClone for F {}

/// easing mode, default [`EasingMode::InOut`]
#[derive(Debug, Clone, Copy)]
pub enum EasingMode {
    /// ease in
    In,
    /// ease out
    Out,
    /// ease in & out
    InOut,
}

impl Default for EasingMode {
    fn default() -> Self {
        EasingMode::In
    }
}

impl EasingMode {
    #[inline]
    fn apply<F: Fn(f64) -> f64>(&self, time: f64, f: &F) -> f64 {
        let time = crate::utils::check_time(time);
        match self {
            EasingMode::In => f(time),
            EasingMode::Out => 1.0 - f(time),
            EasingMode::InOut => {
                if time < 0.5 {
                    f(time)
                } else {
                    1.0 - f(time)
                }
            }
        }
    }
}

/// [`Function`] builder
#[derive(Debug, Clone)]
pub struct Easing<F: Fn(f64) -> f64> {
    mode: EasingMode,
    f: F,
}

impl<F: Fn(f64) -> f64> Easing<F> {
    /// set ease mod, see [`EasingMode`]
    #[inline]
    pub fn mode(mut self, mode: EasingMode) -> Self {
        self.mode = mode;
        self
    }
}

impl<F: Fn(f64) -> f64 + Clone> Function for Easing<F> {
    #[inline]
    fn ease(&self, normalized_time: f64) -> f64 {
        self.mode.apply(normalized_time, &self.f)
    }
}

impl<F: Fn(f64) -> f64 + Clone + 'static> From<F> for Easing<F> {
    #[inline]
    fn from(f: F) -> Self {
        functions::custom(f)
    }
}

/// please refer to:
/// - https://easings.net
/// - http://robertpenner.com/easing/
/// - https://docs.microsoft.com/en-us/dotnet/desktop/wpf/graphics-multimedia/easing-functions?redirectedfrom=MSDN&view=netframeworkdesktop-4.8
mod functions {
    use super::Easing;
    use std::f64::consts::PI;

    /// linear x=t
    #[inline]
    pub fn linear() -> Easing<impl Fn(f64) -> f64 + Clone> {
        custom(|t| t)
    }

    /// sine ease
    #[inline]
    pub fn sine_ease() -> Easing<impl Fn(f64) -> f64 + Clone> {
        custom(move |t| 1.0 - ((t * PI) / 2.0).cos())
    }

    /// pow ease
    #[inline]
    pub fn pow_ease(power: f32) -> Easing<impl Fn(f64) -> f64 + Clone> {
        let power = power as f64;
        custom(move |t| t.powf(power))
    }

    /// quadratic ease
    #[inline]
    pub fn quad_ease() -> Easing<impl Fn(f64) -> f64 + Clone> {
        pow_ease(2.0)
    }

    /// cubic ease
    #[inline]
    pub fn cubic_ease() -> Easing<impl Fn(f64) -> f64 + Clone> {
        pow_ease(3.0)
    }

    /// quart ease
    #[inline]
    pub fn quart_ease() -> Easing<impl Fn(f64) -> f64 + Clone> {
        pow_ease(4.0)
    }

    /// qunit ease
    #[inline]
    pub fn qunit_ease() -> Easing<impl Fn(f64) -> f64 + Clone> {
        pow_ease(5.0)
    }

    /// expo ease
    #[inline]
    pub fn expo_ease() -> Easing<impl Fn(f64) -> f64 + Clone> {
        custom(|t| {
            if t == 0.0 {
                0.0
            } else {
                (2.0 as f64).powf(10.0 * t - 10.0)
            }
        })
    }

    /// circle ease
    #[inline]
    pub fn circle_ease() -> Easing<impl Fn(f64) -> f64 + Clone> {
        custom(|t| 1.0 - (1.0 - t.powi(2)).sqrt())
    }

    /// back ease
    #[inline]
    pub fn back_ease(amplitude: f64) -> Easing<impl Fn(f64) -> f64 + Clone> {
        custom(move |t| t.powi(3) - t * amplitude * (t * PI).sin())
    }

    /// elastic ease
    #[inline]
    pub fn elastic_ease() -> Easing<impl Fn(f64) -> f64 + Clone> {
        const C4: f64 = (2.0 * PI) / 3.0;
        custom(|t| {
            if t == 0.0 {
                0.0
            } else if t == 1.0 {
                1.0
            } else {
                -(2.0 as f64).powf(10.0 * t - 10.0) * ((t * 10.0 - 10.75) * C4).sin()
            }
        })
    }

    /// bounce ease
    #[inline]
    pub fn bounce_ease() -> Easing<impl Fn(f64) -> f64 + Clone> {
        const N1: f64 = 7.5625;
        const D1: f64 = 2.75;
        custom(|t| {
            let v = if t < 1.0 / D1 {
                N1 * t * t
            } else if t < 2.0 / D1 {
                let t = t - 1.5 / D1;
                N1 * t * t + 0.75
            } else if t < 2.5 / D1 {
                let t = t - 2.25 / D1;
                N1 * t * t + 0.9375
            } else {
                let t = t - 2.625 / D1;
                N1 * t * t + 0.984375
            };
            1.0 - v
        })
    }

    /// custom ease function
    #[inline]
    pub fn custom<F: Fn(f64) -> f64 + Clone + 'static>(f: F) -> Easing<F> {
        Easing {
            mode: Default::default(),
            f,
        }
    }
}
