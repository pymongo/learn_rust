use std::marker::PhantomData;

trait LengthType<const F: u32> {}

struct Length<const F: u32>(u32, PhantomData<dyn LengthType<F>>);

// type Meter = Length<1>;
type Mm = Length<1000>;
// type Inch = Length<{ 1000.0 / 25.4 }>;

// 1 meter = 100 cm
type Cm = Length<100>;

impl<const F1: u32> Length<F1> {
    fn new(val: u32) -> Self {
        Length(val, PhantomData)
    }

    fn into_length<const F2: u32>(self) -> Length<F2> {
        Length::new(self.0 * F2 / F1)
    }
}

impl<const F: u32> std::ops::Add for Length<F> {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        Self::new(self.0 + other.0)
    }
}

fn main() {
    // 14 cm
    dbg!((Cm::new(10u32) + Mm::new(40u32).into_length()).0);
    // 140 mm
    dbg!((Mm::new(40) + Cm::new(10).into_length()).0);
}
