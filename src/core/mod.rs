// anim
//
// An animation library, works nicely with Iced and the others
// Copyright: 2021, Joylei <leingliu@gmail.com>
// License: MIT

/// make a type animatable
pub mod animatable;
pub(crate) mod animation;
/// ease functions
pub mod easing;
mod options;
/// timeline definitions
pub mod timeline;
/// utilities
pub mod utils;

use std::time::Duration;

#[doc(inline)]
pub use animatable::Animatable;
#[doc(inline)]
pub use animation::{linear, Animation};
#[doc(inline)]
pub use easing::Function;
pub use options::*;
#[doc(inline)]
pub use timeline::Timeline;

/// [`Duration`]::ZERO
pub const DURATION_ZERO: Duration = Duration::from_secs(0);
