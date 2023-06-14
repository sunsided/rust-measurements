/// The `Measurement` trait and the `implement_measurement!` macro
/// provides a common way for various measurements to be implemented.
///
/// # Example
/// ```
/// #![no_std]
/// // Importing the `implement_measurement` macro from the external crate is important
/// #[macro_use]
/// extern crate measurements;
///
/// use measurements::Measurement;
///
/// struct Cubits {
///     forearms: T
/// }
///
/// impl<T> Measurement<T> for Cubits {
///     fn as_base_units(&self) -> T {
///         self.forearms
///     }
///
///     fn from_base_units(units: T) -> Self {
///         Cubits { forearms: units }
///     }
///
///    fn get_base_units_name(&self) -> &'static str {
///        "cu"
///    }
/// }
///
/// // Invoke the macro to automatically implement Add, Sub, etc...
/// implement_measurement! { Cubits }
///
/// // The main function here is only included to make doc test_utils compile.
/// // You should't need it in your own code.
/// fn main() { }
/// ```

#[cfg(feature = "no-std")]
use core as std;

#[cfg(feature = "no-std")]
use core::num::Float;

/// All measurements implement this.
///
/// It provides conversion functions to and from raw numbers.
pub trait Measurement<T>
where
    T: num_traits::Float,
{
    /// Returns a string containing the most appropriate units for this quantity,
    /// and a floating point value representing this quantity in those units.
    /// Useful when, for example, a length might be in millimeters if it is very small,
    /// or kilometers when it is very large.
    ///
    /// The default implementation always selects the base unit. Override in your
    /// Measurement  impl to select better units if required.
    fn get_appropriate_units(&self) -> (&'static str, T) {
        (self.get_base_units_name(), self.as_base_units())
    }

    /// Given a list of units and their scale relative to the base unit,
    /// select the most appropriate one.
    ///
    /// The list must be smallest to largest, e.g. ("nanometre", 10-9) to
    /// ("kilometre", 10e3)
    fn pick_appropriate_units(&self, list: &[(&'static str, T)]) -> (&'static str, T) {
        for &(unit, ref scale) in list.iter().rev() {
            let value = self.as_base_units() / scale;
            if value >= 1.0 || value <= -1.0 {
                return (unit, value);
            }
        }
        (list[0].0, self.as_base_units() / list[0].1)
    }

    /// Return the base unit for this type, as a string.
    /// For example "kilograms"
    fn get_base_units_name(&self) -> &'static str;

    /// Get this quantity in the base units
    fn as_base_units(&self) -> T;

    /// Create a new quantity from the base units
    fn from_base_units(units: T) -> Self;
}

/// This is a special macro that creates the code to implement
/// `std::fmt::Display`.
#[macro_export]
macro_rules! implement_display {
    ($($t:ty)*) => ($(

        impl<T> ::std::fmt::Display for $t {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                let (unit, value) = self.get_appropriate_units();
                value.fmt(f)?;      // Value
                write!(f, "\u{00A0}{}", unit)
            }
        }
    )*)
}

/// This is a special macro that creates the code to implement
/// operator and comparison overrides.
#[macro_export]
macro_rules! implement_measurement {
    ($($t:ty)*) => ($(

        implement_display!( $t );

        impl<T> ::std::ops::Add for $t {
            type Output = Self;

            fn add(self, rhs: Self) -> Self {
                Self::from_base_units(self.as_base_units() + rhs.as_base_units())
            }
        }

        impl<T> ::std::ops::Sub for $t {
            type Output = Self;

            fn sub(self, rhs: Self) -> Self {
                Self::from_base_units(self.as_base_units() - rhs.as_base_units())
            }
        }

        // Dividing a `$t` by another `$t` returns a ratio.
        //
        impl<T> ::std::ops::Div<$t> for $t {
            type Output = T;

            fn div(self, rhs: Self) -> T {
                self.as_base_units() / rhs.as_base_units()
            }
        }

        // Dividing a `$t` by a factor returns a new portion of the measurement.
        //
        impl<T> ::std::ops::Div<T> for $t {
            type Output = Self;

            fn div(self, rhs: T) -> Self {
                Self::from_base_units(self.as_base_units() / rhs)
            }
        }

        // Multiplying a `$t` by a factor increases (or decreases) that
        // measurement a number of times.
        impl<T> ::std::ops::Mul<T> for $t {
            type Output = Self;

            fn mul(self, rhs: T) -> Self {
                Self::from_base_units(self.as_base_units() * rhs)
            }
        }

        // Multiplying `$t` by a factor is commutative
        impl<T> ::std::ops::Mul<$t> for T {
            type Output = $t;

            fn mul(self, rhs: $t) -> $t {
                rhs * self
            }
        }

        impl<T> ::std::cmp::Eq for $t { }
        impl<T> ::std::cmp::PartialEq for $t {
            fn eq(&self, other: &Self) -> bool {
                self.as_base_units() == other.as_base_units()
            }
        }

        impl<T> ::std::cmp::PartialOrd for $t {
            fn partial_cmp(&self, other: &Self) -> Option<::std::cmp::Ordering> {
                self.as_base_units().partial_cmp(&other.as_base_units())
            }
        }
    )*)
}
