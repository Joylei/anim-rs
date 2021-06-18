// anim
//
// An animation library, works nicely with Iced and the others
// Copyright: 2021, Joylei <leingliu@gmail.com>
// License: MIT

//! animation library with iced or not
#![warn(missing_docs)]

mod core;
/// iced animation backend
#[cfg(feature = "iced-backend")]
mod iced;
/// thread local based timeline
#[cfg(feature = "local")]
pub mod local;

// reexports
pub use crate::core::*;
#[cfg(feature = "iced-backend")]
pub use crate::iced::*;
