extern crate units;
use std::marker::PhantomData;

use units::measures;

trait Unit {
    fn from_pure(value: f64) -> f64;
    fn to_pure(value: f64) -> f64;
}

struct UnitValue<T: Unit> {
    value: f64,
    unit: PhantomData<T>,
}

impl<T: Unit> UnitValue<T> {
    fn to_pure(self: &Self) -> f64 {
        T::to_pure(self.value)
    }

    fn from(value: f64) -> Self {
        Self {
            value: T::from_pure(value),
            unit: PhantomData,
        }
    }
}

#[measures(space)]
struct Meter;

impl Unit for Meter {
    fn from_pure(value: f64) -> f64 {
        value
    }

    fn to_pure(value: f64) -> f64 {
        value
    }
}

#[measures(space)]
struct Centimeter;

impl Unit for Centimeter {
    fn from_pure(value: f64) -> f64 {
        value * 100.0
    }

    fn to_pure(value: f64) -> f64 {
        value / 100.0
    }
}

fn main() {
    let length_a = UnitValue::<Meter>::from(1.0);

    println!("length_a: {}", length_a.to_pure());

    let length_b = UnitValue::<Centimeter>::from(100.0);

    println!("length_b: {}", length_b.to_pure());

    println!("Meter value type: {}", Meter::VALUE_TYPE);
}
