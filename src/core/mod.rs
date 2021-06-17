/// make a type animatable
pub mod animatable;
/// ease functions
pub mod easing;
/// timeline definitions
pub mod timeline;
/// utilities
pub mod utils;

use std::time::Duration;

#[doc(inline)]
pub use animatable::Animatable;
#[doc(inline)]
pub use easing::Function;
#[doc(inline)]
pub use timeline::Animation;
#[doc(inline)]
pub use timeline::Timeline;

/// [`Duration`]::ZERO
pub const DURATION_ZERO: Duration = Duration::from_secs(0);
