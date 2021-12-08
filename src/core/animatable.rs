// anim
//
// A framework independent animation library for rust, works nicely with Iced and the others
// Copyright: 2021, Joylei <leingliu@gmail.com>
// License: MIT

#![allow(non_snake_case)]

///  generates output values based on its timing progress
///
/// see [`crate::Timeline`]
pub trait Animatable: Sized + Clone {
    /// generates output values based on its timing progress
    fn animate(&self, to: &Self, time: f64) -> Self;
}

//-------- primitives -----------
macro_rules! impl_primitive {
    ($ty:ident) => {
        impl Animatable for $ty {
            #[inline]
            fn animate(&self, to: &Self, time: f64) -> Self {
                if time == 0.0 {
                    return *self;
                }
                if (1.0 - time).abs() < f64::EPSILON {
                    return *to;
                }
                if self == to {
                    return *self;
                }
                crate::utils::check_time(time);
                let v = (*self as f64) * (1.0 - time) + (*to as f64) * time;
                if *to >= *self {
                    (v + 0.5) as Self
                } else {
                    (v - 0.5) as Self
                }
            }
        }
    };
    ($ty:ident, float) => {
        impl Animatable for $ty {
            #[inline]
            fn animate(&self, to: &Self, time: f64) -> Self {
                if time == 0.0 {
                    return *self;
                }
                if (1.0 - time).abs() < f64::EPSILON {
                    return *to;
                }
                if (self - to).abs() < $ty::EPSILON {
                    return *self;
                }
                crate::utils::check_time(time);
                // from + (to-from) * time
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
impl_primitive!(usize);
impl_primitive!(i8);
impl_primitive!(i16);
impl_primitive!(i32);
impl_primitive!(i64);
impl_primitive!(i128);
impl_primitive!(isize);
impl_primitive!(f32, float);
impl_primitive!(f64, float);

impl Animatable for bool {
    #[inline]
    fn animate(&self, to: &Self, time: f64) -> Self {
        if time < 1.0 {
            *self
        } else {
            *to
        }
    }
}

impl Animatable for char {
    #[inline]
    fn animate(&self, to: &Self, time: f64) -> Self {
        if self == to {
            return *self;
        }

        let from_idx = *self as u32;
        let to_idx = *to as u32;
        let idx = from_idx.animate(&to_idx, time);
        let n = if from_idx > to_idx {
            from_idx - idx
        } else {
            idx - from_idx
        };
        let mut rng = *self..=*to;
        match rng.nth(n as usize) {
            Some(c) => c,
            None => *self,
        }
    }
}

//-------- tuples -----------

macro_rules! impl_tuple {
    ($($n:tt $name:ident)+) => {
        impl<'de, $($name,)+> Animatable for ($($name,)+)
        where
            $($name: Animatable,)+
        {
            #[inline]
            fn animate(&self, to: &Self, time: f64) -> Self
            {
                $(
                    let $name = Animatable::animate(&self.$n, &to.$n, time);
                )+
                ($($name,)+)
            }
        }
    }
}

impl_tuple!(0 T0);
impl_tuple!(0 T0 1 T1);
impl_tuple!(0 T0 1 T1 2 T2);
impl_tuple!(0 T0 1 T1 2 T2 3 T3);
impl_tuple!(0 T0 1 T1 2 T2 3 T3 4 T4);
impl_tuple!(0 T0 1 T1 2 T2 3 T3 4 T4 5 T5);
impl_tuple!(0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6);
impl_tuple!(0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7);
impl_tuple!(0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8);
impl_tuple!(0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9);
impl_tuple!(0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10);
impl_tuple!(0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11);
impl_tuple!(0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12);
impl_tuple!(0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12 13 T13);
impl_tuple!(0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12 13 T13 14 T14);
impl_tuple!(0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12 13 T13 14 T14 15 T15);

#[cfg(test)]
mod test {
    use crate::Animatable;

    #[test]
    fn test_bool() {
        let v = false.animate(&true, 0.0);
        assert!(v == false);

        let v = false.animate(&true, 0.5);
        assert!(v == false);

        let v = false.animate(&true, 1.0);
        assert!(v == true);

        let v = true.animate(&true, 0.3);
        assert!(v == true);

        let v = false.animate(&false, 0.2);
        assert!(v == false);
    }

    #[test]
    fn test_char() {
        let v = 'a'.animate(&'e', 0.0);
        assert_eq!(v, 'a');

        let v = 'a'.animate(&'e', 0.5);
        assert_eq!(v, 'c');

        let v = 'a'.animate(&'e', 0.555);
        assert_eq!(v, 'c');

        let v = 'a'.animate(&'e', 1.0);
        assert_eq!(v, 'e');
    }
}
