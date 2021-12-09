// anim
//
// A framework independent animation library for rust, works nicely with Iced and the others
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

mod clock;

use std::time::Duration;

#[doc(inline)]
pub use animatable::Animatable;
#[doc(inline)]
pub use animation::{Animation, Cursor, KeyFrame, KeyTime, SeekFrom};
#[doc(inline)]
pub use easing::Function;
#[doc(inline)]
pub use options::*;
#[doc(inline)]
pub use timeline::Timeline;

/// deprecated, please use [`builder::linear`] instead
#[deprecated]
#[doc(hidden)]
pub use animation::linear;

/// [`Duration`]::ZERO
pub const DURATION_ZERO: Duration = Duration::from_secs(0);

/// default animation time, 1 second
pub const DEFAULT_ANIMATION_DURATION: Duration = Duration::from_secs(1);

/// animation builders
pub mod builder {
    #[doc(inline)]
    pub use super::animation::{constant, key_frames, linear, steps, steps_infinite};
}
