//! Implements a bridging structure to distinguish between Torque and Energy

use super::*;

/// If you multiply a Force by a Length, we can't tell if you're
/// pushing something along (which requires Energy) or rotating
/// something (which creates a Torque). This struct is what results
/// from the multiplication, and you have to then convert
/// it to whichever you want.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct TorqueEnergy<T>
where
    T: num_traits::Float,
{
    newton_metres: T,
}

impl<T> std::convert::From<TorqueEnergy<T>> for Torque<T>
where
    T: num_traits::Float,
{
    fn from(t: TorqueEnergy<T>) -> Torque<T> {
        Torque::from_newton_metres(t.newton_metres)
    }
}

impl<T> std::convert::From<TorqueEnergy<T>> for Energy<T>
where
    T: num_traits::Float,
{
    fn from(t: TorqueEnergy<T>) -> Energy<T> {
        Energy::from_joules(t.newton_metres)
    }
}

impl<T> Measurement<T> for TorqueEnergy<T>
where
    T: num_traits::Float,
{
    fn as_base_units(&self) -> T {
        self.newton_metres
    }

    fn from_base_units(units: T) -> Self {
        TorqueEnergy {
            newton_metres: units,
        }
    }

    fn get_base_units_name(&self) -> &'static str {
        "Nm||J"
    }
}
