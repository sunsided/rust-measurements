use super::measurement::*;
use super::Speed;
use std::time::Duration;

/// The `Acceleration` struct can be used to deal with Accelerations in a common way.
/// Common metric and imperial units are supported.
///
/// # Example
///
/// ```
/// use measurements::{Acceleration, Length, Speed};
/// use std::time::Duration;
///
/// // Standing quarter mile in 10.0 dead, at 120.0 mph
/// let track = Length::from_miles(0.25);
/// let finish = Speed::from_miles_per_hour(120.0);
/// let time = Duration::new(10, 0);
/// let accel = finish / time;
/// println!("You accelerated over {} at an average of {}", track, accel);
/// ```
#[derive(Copy, Clone, Debug)]
pub struct Acceleration {
    meters_per_second_per_second: f64,
}

impl Acceleration {
    pub fn from_meters_per_second_per_second(meters_per_second_per_second: f64) -> Acceleration {
        Acceleration { meters_per_second_per_second: meters_per_second_per_second }
    }

    pub fn from_metres_per_second_per_second(metres_per_second_per_second: f64) -> Acceleration {
        Acceleration::from_meters_per_second_per_second(metres_per_second_per_second)
    }

    pub fn as_meters_per_second_per_second(&self) -> f64 {
        self.meters_per_second_per_second
    }

    pub fn as_metres_per_second_per_second(&self) -> f64 {
        self.as_meters_per_second_per_second()
    }
}

/// Acceleration * Time = Speed
impl ::std::ops::Mul<Duration> for Acceleration {
    type Output = Speed;

    fn mul(self, rhs: Duration) -> Speed {
        // It would be useful if Duration had a method that did this...
        let seconds: f64 = rhs.as_secs() as f64 + ((rhs.subsec_nanos() as f64) * 1e-9);
        Speed::from_meters_per_second(self.as_meters_per_second_per_second() * seconds)
    }
}

/// Time * Acceleration = Speed
impl ::std::ops::Mul<Acceleration> for Duration {
    type Output = Speed;

    fn mul(self, rhs: Acceleration) -> Speed {
        rhs * self
    }
}

impl Measurement for Acceleration {
    fn get_base_units(&self) -> f64 {
        self.meters_per_second_per_second
    }

    fn from_base_units(units: f64) -> Self {
        Self::from_meters_per_second_per_second(units)
    }

    fn get_base_units_name(&self) -> &'static str {
        "m/s\u{00B2}"
    }
}

implement_measurement! { Acceleration }