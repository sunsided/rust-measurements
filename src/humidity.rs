//! Types and constants for handling humidity.

use super::measurement::*;
use density::Density;
use pressure::Pressure;
use temperature::Temperature;

/// The `Humidity` struct can be used to deal with relative humidity
/// in air in a common way. Relative humidity is an important metric used
/// in weather forecasts.
///
/// Relative humidity (as a ratio and percentage) and conversions between
/// relative humidity and dewpoint are supported. It also provides calculations
/// giving vapour pressure and absolute humidity.
///
/// Relative humidity gives the ratio of how much moisture the air is
/// holding to how much moisture it could hold at a given temperature.
/// Here we use the technical definition of humidity as ratio of the
/// actual water vapor pressure to the equilibrium vapor pressure
/// (often called the "saturation" vapor pressure).
///
/// For dewpoint calculations, we use the algorithm commonly known as
/// the Magnus formula, with coefficients derived by Alduchov and
/// Eskridge (1996), which gives resonable accuracy (vapour pressure
/// error < 0.2%) for temperatures between 0 deg C, and 50 deg C.
///
/// # Example:
///
/// ```
///     //  calculate the dewpoint from the relative humidity
///     use measurements::{Humidity,Temperature};
///
///     let humidity = Humidity::from_percent(85.0);
///     let temp = Temperature::from_celsius(18.0);
///     let dewpoint = humidity.as_dewpoint(temp);
///     println!("At {} humidity, air at {} has a dewpoint of {}", humidity, temp, dewpoint);
///
/// ```
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Copy, Clone, Debug, Default)]
pub struct Humidity<T>
where
    T: num_traits::Float,
{
    relative_humidity: T, // expressed as a percentage
}

impl<T> Humidity<T>
where
    T: num_traits::Float,
{
    /// Create a new Humidity from a floating point value percentage (i.e. 0.0% to 100.0%)
    pub fn from_percent(percent: T) -> Self {
        Humidity {
            relative_humidity: percent,
        }
    }

    /// Create a new Humidity from a floating point value ratio (i.e. 0.0 to 1.0)
    pub fn from_ratio(relative_humidity: T) -> Self {
        Humidity {
            relative_humidity: relative_humidity * 100.0,
        }
    }
    /// Convert this relative humidity to a value expressed as a ratio (i.e. 0.0 to 1.0)
    pub fn as_ratio(&self) -> T {
        self.relative_humidity / 100.0
    }

    /// Convert this relative humidty to a value expressed as a percentage (i.e. 0.0% to 100.0%)
    pub fn as_percent(&self) -> T {
        self.relative_humidity
    }

    /// Calculates Dewpoint from humidity and air temperature using the Magnus-Tetens
    /// approximation, with coefficients derived by Alduchov and Eskridge (1996). The formulas assume
    //  standard atmospheric pressure.
    #[cfg(feature = "std")]
    pub fn as_dewpoint(&self, temp: Temperature) -> Temperature {
        let humidity = self.relative_humidity / 100.0;
        let celsius = temp.as_celsius();
        let dewpoint: T = 243.04 * (humidity.ln() + ((17.625 * celsius) / (243.04 + celsius)))
            / (17.625 - humidity.ln() - ((17.625 * celsius) / (243.04 + celsius)));
        Temperature::from_celsius(dewpoint)
    }

    /// Calculates Dewpoint from humidity and air temperature using the Magnus-Tetens
    /// approximation, with coefficients derived by Alduchov and Eskridge (1996). The formulas assume
    //  standard atmospheric pressure.
    #[cfg(not(feature = "std"))]
    pub fn as_dewpoint(&self, temp: Temperature<T>) -> Temperature<T> {
        let humidity = self.relative_humidity / 100.0;
        let celsius = temp.as_celsius();
        let humidity_ln = libm::log(humidity);
        let dewpoint: T = 243.04 * (humidity_ln + ((17.625 * celsius) / (243.04 + celsius)))
            / (17.625 - humidity_ln - ((17.625 * celsius) / (243.04 + celsius)));
        Temperature::from_celsius(dewpoint)
    }

    /// Calculates the actual vapour pressure in the air, based on the air temperature and humidity
    /// at standard atmospheric pressure (1013.25 mb), using the Buck formula (accurate to +/- 0.02%
    /// between 0 deg C and 50 deg C)
    #[cfg(feature = "std")]
    pub fn as_vapor_pressure(&self, temp: Temperature) -> Pressure {
        let temp = temp.as_celsius();
        let saturation_vapor_pressure =
            0.61121 * ((18.678 - (temp / 234.5)) * (temp / (257.14 + temp))).exp();
        Pressure::from_kilopascals((self.relative_humidity * saturation_vapor_pressure) / 100.0)
    }

    /// Calculates the actual vapour pressure in the air, based on the air temperature and humidity
    /// at standard atmospheric pressure (1013.25 mb), using the Buck formula (accurate to +/- 0.02%
    /// between 0 deg C and 50 deg C)
    #[cfg(not(feature = "std"))]
    pub fn as_vapor_pressure(&self, temp: Temperature<T>) -> Pressure<T> {
        let temp = temp.as_celsius();
        let saturation_vapor_pressure =
            0.61121 * libm::exp((18.678 - (temp / 234.5)) * (temp / (257.14 + temp)));
        Pressure::from_kilopascals((self.relative_humidity * saturation_vapor_pressure) / 100.0)
    }

    /// Calculates the absolute humidity (i.e. the density of water vapor in the air (kg/m3)), using
    /// the Ideal Gas Law equation.
    pub fn as_absolute_humidity(&self, temp: Temperature<T>) -> Density<T> {
        // use the Ideal Gas Law equation (Density = Pressure / (Temperature * [gas constant
        // for water vapor= 461.5 (J/kg*Kelvin)]))
        let density = self.as_vapor_pressure(temp).as_pascals() / (temp.as_kelvin() * 461.5);
        Density::from_kilograms_per_cubic_meter(density)
    }

    /// Calculates humidity from dewpoint and air temperature using the Magnus-Tetens
    /// Approximation, with coefficients derived by Alduchov and Eskridge (1996). The formulas assume
    //  standard atmospheric pressure.
    #[cfg(feature = "std")]
    pub fn from_dewpoint(dewpoint: Temperature, temp: Temperature) -> Humidity {
        let dewpoint = dewpoint.as_celsius();
        let temp = temp.as_celsius();
        let rh = 100.0
            * (((17.625 * dewpoint) / (243.04 + dewpoint)).exp()
                / ((17.625 * temp) / (243.04 + temp)).exp());
        Humidity::from_percent(rh)
    }

    /// Calculates humidity from dewpoint and air temperature using the Magnus-Tetens
    /// Approximation, with coefficients derived by Alduchov and Eskridge (1996). The formulas assume
    //  standard atmospheric pressure.
    #[cfg(not(feature = "std"))]
    pub fn from_dewpoint(dewpoint: Temperature<T>, temp: Temperature<T>) -> Humidity<T> {
        let dewpoint = dewpoint.as_celsius();
        let temp = temp.as_celsius();
        let rh = 100.0
            * (libm::exp((17.625 * dewpoint) / (243.04 + dewpoint))
                / libm::exp((17.625 * temp) / (243.04 + temp)));
        Humidity::from_percent(rh)
    }
}

impl<T> Measurement<T> for Humidity<T>
where
    T: num_traits::Float,
{
    fn as_base_units(&self) -> T {
        self.relative_humidity
    }

    fn from_base_units(relative_humidity: T) -> Self {
        Self::from_percent(relative_humidity)
    }

    fn get_base_units_name(&self) -> &'static str {
        "%"
    }
}

impl<T> ::std::cmp::Eq for Humidity<T> where T: num_traits::Float {}
impl<T> ::std::cmp::PartialEq for Humidity<T>
where
    T: num_traits::Float,
{
    fn eq(&self, other: &Self) -> bool {
        self.as_base_units() == other.as_base_units()
    }
}

impl<T> ::std::cmp::PartialOrd for Humidity<T>
where
    T: num_traits::Float,
{
    fn partial_cmp(&self, other: &Self) -> Option<::std::cmp::Ordering> {
        self.as_base_units().partial_cmp(&other.as_base_units())
    }
}

implement_display!(Humidity<T>);

#[cfg(test)]
mod test {
    use humidity::*;
    use test_utils::assert_almost_eq;

    // Humidity Units
    #[test]
    fn percent() {
        let t = Humidity::from_percent(50.0);
        let o = t.as_percent();

        assert_almost_eq(o, 50.0);
    }

    #[test]
    fn ratio() {
        let t = Humidity::from_ratio(0.1);
        let o = t.as_ratio();
        assert_almost_eq(o, 0.1);
    }
    // Dewpoint calculation
    #[test]
    fn to_dewpoint1() {
        let humidity = Humidity::from_percent(85.0);
        let temp = Temperature::from_celsius(18.0);
        let dewpoint = humidity.as_dewpoint(temp);
        assert_almost_eq(dewpoint.as_celsius(), 15.44);
    }
    #[test]
    fn to_dewpoint2() {
        let humidity = Humidity::from_percent(40.0);
        let temp = Temperature::from_celsius(5.0);
        let dewpoint = humidity.as_dewpoint(temp);
        assert_almost_eq(dewpoint.as_celsius(), -7.5);
    }
    #[test]
    fn to_dewpoint3() {
        let humidity = Humidity::from_percent(95.0);
        let temp = Temperature::from_celsius(30.0);
        let dewpoint = humidity.as_dewpoint(temp);
        assert_almost_eq(dewpoint.as_celsius(), 29.11);
    }
    #[test]
    fn from_dewpoint1() {
        let temp = Temperature::from_celsius(18.0);
        let dewpoint = Temperature::from_celsius(15.44);
        let rh = Humidity::from_dewpoint(dewpoint, temp);
        assert_almost_eq(rh.as_percent(), 85.0);
    }
    #[test]
    fn vapour_pressure() {
        let humidity = Humidity::from_percent(60.0);
        let temp = Temperature::from_celsius(25.0);
        let vp = humidity.as_vapor_pressure(temp);
        assert_almost_eq(vp.as_hectopascals(), 19.011);
    }
    #[test]
    // also tests as_vapor_pressure() on the fly
    fn absolute_humidity() {
        let humidity = Humidity::from_percent(60.0);
        let temp = Temperature::from_celsius(25.0);
        let density = humidity.as_absolute_humidity(temp);
        assert_almost_eq(density.as_kilograms_per_cubic_meter(), 0.0138166);
    }
    #[test]
    // round-trip test
    fn from_dewpoint2() {
        let humidity = Humidity::from_percent(95.0);
        let temp = Temperature::from_celsius(30.0);
        let dewpoint = humidity.as_dewpoint(temp);
        let rh = Humidity::from_dewpoint(dewpoint, temp);
        assert_almost_eq(humidity.as_percent(), rh.as_percent());
    }

    // Traits
    #[test]
    fn eq() {
        let a = Humidity::from_percent(20.0);
        let b = Humidity::from_percent(20.0);
        assert_eq!(a == b, true);
    }

    #[test]
    fn neq() {
        let a = Humidity::from_percent(20.0);
        let b = Humidity::from_percent(19.0);
        assert_eq!(a == b, false);
    }

    #[test]
    fn cmp() {
        let a = Humidity::from_percent(19.0);
        let b = Humidity::from_percent(20.0);
        assert_eq!(a < b, true);
        assert_eq!(a <= b, true);
        assert_eq!(a > b, false);
        assert_eq!(a >= b, false);
    }
}
