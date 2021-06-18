// anim
//
// An animation library, works nicely with Iced and the others
// Copyright: 2021, Joylei <leingliu@gmail.com>
// License: MIT

use impl_trait_for_tuples::impl_for_tuples;

///  generates output values based on its timing progress
///
/// see [`crate::core::timeline::Timeline`]
pub trait Animatable: Sized + Clone {
    /// generates output values based on its timing progress
    fn animate(&self, to: &Self, time: f64) -> Self;
}

//-------- primitives -----------

#[doc(hidden)]
macro_rules! impl_primitive {
    ($t:ident) => {
        impl Animatable for $t {
            #[inline]
            fn animate(&self, to: &Self, time: f64) -> Self {
                if self == to {
                    return *self;
                }
                crate::utils::check_time(time);
                // from + (from-to) * time
                let v = (*self as f64) * (1.0 - time) + (*to as f64) * time;
                v.round() as Self
            }
        }
    };
    ($t:ident, float) => {
        impl Animatable for $t {
            #[inline]
            fn animate(&self, to: &Self, time: f64) -> Self {
                if self == to {
                    return *self;
                }
                crate::utils::check_time(time);
                // from + (from-to) * time
                let v = (*self as f64) * (1.0 - time) + (*to as f64) * time;
                v as Self
            }
        }
    };
}

impl_primitive!(u8);
impl_primitive!(u16);
impl_primitive!(u32);
impl_primitive!(u64);
impl_primitive!(u128);
impl_primitive!(i8);
impl_primitive!(i16);
impl_primitive!(i32);
impl_primitive!(i64);
impl_primitive!(i128);
impl_primitive!(f32, float);
impl_primitive!(f64, float);

// bool does not make too much sense here

impl Animatable for char {
    fn animate(&self, to: &Self, time: f64) -> Self {
        let from_idx = *self as u32;
        let to_idx = *to as u32;
        let n = from_idx.animate(&to_idx, time) as usize;
        let mut rng = *self..=*to;
        match rng.nth(n) {
            Some(c) => c,
            None => *self,
        }
    }
}

//-------- tuples -----------

#[impl_for_tuples(1, 10)]
impl Animatable for Tuple {
    for_tuples!( where #( Tuple: Animatable )* );

    fn animate(&self, to: &Self, time: f64) -> Self {
        for_tuples!( (#( Tuple::animate(&self.Tuple, &to.Tuple, time) ),* ))
    }
}
