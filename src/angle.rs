//! Types and constants for handling angles

use super::measurement::*;

#[cfg(feature = "from_str")]
use regex::Regex;
#[cfg(feature = "from_str")]
use std::str::FromStr;

/// The 'Angle' struct can be used to deal with angles in a common way.
///
/// # Example
///
/// ```
/// use measurements::Angle;
///
/// let whole_cake = Angle::from_degrees(360.0);
/// let pieces = 6.0;
/// let slice = whole_cake / pieces;
/// println!("Each slice will be {} degrees", slice.as_degrees());
/// ```
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Copy, Clone, Debug, Default)]
pub struct Angle<T>
where
    T: num_traits::Float,
{
    radians: T,
}

impl<T> Angle<T>
where
    T: num_traits::Float,
{
    /// Create a new Angle from a floating point value in degrees
    pub fn from_degrees(degrees: T) -> Self {
        Angle::from_radians(degrees * ::PI / 180.0)
    }

    /// Create a new Angle from a floating point value in radians
    pub fn from_radians(radians: T) -> Self {
        Angle { radians }
    }

    /// Convert this Angle to a floating point value in degrees
    pub fn as_degrees(&self) -> T {
        self.radians * 180.0 / ::PI
    }

    /// Convert this Angle to a floating point value in radians
    pub fn as_radians(&self) -> T {
        self.radians
    }

    /// Calculate the cosine of this angle
    #[cfg(feature = "std")]
    pub fn cos(&self) -> T {
        self.radians.cos()
    }

    /// Calculate the sine of this angle
    #[cfg(feature = "std")]
    pub fn sin(&self) -> T {
        self.radians.sin()
    }

    /// Calculate the sine and cosine of this angle
    #[cfg(feature = "std")]
    pub fn sin_cos(&self) -> (T, T) {
        self.radians.sin_cos()
    }

    /// Calculate the tangent of this angle
    #[cfg(feature = "std")]
    pub fn tan(&self) -> T {
        self.radians.tan()
    }

    /// Calculate the arcsine of a number
    #[cfg(feature = "std")]
    pub fn asin(num: T) -> Self {
        Angle::from_radians(num.asin())
    }

    /// Calculate the arccosine of a number
    #[cfg(feature = "std")]
    pub fn acos(num: T) -> Self {
        Angle::from_radians(num.acos())
    }

    /// Calculate the arctangent of a number
    #[cfg(feature = "std")]
    pub fn atan(num: T) -> Self {
        Angle::from_radians(num.atan())
    }
}

impl<T> Measurement<T> for Angle<T>
where
    T: num_traits::Float,
{
    fn as_base_units(&self) -> T {
        self.radians
    }

    fn from_base_units(units: T) -> Self {
        Self::from_radians(units)
    }

    fn get_base_units_name(&self) -> &'static str {
        "rad"
    }
}

#[cfg(feature = "from_str")]
impl FromStr for Angle
where
    T: num_traits::Float,
{
    type Err = std::num::ParseFloatError;

    /// Create a new Angle from a string
    /// Plain numbers in string are considered to be plain degrees
    fn from_str(val: &str) -> Result<Self, Self::Err> {
        if val.is_empty() {
            return Ok(Angle::from_degrees(0.0));
        }

        let re = Regex::new(r"(?i)\s*([0-9.]*)\s?(deg|\u{00B0}|rad)\s*$").unwrap();
        if let Some(caps) = re.captures(val) {
            let float_val = caps.get(1).unwrap().as_str();
            return Ok(
                match caps.get(2).unwrap().as_str().to_lowercase().as_str() {
                    "deg" | "\u{00B0}" => Angle::from_degrees(float_val.parse::<T>()?),
                    "rad" => Angle::from_radians(float_val.parse::<T>()?),
                    _ => Angle::from_degrees(val.parse::<T>()?),
                },
            );
        }

        Ok(Angle::from_degrees(val.parse::<T>()?))
    }
}

implement_measurement! { Angle<T> }

#[cfg(test)]
mod test {
    use angle::*;
    use std::f64::consts::PI;
    use test_utils::assert_almost_eq;

    #[test]
    fn radians() {
        let i1 = Angle::from_degrees(360.0);
        let r1 = i1.as_radians();
        let i2 = Angle::from_radians(PI);
        let r2 = i2.as_degrees();
        assert_almost_eq(r1, 2.0 * PI);
        assert_almost_eq(r2, 180.0);
    }

    #[test]
    #[cfg(feature = "from_str")]
    fn angle_from_str() {
        let t = Angle::from_str("100 deg");
        assert!(t.is_ok());

        let o = t.unwrap().as_degrees();
        assert_almost_eq(o, 100.0);
    }

    #[test]
    #[cfg(feature = "from_str")]
    fn angle_from_degree_str() {
        let t = Angle::from_str("100°");
        assert!(t.is_ok());

        let o = t.unwrap().as_degrees();
        assert_almost_eq(o, 100.0);
    }

    #[test]
    #[cfg(feature = "from_str")]
    fn angle_from_radian_str() {
        let t = Angle::from_str("100rad");
        assert!(t.is_ok());

        let o = t.unwrap().as_radians();
        assert_almost_eq(o, 100.0);
    }

    #[test]
    #[cfg(feature = "from_str")]
    fn default_str() {
        let t = Angle::from_str("100");
        assert!(t.is_ok());

        let o = t.unwrap().as_degrees();
        assert_almost_eq(o, 100.0);
    }

    #[test]
    #[cfg(feature = "from_str")]
    fn invalid_str() {
        let t = Angle::from_str("abcd");
        assert!(t.is_err());
    }
}
