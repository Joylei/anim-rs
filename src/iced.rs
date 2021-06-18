// anim
//
// An animation library, works nicely with Iced and the others
// Copyright: 2021, Joylei <leingliu@gmail.com>
// License: MIT

use crate::core::Animatable;
use iced_native::{Color, Point, Rectangle, Size, Vector};

impl Animatable for Point {
    #[inline]
    fn animate(&self, to: &Self, time: f64) -> Self {
        let x = self.x.animate(&to.x, time);
        let y = self.y.animate(&to.y, time);
        Point::new(x, y)
    }
}

impl<T: Animatable> Animatable for Rectangle<T> {
    #[inline]
    fn animate(&self, to: &Self, time: f64) -> Self {
        let x = self.x.animate(&to.x, time);
        let y = self.y.animate(&to.y, time);
        let width = self.width.animate(&to.width, time);
        let height = self.height.animate(&to.height, time);
        Rectangle {
            x,
            y,
            width,
            height,
        }
    }
}

impl Animatable for Color {
    #[inline]
    fn animate(&self, to: &Self, time: f64) -> Self {
        let r = self.r.animate(&to.r, time);
        let g = self.g.animate(&to.g, time);
        let b = self.b.animate(&to.b, time);
        let a = self.a.animate(&to.a, time);
        Color { r, g, b, a }
    }
}

impl<T: Animatable> Animatable for Size<T> {
    #[inline]
    fn animate(&self, to: &Self, time: f64) -> Self {
        let width = self.width.animate(&to.width, time);
        let height = self.height.animate(&to.height, time);
        Size { width, height }
    }
}

impl<T: Animatable> Animatable for Vector<T> {
    #[inline]
    fn animate(&self, to: &Self, time: f64) -> Self {
        let x = self.x.animate(&to.x, time);
        let y = self.y.animate(&to.y, time);
        Vector { x, y }
    }
}
