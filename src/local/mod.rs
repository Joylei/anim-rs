// anim
//
// A framework independent animation library for rust, works nicely with Iced and the others
// Copyright: 2021, Joylei <leingliu@gmail.com>
// License: MIT

mod animator;
mod timeline;

use crate::core::{Animatable, Options};
#[doc(inline)]
pub use animator::{timeline, update};
#[doc(inline)]
pub use timeline::Timeline;

impl<T: Animatable + 'static> From<Options<T>> for Timeline<T> {
    #[inline]
    fn from(opt: Options<T>) -> Self {
        timeline(opt.build())
    }
}
