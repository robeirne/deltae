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
    ///
    /// # Example
    ///
    /// ```
    /// use deltae::{LabValue, Tolerance, DeltaEq, DEMethod::DE2000};
    ///
    /// let lab0 = LabValue::new(50.0, 20.0, 30.0).unwrap();
    /// let lab1 = LabValue::new(50.1, 19.9, 30.2).unwrap();
    /// let tol = Tolerance::new(DE2000, 1.0);
    ///
    /// assert!(lab0.delta_eq(lab1, tol));
    ///
    /// let lab2 = LabValue::new(55.0, 25.0, 35.0).unwrap();
    /// assert!(!lab0.delta_eq(lab2, tol));
    /// ```
    ///
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
    pub fn new(method: DEMethod, value: f64) -> Self {
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

/// Trait to determine if values are almost equivalent despite rounding errors
/// 
/// # Example
///
/// ```
/// use deltae::AlmostEq;
///
/// struct MyStruct(f64);
///
/// impl AlmostEq<Self, f64> for MyStruct {
///     const TOLERANCE: f64 = 1e-5;
///     fn almost_eq(&self, rhs: &Self) -> bool {
///         (self.0 - rhs.0).abs() < Self::TOLERANCE
///     }
/// }
///
/// #[test]
/// fn test_almost_eq() {
///     assert_almost_eq!(MyStruct(1.000000), MyStruct(1.000001));
///     assert_almost_ne!(MyStruct(1.00000), MyStruct(1.00001));
/// }
/// ```
///
pub trait AlmostEq<Rhs, T> {
    /// The maximum difference between to values in order to be considered almost equal
    const TOLERANCE: T;
    /// Should return true if the absolute difference between the two values is less than the
    /// `TOLERANCE`
    fn almost_eq(&self, rhs: &Rhs) -> bool;

    /// Return the tolerance value for the type
    fn tolerance(&self) -> T {
        Self::TOLERANCE
    }
}

/// Convenience macro for the [`AlmostEq`] trait. Panics if the two items are not equivalent within
/// the given tolerance.
///
/// # Example
///
/// ```
/// use deltae::{AlmostEq, assert_almost_eq};
///
/// assert_almost_eq!(1.000000_f64, 1.000001_f64);
/// ```
///
/// [`AlmostEq`]:trait.AlmostEq.html
#[macro_export]
macro_rules! assert_almost_eq {
    ($lhs:expr, $rhs:expr) => {
        if !$lhs.almost_eq(&$rhs) {
            panic!(
                "assertion failed: (left ~= right)\n  left: {:?}\n right: {:?}\n   tol: {:?}",
                $lhs, $rhs, $lhs.tolerance(),
            );
        }
    }
}

/// Convenience macro for the [`AlmostEq`] trait. Panics if the two items are equivalent within the
/// given tolerance.
///
/// # Example
///
/// ```
/// use deltae::{AlmostEq, assert_almost_ne};
///
/// assert_almost_ne!(1.0_f64, 1.1_f64);
/// ```
///
/// [`AlmostEq`]:trait.AlmostEq.html
#[macro_export]
macro_rules! assert_almost_ne {
    ($lhs:expr, $rhs:expr) => {
        if $lhs.almost_eq(&$rhs) {
            panic!(
                "assertion failed: (left !~= right)\n  left: {:?}\n right: {:?}\n   tol: {:?}",
                $lhs, $rhs, $lhs.tolerance(),
            );
        }
    }
}

#[test]
fn almost_eq_ne() {
    assert_almost_eq!(1.0, 1.0);
    assert_almost_eq!(1.000000, 1.000001);
    assert_almost_ne!(1.0, 1.1);
    assert_almost_ne!(1.00000, 1.00001);
}

impl AlmostEq<f64, f64> for f64 {
    const TOLERANCE: f64 = 1e-5;
    fn almost_eq(&self, rhs: &f64) -> bool {
        (self - rhs).abs() < Self::TOLERANCE
    }
}

//impl AlmostEq<f64, f64> for f64 {
    //const TOLERANCE: f64 = 1e-5;
    //fn almost_eq(&self, rhs: &f64) -> bool {
        //(self - rhs).abs() < Self::TOLERANCE
    //}
//}

impl AlmostEq<Self, f64> for DeltaE {
    const TOLERANCE: f64 = f64::TOLERANCE;
    fn almost_eq(&self, rhs: &Self) -> bool {
        self.method == rhs.method
            && self.value.almost_eq(&rhs.value)
    }
}

impl AlmostEq<Self, f64> for LabValue {
    const TOLERANCE: f64 = f64::TOLERANCE;
    fn almost_eq(&self, rhs: &Self) -> bool {
        self.l.almost_eq(&rhs.l)
            && self.a.almost_eq(&rhs.a)
            && self.b.almost_eq(&rhs.b)
    }
}

impl AlmostEq<Self, f64> for nominalize::RgbNominalValue {
    const TOLERANCE: f64 = f64::TOLERANCE;
    fn almost_eq(&self, rhs: &Self) -> bool {
        self.r.almost_eq(&rhs.r)
            && self.g.almost_eq(&rhs.g)
            && self.b.almost_eq(&rhs.b)
    }
}
