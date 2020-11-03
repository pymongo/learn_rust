#![feature(const_generics)]

use std::marker::PhantomData;

trait LengthType<const F: f64> {}

struct Length<const F: f64>(f64, PhantomData<dyn LengthType<F>>);

// type Meter = Length<1.0>;
type Mm = Length<1000.0>;
type Inch = Length<{1000.0/25.4}>;

impl<const F1: f64> Length<F1> {
    fn new(val: f64) -> Self {
        Length(val, PhantomData)
    }

    fn _from<const F2: f64>(src: Length<F2>) -> Self {
        Self::new(src.0 * F1 / F2)
    }

    fn _into<const F2: f64>(self) -> Length<F2> {
        Length::new(self.0 * F2 / F1)
    }
}

impl<const F: f64> std::ops::Add for Length<F> {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        Self::new(self.0 + other.0)
    }
}

fn main() {
    // should print 11.5(Inch)=294(mm)
    dbg!((Inch::new(10f64) + Mm::new(40f64)._into()).0);
    // should print 294.0(mm)
    dbg!((Mm::new(40.0) + Inch::new(10.0)._into()).0);
}