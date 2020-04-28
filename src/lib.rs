#![warn(missing_docs)]

//! Calculate [Delta E](http://www.colorwiki.com/wiki/Delta_E:_The_Color_Difference)
//! (color difference) between two colors in CIE Lab space.
//!
//! # Examples
//!
//! ```
//! use std::error::Error;
//! use deltae::*;
//!
//! fn main() -> Result<(), Box<dyn Error>>{
//!     // Lab from a string
//!     let lab0: LabValue = "89.73, 1.88, -6.96".parse()?;
//!     // Lab directly from values
//!     let lab1 = LabValue {
//!         l: 95.08,
//!         a: -0.17,
//!         b: -10.81,
//!     }.validate()?; // Validate that the values are in range
//!
//!     // Calculate DeltaE between two lab values
//!     let de0 = DeltaE::new(&lab0, &lab1, DE2000);
//!     // Use the Delta trait
//!     let de1 = lab0.delta(lab1, DE2000);
//!     assert_eq!(de0, de1);
//!
//!     // Convert to other color types
//!     let lch0 = LchValue::from(lab0);
//!     let xyz0 = XyzValue::from(lab1);
//!     assert!(lch0.delta_eq(lab0, Tolerance::default()));
//!     let de2 = xyz0.delta(lab1, DE2000);
//!     dbg!(de2);
//!     assert!(xyz0.delta_eq(lab1, Tolerance::default()));
//!
//!     // Calculate DeltaE between different color types
//!     let de2 = lch0.delta(xyz0, DE2000);
//!     assert_eq!(de2.round_to(4), de0.round_to(4));
//!     // There is some loss of accuracy in the conversion.
//!     // Usually rounding to 4 decimal places is more than enough.
//!
//!     println!("{}\n{}\n{}\n{}\n{}\n{}\n{}\n",
//!         lab0, // [L:89.73, a:1.88, b:-6.96]
//!         lab1, // [L:95.08, a:-0.17, b:-10.81]
//!         lch0, // [L:89.73, c:7.2094383, h:285.11572]
//!         xyz0, // [X:0.84576, Y:0.8780792, Z:1.0353166]
//!         de0,  // 5.316941
//!         de1,  // 5.316941
//!         de2,  // 5.316937
//!     );
//!
//!     Ok(())
//! }
//! ```

mod color;
mod convert;
mod delta;
mod eq;
mod parse;
mod round;
mod validate;
#[macro_use]
pub mod matrix;
pub mod illuminant;

#[cfg(test)]
mod tests;

pub use color::*;
pub use convert::*;
pub use delta::*;
pub use eq::*;
pub use parse::*;
pub use round::*;
pub use validate::*;
pub use DEMethod::*;

use std::fmt;
use std::io;

pub(crate) type ValueResult<T> = Result<T, color::ValueError>;

/// ## The measured difference between two colors
///
/// There are many different methods of calculating color difference.
/// Different methods have a specific purpose, mainly in determining the level
/// of tolerance for describing the difference between two colors.
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct DeltaE {
    /// The mathematical method used for calculating color difference
    method: DEMethod,
    /// The calculated value
    value: f32,
}

impl DeltaE {
    /// New `DeltaE` from colors and `DEMethod`.
    pub fn new<A, B>(a: A, b: B, method: DEMethod) -> Self
    where A: Delta, B: Delta {
        a.delta(b, method)
    }

    /// Gets the `DEMethod` used to calculate `DeltaE`
    pub fn method(&self) -> &DEMethod {
        &self.method
    }

    /// Gets the numerical value of the `DeltaE`
    pub fn value(&self) -> &f32 {
        &self.value
    }
}

pub(crate) fn fmt_prec<D: fmt::Display>(f: &fmt::Formatter, d: D) -> String {
    if let Some(precision) = f.precision() {
        format!("{:.*}", precision, d)
    } else {
        format!("{}", d)
    }
}

impl fmt::Display for DeltaE {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", fmt_prec(f, self.value), fmt_prec(f, self.method))
    }
}

#[test]
fn deltae_display() {
    let de = DeltaE { method: DE2000, value: 1.0 };
    assert_eq!(
        format!("{}", de),
        "1 DE2000"
    );
    assert_eq!(
        format!("{:.4}", de),
        "1.0000 DE2000"
    );

    let de = DeltaE { method: DECMC(1.0, 1.0), value: 1.0 };
    assert_eq!(
        format!("{:.4}", de),
        "1.0000 DECMC(1.0000:1.0000)"
    );
}

impl PartialEq<f32> for DeltaE {
    fn eq(&self, f: &f32) -> bool {
        self.value == *f
    }
}

/// One should be careful when ordering DeltaE. A `DE2000:1.0` value is not
/// necessarily the same amount of color difference as a `DE1976:1.0` value.
impl PartialOrd for DeltaE {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.value.partial_cmp(&other.value)
    }
}

/// The most common DeltaE methods
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum DEMethod{
    /// The default DeltaE method
    DE2000,
    /// An implementation of DeltaE with tolerances for Lightness and Chroma
    DECMC(f32, f32),
    /// CIE94 DeltaE implementation, weighted with a tolerance for graphics
    DE1994G,
    /// CIE94 DeltaE implementation, weighted with a tolerance for textiles
    DE1994T,
    /// The original DeltaE implementation, a basic euclidian distance formula
    DE1976,
}

/// DeltaE CMC (1:1)
pub const DECMC1: DEMethod = DECMC(1.0, 1.0);
/// DeltaE CMC (2:1)
pub const DECMC2: DEMethod = DECMC(2.0, 1.0);

impl Eq for DEMethod {}

impl Default for DEMethod {
    fn default() -> DEMethod {
        DEMethod::DE2000
    }
}

impl fmt::Display for DEMethod {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DECMC(tl, tc) => {
                if let Some(p) = f.precision() {
                    write!(f, "DECMC({:.*}:{:.*})", p, tl, p, tc)
                } else {
                    write!(f, "DECMC({}:{})", tl, tc)
                }
            }
            _ => write!(f, "{:?}", self)
        }
    }
}

#[test]
fn de_method_display() {
    assert_eq!(
        format!("{:.4}", DEMethod::DECMC(3.0, 4.0)),
        "DECMC(3.0000:4.0000)"
    );
    assert_eq!(
        format!("{}", DEMethod::DECMC(1.0, 2.0)),
        "DECMC(1:2)"
    );
    assert_eq!(
        format!("{}", DEMethod::DE2000),
        "DE2000"
    );
}
