// anim
//
// A framework independent animation library for rust, works nicely with Iced and the others
// Copyright: 2021, Joylei <leingliu@gmail.com>
// License: MIT

use super::{Animation, BaseAnimation};
use std::{cell::RefCell, time::Duration};

/// caches animated value, reducing computing while not animating.
/// you might want to use it at the end of the animation chains.
#[derive(Debug)]
pub struct Cache<T>
where
    T: Animation,
    T::Item: Clone,
{
    src: T,
    cell: RefCell<Option<(Duration, T::Item)>>,
}

impl<T> Cache<T>
where
    T: Animation,
    T::Item: Clone,
{
    #[inline(always)]
    pub(super) fn new(src: T) -> Self {
        Self {
            src,
            cell: Default::default(),
        }
    }
}

impl<T> BaseAnimation for Cache<T>
where
    T: Animation,
    T::Item: Clone,
{
    type Item = T::Item;

    #[inline(always)]
    fn duration(&self) -> Option<Duration> {
        self.src.duration()
    }

    #[inline]
    fn animate(&self, mut elapsed: Duration) -> Self::Item {
        if let Some(duration) = self.duration() {
            if elapsed > duration {
                //finished
                elapsed = duration;
            }
        }

        if let Some((time, value)) = &*self.cell.borrow() {
            if time == &elapsed {
                return value.clone();
            }
        }
        let value = self.src.animate(elapsed);
        {
            let cell = &mut *self.cell.borrow_mut();
            *cell = Some((elapsed, value.clone()));
        }
        value
    }
}
