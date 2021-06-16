//! animation library for iced
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
