//! Types and constants for handling temperature.

use super::measurement::*;
#[cfg(feature = "from_str")]
use regex::Regex;
#[cfg(feature = "from_str")]
use std::str::FromStr;

/// The `Temperature` struct can be used to deal with absolute temperatures in
/// a common way.
///
/// # Example
///
/// ```
/// use measurements::Temperature;
///
/// let boiling_water = Temperature::from_celsius(100.0);
/// let fahrenheit = boiling_water.as_fahrenheit();
/// println!("Boiling water measures at {} degrees fahrenheit.", fahrenheit);
/// ```
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Copy, Clone, Debug, Default)]
pub struct Temperature<T>
where
    T: num_traits::Float,
{
    degrees_kelvin: T,
}

/// The `TemperatureDelta` struct can be used to deal with differences between
/// temperatures in a common way.
///
/// # Example
///
/// ```
/// use measurements::{Temperature, TemperatureDelta};
///
/// let boiling_water = Temperature::from_celsius(100.0);
/// let frozen_water = Temperature::from_celsius(0.0);
/// let difference: TemperatureDelta = boiling_water - frozen_water;
/// println!("Boiling water is {} above freezing.", difference);
/// ```
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Copy, Clone, Debug)]
pub struct TemperatureDelta<T>
where
    T: num_traits::Float,
{
    kelvin_degrees: T,
}

impl<T> TemperatureDelta<T>
where
    T: num_traits::Float,
{
    /// Create a new TemperatureDelta from a floating point value in Kelvin
    pub fn from_kelvin(kelvin_degrees: T) -> Self {
        TemperatureDelta { kelvin_degrees }
    }

    /// Create a new TemperatureDelta from a floating point value in Celsius
    pub fn from_celsius(celsius_degrees: T) -> Self {
        TemperatureDelta::from_kelvin(celsius_degrees)
    }

    /// Create a new TemperatureDelta from a floating point value in Fahrenheit
    pub fn from_fahrenheit(farenheit_degrees: T) -> Self {
        TemperatureDelta {
            kelvin_degrees: farenheit_degrees / 1.8,
        }
    }

    /// Create a new TemperatureDelta from a floating point value in Rankine
    pub fn from_rankine(rankine_degrees: T) -> Self {
        TemperatureDelta {
            kelvin_degrees: rankine_degrees / 1.8,
        }
    }

    /// Convert this TemperatureDelta to a floating point value in Kelvin
    pub fn as_kelvin(&self) -> T {
        self.kelvin_degrees
    }

    /// Convert this TemperatureDelta to a floating point value in Celsius
    pub fn as_celsius(&self) -> T {
        self.kelvin_degrees
    }

    /// Convert this TemperatureDelta to a floating point value in Fahrenheit
    pub fn as_fahrenheit(&self) -> T {
        self.kelvin_degrees * 1.8
    }

    /// Convert this TemperatureDelta to a floating point value in Rankine
    pub fn as_rankine(&self) -> T {
        self.kelvin_degrees * 1.8
    }
}

impl<T> Temperature<T>
where
    T: num_traits::Float,
{
    /// Create a new Temperature from a floating point value in Kelvin
    pub fn from_kelvin(degrees_kelvin: T) -> Self {
        Temperature { degrees_kelvin }
    }

    /// Create a new Temperature from a floating point value in Celsius
    pub fn from_celsius(degrees_celsius: T) -> Self {
        Self::from_kelvin(degrees_celsius + 273.15)
    }

    /// Create a new Temperature from a floating point value in Fahrenheit
    pub fn from_fahrenheit(degrees_fahrenheit: T) -> Self {
        Self::from_kelvin((degrees_fahrenheit - 32.0) / 1.8 + 273.15)
    }

    /// Create a new Temperature from a floating point value in Rankine
    pub fn from_rankine(degrees_rankine: T) -> Self {
        Self::from_kelvin((degrees_rankine - 491.67) / 1.8 + 273.15)
    }

    /// Convert this absolute Temperature to a floating point value in Kelvin
    pub fn as_kelvin(&self) -> T {
        self.degrees_kelvin
    }

    /// Convert this absolute Temperature to a floating point value in Celsius
    pub fn as_celsius(&self) -> T {
        self.degrees_kelvin - 273.15
    }

    /// Convert this absolute Temperature to a floating point value in Fahrenheit
    pub fn as_fahrenheit(&self) -> T {
        (self.degrees_kelvin - 273.15) * 1.8 + 32.0
    }

    /// Convert this absolute Temperature to a floating point value in Rankine
    pub fn as_rankine(&self) -> T {
        (self.degrees_kelvin - 273.15) * 1.8 + 491.67
    }
}

impl<T> Measurement<T> for Temperature<T>
where
    T: num_traits::Float,
{
    fn as_base_units(&self) -> T {
        self.degrees_kelvin
    }

    fn from_base_units(degrees_kelvin: T) -> Self {
        Self::from_kelvin(degrees_kelvin)
    }

    fn get_base_units_name(&self) -> &'static str {
        "K"
    }
}

impl<T> Measurement<T> for TemperatureDelta<T>
where
    T: num_traits::Float,
{
    fn as_base_units(&self) -> T {
        self.kelvin_degrees
    }

    fn from_base_units(kelvin_degrees: T) -> Self {
        Self::from_kelvin(kelvin_degrees)
    }

    fn get_base_units_name(&self) -> &'static str {
        "K"
    }
}

impl<T> ::std::ops::Add<TemperatureDelta<T>> for Temperature<T>
where
    T: num_traits::Float,
{
    type Output = Temperature<T>;

    fn add(self, other: TemperatureDelta<T>) -> Temperature<T> {
        Temperature::from_kelvin(self.degrees_kelvin + other.kelvin_degrees)
    }
}

impl<T> ::std::ops::Add<Temperature<T>> for TemperatureDelta<T>
where
    T: num_traits::Float,
{
    type Output = Temperature<T>;

    fn add(self, other: Temperature<T>) -> Temperature<T> {
        other + self
    }
}

impl<T> ::std::ops::Sub<TemperatureDelta<T>> for Temperature<T>
where
    T: num_traits::Float,
{
    type Output = Temperature<T>;

    fn sub(self, other: TemperatureDelta<T>) -> Temperature<T> {
        Temperature::from_kelvin(self.degrees_kelvin - other.kelvin_degrees)
    }
}

impl<T> ::std::ops::Sub<Temperature<T>> for Temperature<T>
where
    T: num_traits::Float,
{
    type Output = TemperatureDelta<T>;

    fn sub(self, other: Temperature<T>) -> TemperatureDelta<T> {
        TemperatureDelta::from_kelvin(self.degrees_kelvin - other.degrees_kelvin)
    }
}

impl<T> ::std::cmp::Eq for Temperature<T> where T: num_traits::Float {}
impl<T> ::std::cmp::PartialEq for Temperature<T>
where
    T: num_traits::Float,
{
    fn eq(&self, other: &Self) -> bool {
        self.as_base_units() == other.as_base_units()
    }
}

impl<T> ::std::cmp::PartialOrd for Temperature<T>
where
    T: num_traits::Float,
{
    fn partial_cmp(&self, other: &Self) -> Option<::std::cmp::Ordering> {
        self.as_base_units().partial_cmp(&other.as_base_units())
    }
}

#[cfg(feature = "from_str")]
impl<T> FromStr for Temperature<T>
where
    T: num_traits::Float,
{
    type Err = std::num::ParseFloatError;

    /// Create a new Temperature from a string
    /// Plain numbers in string are considered to be Celsius
    fn from_str(val: &str) -> Result<Self, Self::Err> {
        if val.is_empty() {
            return Ok(Temperature::from_celsius(0.0));
        }

        let re = Regex::new(r"\s*([0-9.]*)\s?(deg|\u{00B0}|)?\s?([fckrFCKR]{1})\s*$").unwrap();
        if let Some(caps) = re.captures(val) {
            let float_val = caps.get(1).unwrap().as_str();
            return Ok(
                match caps.get(3).unwrap().as_str().to_uppercase().as_str() {
                    "F" => Temperature::from_fahrenheit(float_val.parse::<T>()?),
                    "C" => Temperature::from_celsius(float_val.parse::<T>()?),
                    "K" => Temperature::from_kelvin(float_val.parse::<T>()?),
                    "R" => Temperature::from_rankine(float_val.parse::<T>()?),
                    _ => Temperature::from_celsius(val.parse::<T>()?),
                },
            );
        }

        Ok(Temperature::from_celsius(val.parse::<T>()?))
    }
}

implement_display!(Temperature<T>);
implement_measurement!(TemperatureDelta<T>);

#[cfg(test)]
mod test {
    use temperature::*;
    use test_utils::assert_almost_eq;

    // Temperature Units
    #[test]
    fn kelvin() {
        let t = Temperature::from_kelvin(100.0);
        let o = t.as_kelvin();

        assert_almost_eq(o, 100.0);
    }

    #[test]
    fn celsius() {
        let t = Temperature::from_kelvin(100.0);
        let o = t.as_celsius();

        assert_almost_eq(o, -173.15);
    }

    #[test]
    fn fahrenheit() {
        let t = Temperature::from_kelvin(100.0);
        let o = t.as_fahrenheit();

        assert_almost_eq(o, -279.67);
    }

    #[test]
    fn rankine() {
        let t = Temperature::from_kelvin(100.0);
        let o = t.as_rankine();

        assert_almost_eq(o, 180.0);
    }

    #[test]
    #[cfg(feature = "from_str")]
    fn empty_str() {
        let t = Temperature::from_str("");
        assert!(t.is_ok());

        let o = t.unwrap().as_celsius();
        assert_eq!(o, 0.0);
    }

    #[test]
    #[cfg(feature = "from_str")]
    fn celsius_str() {
        let t = Temperature::from_str("100C");
        assert!(t.is_ok());

        let o = t.unwrap().as_celsius();
        assert_almost_eq(o, 100.0);
    }

    #[test]
    #[cfg(feature = "from_str")]
    fn celsius_space_str() {
        let t = Temperature::from_str("100 C");
        assert!(t.is_ok());

        let o = t.unwrap().as_celsius();
        assert_almost_eq(o, 100.0);
    }

    #[test]
    #[cfg(feature = "from_str")]
    fn celsius_degree_str() {
        let t = Temperature::from_str("100°C");
        assert!(t.is_ok());

        let o = t.unwrap().as_celsius();
        assert_almost_eq(o, 100.0);
    }

    #[test]
    #[cfg(feature = "from_str")]
    fn fahrenheit_str() {
        let t = Temperature::from_str("100F");
        assert!(t.is_ok());

        let o = t.unwrap().as_fahrenheit();
        assert_almost_eq(o, 100.0);
    }

    #[test]
    #[cfg(feature = "from_str")]
    fn fahrenheit_lc_str() {
        let t = Temperature::from_str("100 f");
        assert!(t.is_ok());

        let o = t.unwrap().as_fahrenheit();
        assert_almost_eq(o, 100.0);
    }

    #[test]
    #[cfg(feature = "from_str")]
    fn fahrenheit_degree_str() {
        let t = Temperature::from_str("100 deg f");
        assert!(t.is_ok());

        let o = t.unwrap().as_fahrenheit();
        assert_almost_eq(o, 100.0);
    }

    #[test]
    #[cfg(feature = "from_str")]
    fn rankine_str() {
        let t = Temperature::from_str("100R");
        assert!(t.is_ok());

        let o = t.unwrap().as_rankine();
        assert_almost_eq(o, 100.0);
    }

    #[test]
    #[cfg(feature = "from_str")]
    fn rankine_degree_str() {
        let t = Temperature::from_str("100 °R");
        assert!(t.is_ok());

        let o = t.unwrap().as_rankine();
        assert_almost_eq(o, 100.0);
    }

    #[test]
    #[cfg(feature = "from_str")]
    fn number_str() {
        let t = Temperature::from_str("100.5");
        assert!(t.is_ok());

        let o = t.unwrap().as_celsius();
        assert_almost_eq(o, 100.5);
    }

    #[test]
    #[cfg(feature = "from_str")]
    fn invalid_str() {
        let t = Temperature::from_str("abcd");
        assert!(t.is_err());
    }

    // Traits
    #[test]
    fn add() {
        let a = Temperature::from_kelvin(2.0);
        let b = TemperatureDelta::from_kelvin(4.0);
        let c = a + b;
        let d = b + a;
        assert_almost_eq(c.as_kelvin(), 6.0);
        assert_eq!(c, d);
    }

    #[test]
    fn add2() {
        let a = TemperatureDelta::from_kelvin(2.0);
        let b = TemperatureDelta::from_kelvin(4.0);
        let c = a + b;
        let d = b + a;
        assert_almost_eq(c.as_kelvin(), 6.0);
        assert_eq!(c, d);
    }

    #[test]
    fn sub() {
        let a = Temperature::from_kelvin(4.0);
        let b = TemperatureDelta::from_kelvin(2.0);
        let c = a - b;
        assert_almost_eq(c.as_kelvin(), 2.0);
    }

    #[test]
    fn sub2() {
        let a = Temperature::from_fahrenheit(212.0);
        let b = Temperature::from_celsius(75.0);
        let c = a - b;
        assert_almost_eq(c.as_kelvin(), 25.0);
    }

    #[test]
    fn sub3() {
        let a = TemperatureDelta::from_fahrenheit(180.0);
        let b = TemperatureDelta::from_celsius(75.0);
        let c = a - b;
        assert_almost_eq(c.as_kelvin(), 25.0);
    }

    #[test]
    fn mul() {
        let a = TemperatureDelta::from_celsius(5.0);
        let b = a * 2.0;
        let c = 2.0 * a;
        assert_almost_eq(b.as_celsius(), 10.0);
        assert_eq!(b, c);
    }

    #[test]
    fn eq() {
        let a = Temperature::from_kelvin(2.0);
        let b = Temperature::from_kelvin(2.0);
        assert_eq!(a == b, true);
    }

    #[test]
    fn neq() {
        let a = Temperature::from_kelvin(2.0);
        let b = Temperature::from_kelvin(4.0);
        assert_eq!(a == b, false);
    }

    #[test]
    fn cmp() {
        let a = Temperature::from_kelvin(2.0);
        let b = Temperature::from_kelvin(4.0);
        assert_eq!(a < b, true);
        assert_eq!(a <= b, true);
        assert_eq!(a > b, false);
        assert_eq!(a >= b, false);
    }
}
