/// We can determine that two colors are equivalent if the Delta is less than
/// a certain value. Typically, two colors with a DE2000 value of less than 1.0
/// are considered indistinguishable.
///
use crate::*;
use delta::Delta;

/// A trait for determining if a color is equivalent to another within a DeltaE tolerance
pub trait DeltaEq<D>
where
    Self: Sized + Delta,
    D: Sized + Delta,
{
    /// Determine if the delta between two colors is within a given `Tolerance`
    fn delta_eq<T: Into<Tolerance>>(self, other: D, tolerance: T) -> bool {
        let tolerance = tolerance.into();
        self.delta(other, tolerance.0.method) <= tolerance.0
    }
}

impl<D, T> DeltaEq<D> for T where D: Delta, T: Delta {}

/// A wrapper around DeltaE for defining a tolerance for the DeltaEq trait
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Tolerance(DeltaE);

impl Tolerance {
    /// Construct a new Tolerance from a value and a DeMethod
    pub fn new(method: DEMethod, value: f32) -> Self {
        Tolerance(
            DeltaE { method, value }
        )
    }
}

impl Default for Tolerance {
    fn default() -> Self {
        Tolerance(DeltaE { method: DE2000, value: 1.0 })
    }
}

impl From<&Self> for Tolerance {
    fn from(tolerance: &Self) -> Self {
        *tolerance
    }
}

impl From<&DeltaE> for Tolerance {
    fn from(de: &DeltaE) -> Self {
        Tolerance(*de)
    }
}

impl From<DeltaE> for Tolerance {
    fn from(de: DeltaE) -> Self {
        Tolerance(de)
    }
}

impl PartialEq<DeltaE> for Tolerance {
    fn eq(&self, other: &DeltaE) -> bool {
        &self.0 == other
    }
}

impl PartialEq<Tolerance> for DeltaE {
    fn eq(&self, other: &Tolerance) -> bool {
        self == &other.0
    }
}
