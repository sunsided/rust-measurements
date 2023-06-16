//! # Measurements
//!
//! Measurements is a crate that lets you represent physical quantities, such
//! as Lengths, Masses, Pressures, etc. Each quantity has a series of
//! functions that allow you to convert to and from common units. You can also
//! perform arithmetic on the quantities - for example you can divide a Force
//! by an Area to get a Pressure.

#![deny(warnings, missing_docs)]
#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(
    all(feature = "std", feature = "num-traits"),
    feature("num-traits/std")
)]

#[cfg(not(feature = "std"))]
use core as std;
#[cfg(not(feature = "std"))]
use core::time;

#[cfg(feature = "std")]
use std::time;

#[cfg(feature = "serde")]
#[macro_use]
extern crate serde;

#[cfg(feature = "from_str")]
extern crate regex;

use std::f64::consts::PI;

#[macro_use]
mod measurement;
pub use measurement::Measurement;

pub mod length;
pub use length::{Distance, Length};

pub mod temperature;
pub use temperature::{Temperature, TemperatureDelta};

pub mod humidity;
pub use humidity::Humidity;

pub mod mass;
pub use mass::Mass;

pub mod volume;
pub use volume::Volume;

pub mod density;
pub use density::Density;

pub mod pressure;
pub use pressure::Pressure;

pub mod speed;
pub use speed::Speed;

pub mod acceleration;
pub use acceleration::Acceleration;

pub mod energy;
pub use energy::Energy;

pub mod power;
pub use power::Power;

pub mod voltage;
pub use voltage::Voltage;

pub mod current;
pub use current::Current;

pub mod resistance;
pub use resistance::Resistance;

pub mod force;
pub use force::Force;

pub mod area;
pub use area::Area;

pub mod angle;
pub use angle::Angle;

pub mod frequency;
pub use frequency::Frequency;

pub mod angular_velocity;
pub use angular_velocity::AngularVelocity;

pub mod torque;
pub use torque::Torque;

pub mod data;
pub use data::Data;

mod torque_energy;
pub use torque_energy::TorqueEnergy;

pub mod prelude;

pub mod test_utils;

/// For given types A, B and C, implement, using base units:
///     - A = B * C
///     - A = C * B
///     - B = A / C
///     - C = A / B
macro_rules! impl_maths {
    ($a:ty, $b:ty) => {
        impl<T> std::ops::Mul<$b> for $b
        where
            T: num_traits::Float,
        {
            type Output = $a;

            fn mul(self, rhs: $b) -> Self::Output {
                Self::Output::from_base_units(self.as_base_units() * rhs.as_base_units())
            }
        }

        impl<T> std::ops::Div<$b> for $a
        where
            T: num_traits::Float,
        {
            type Output = $b;

            fn div(self, rhs: $b) -> Self::Output {
                Self::Output::from_base_units(self.as_base_units() / rhs.as_base_units())
            }
        }
    };

    ($a:ty, $b:ty, $c:ty) => {
        impl<T> std::ops::Mul<$b> for $c
        where
            T: num_traits::Float,
        {
            type Output = $a;

            fn mul(self, rhs: $b) -> Self::Output {
                Self::Output::from_base_units(self.as_base_units() * rhs.as_base_units())
            }
        }

        impl<T> std::ops::Mul<$c> for $b
        where
            T: num_traits::Float,
        {
            type Output = $a;

            fn mul(self, rhs: $c) -> Self::Output {
                Self::Output::from_base_units(self.as_base_units() * rhs.as_base_units())
            }
        }

        impl<T> std::ops::Div<$c> for $a
        where
            T: num_traits::Float,
        {
            type Output = $b;

            fn div(self, rhs: $c) -> Self::Output {
                Self::Output::from_base_units(self.as_base_units() / rhs.as_base_units())
            }
        }

        impl<T> std::ops::Div<$b> for $a
        where
            T: num_traits::Float,
        {
            type Output = $c;

            fn div(self, rhs: $b) -> Self::Output {
                Self::Output::from_base_units(self.as_base_units() / rhs.as_base_units())
            }
        }
    };
}

impl<T> Measurement<T> for time::Duration
where
    T: num_traits::Float,
{
    fn as_base_units(&self) -> T {
        self.as_secs() as T + (T::from(self.subsec_nanos()) * 1e-9)
    }

    fn from_base_units(units: T) -> Self {
        let subsec_nanos = ((units * 1e9) % 1e9) as u32;
        let secs = units as u64;
        time::Duration::new(secs, subsec_nanos)
    }

    fn get_base_units_name(&self) -> &'static str {
        "s"
    }
}

impl_maths!(Area<T>, Length<T>);
impl_maths!(Energy<T>, time::Duration, Power<T>);
impl_maths!(Force<T>, Mass<T>, Acceleration<T>);
impl_maths!(Force<T>, Pressure<T>, Area<T>);
impl_maths!(Length<T>, time::Duration, Speed<T>);
impl_maths!(Power<T>, Force<T>, Speed<T>);
impl_maths!(Speed<T>, time::Duration, Acceleration<T>);
impl_maths!(Volume<T>, Length<T>, Area<T>);
impl_maths!(Power<T>, AngularVelocity<T>, Torque<T>);
impl_maths!(Power<T>, Voltage<T>, Current<T>);
impl_maths!(Voltage<T>, Resistance<T>, Current<T>);

// Force * Distance is ambiguous. Create an ambiguous struct the user can then
// cast into either Torque or Energy.

impl_maths!(TorqueEnergy<T>, Force<T>, Length<T>);

// Implement the divisions manually (the above macro only implemented the
// TorqueEnergy / X operations).

impl<T> std::ops::Div<Length<T>> for Torque<T> {
    type Output = Force<T>;

    fn div(self, rhs: Length<T>) -> Self::Output {
        Self::Output::from_base_units(self.as_base_units() / rhs.as_base_units())
    }
}

impl<T> std::ops::Div<Force<T>> for Torque<T> {
    type Output = Length<T>;

    fn div(self, rhs: Force<T>) -> Self::Output {
        Self::Output::from_base_units(self.as_base_units() / rhs.as_base_units())
    }
}

impl<T> std::ops::Div<Length<T>> for Energy<T> {
    type Output = Force<T>;

    fn div(self, rhs: Length<T>) -> Self::Output {
        Self::Output::from_base_units(self.as_base_units() / rhs.as_base_units())
    }
}

impl<T> std::ops::Div<Force<T>> for Energy<T> {
    type Output = Length<T>;

    fn div(self, rhs: Force<T>) -> Self::Output {
        Self::Output::from_base_units(self.as_base_units() / rhs.as_base_units())
    }
}
