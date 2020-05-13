//! Standard Illuminants for Chromatic Adaptation.
//! See also: [BruceLindbloom.com](http://www.brucelindbloom.com/index.html?Eqn_RGB_XYZ_Matrix.html)

use crate::{XyzValue, matrix::*, matrix3x1, chromatic_adaptation::*};

/// Tungsten-filament (incandescent)
pub const A:   Matrix3x1 = matrix3x1![1.09850; 1.00000; 0.35585;];
/// Daylight simulation at noon (4874°K)
pub const B:   Matrix3x1 = matrix3x1![0.99072; 1.00000; 0.85223;];
/// Daylight simulation average (6774°K)
pub const C:   Matrix3x1 = matrix3x1![0.98074; 1.00000; 1.18232;];
/// Natural daylight at horizon (5003°K)
pub const D50: Matrix3x1 = matrix3x1![0.96422; 1.00000; 0.82521;];
/// Natural daylight at mid-morning (5503°K)
pub const D55: Matrix3x1 = matrix3x1![0.95682; 1.00000; 0.92149;];
/// Natural daylight at noon (6504°K)
pub const D65: Matrix3x1 = matrix3x1![0.95047; 1.00000; 1.08883;];
/// Natural daylight in north sky (7504°K)
pub const D75: Matrix3x1 = matrix3x1![0.94972; 1.00000; 1.22638;];
/// Equal energy radiator (constant spectral distribution)
pub const E:   Matrix3x1 = matrix3x1![1.00000; 1.00000; 1.00000;];
/// Fluorescent (standard)
pub const F2:  Matrix3x1 = matrix3x1![0.99186; 1.00000; 0.67393;];
/// Fluroescent (Broadband)
pub const F7:  Matrix3x1 = matrix3x1![0.95041; 1.00000; 1.08747;];
/// Fluorescent (Narrowband)
pub const F11: Matrix3x1 = matrix3x1![1.00962; 1.00000; 0.64350;];

/// Common Standard Illuminant Types.
/// This list is not exhaustive.
#[non_exhaustive]
#[derive(Debug, Copy, Clone)]
pub enum Illuminant {
    /// Tungsten-filament (incandescent)
    A,
    /// Daylight simulation at noon (4874°K)
    B,
    /// Daylight simulation average (6774°K)
    C,
    /// Natural daylight at horizon (5003°K)
    D50,
    /// Natural daylight at mid-morning (5503°K)
    D55,
    /// Natural daylight at noon (6504°K)
    D65,
    /// Natural daylight in north sky (7504°K)
    D75,
    /// Equal energy radiator (constant spectral distribution)
    E,
    /// Fluorescent (standard)
    F2,
    /// Fluroescent (Broadband)
    F7,
    /// Fluorescent (Narrowband)
    F11,
    /// Any arbitrary Illuminant
    Other(Matrix3x1)
}

impl Illuminant {
    /// Get the `XyzValue` of the `Illuminant` type
    pub fn xyz(self) -> XyzValue {
        Matrix3x1::from(self).into()
    }

    /// Returns an illuminant's cone response domain via a 3x3
    /// chromatic adaptation matrix
    pub fn cone_response_domain(&self, method_matrix: Matrix3x3) -> ConeResponseDomain {
        (method_matrix * Matrix3x1::from(*self)).into()
    }

    /// Create a custom Illuminant
    pub fn other(x: f64, y: f64, z: f64) -> Self {
        Illuminant::Other(
            matrix3x1![x; y; z;]
        )
    }
}

impl Default for Illuminant {
    fn default() -> Self {
        Illuminant::D50
    }
}

impl PartialEq for Illuminant {
    fn eq(&self, rhs: &Self) -> bool {
        Matrix3x1::from(*self) == Matrix3x1::from(*rhs)
    }
}

impl Eq for Illuminant {}

impl From<Illuminant> for Matrix3x1 {
    fn from(illum: Illuminant) -> Self {
        match illum {
           Illuminant::A   => A,
           Illuminant::B   => B,
           Illuminant::C   => C,
           Illuminant::D50 => D50,
           Illuminant::D55 => D55,
           Illuminant::D65 => D65,
           Illuminant::D75 => D75,
           Illuminant::E   => E,
           Illuminant::F2  => F2,
           Illuminant::F7  => F7,
           Illuminant::F11 => F11,
           Illuminant::Other(matrix) => matrix,
        }      
    }
}
