/// make a type animatable
pub mod animatable;
/// animation scheduler related
pub mod animator;
/// ease functions
pub mod easing;
/// timeline definitions
pub mod timeline;
/// utilities
pub mod utils;

#[doc(inline)]
pub use animatable::Animatable;
#[doc(inline)]
pub use easing::Function;
#[doc(inline)]
pub use timeline::Timeline;
