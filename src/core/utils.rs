// anim
//
// An animation library, works nicely with Iced and the others
// Copyright: 2021, Joylei <leingliu@gmail.com>
// License: MIT

/// normalized time must be in 0..1
#[inline(always)]
pub fn check_time(time: f64) -> f64 {
    debug_assert!(time >= 0.0 || time <= 1.0);
    time
}
