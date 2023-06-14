//! Utility code for writing tests.

const DEFAULT_DELTA: f64 = 1e-5;

/// Check two floating point values are approximately equal
pub fn almost_eq<T>(a: T, b: T) -> bool
where
    T: num_traits::Float,
{
    almost_eq_delta(a, b, DEFAULT_DELTA)
}

/// Check two floating point values are approximately equal using some given delta (a fraction of the inputs)
pub fn almost_eq_delta<T>(a: T, b: T, d: T) -> bool
where
    T: num_traits::Float,
{
    (abs(a - b) / a) < d
}

/// Assert two floating point values are approximately equal
pub fn assert_almost_eq<T>(a: T, b: T)
where
    T: num_traits::Float,
{
    assert_almost_eq_delta(a, b, DEFAULT_DELTA);
}

/// Assert two floating point values are approximately equal using some given delta (a fraction of the inputs)
pub fn assert_almost_eq_delta<T>(a: T, b: T, d: T)
where
    T: num_traits::Float + ::std::fmt::Debug,
{
    if !almost_eq_delta(a, b, d) {
        panic!("assertion failed: {:?} != {:?} (within {:?})", a, b, d);
    }
}

/// This function doesn't seem to be available no `#![no_std]` so we re-
/// implement it here.
fn abs<T>(x: T) -> T
where
    T: num_traits::Float,
{
    if x > 0.0 {
        x
    } else {
        -x
    }
}
