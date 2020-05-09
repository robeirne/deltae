//! Definitions for manipulating types into a 3x3 matrix for color conversions

use crate::*;
use nominalize::*;
use std::ops::{Index, Mul};

/// Create a new [`Matrix3x3`] from a list of floats in column-major order
///
/// # Example
///
/// ```
/// use deltae::{matrix3x3, matrix::Matrix3x3};
///
/// let matrix = matrix3x3![
///     0.5767309, 0.1855540, 0.1881852;
///     0.2973769, 0.6273491, 0.0752741;
///     0.0270343, 0.0706872, 0.9911085;
/// ];
///
/// assert_eq!(matrix[2], 0.0270343);
/// ```
///
/// [`Matrix3x3`]: matrix/struct.Matrix3x3.html
#[macro_export]
macro_rules! matrix3x3 {
    [
        $x0:expr, $y0:expr, $z0:expr;
        $x1:expr, $y1:expr, $z1:expr;
        $x2:expr, $y2:expr, $z2:expr;
    ] => {
        Matrix3x3::new(
            $x0, $y0, $z0,
            $x1, $y1, $z1,
            $x2, $y2, $z2,
        )
    };
}

/// # A 3x3 matrix of floats
///
/// |    | c     | o     | l     |
/// |:-- |:----- |:----- |:----- |
/// | r  | (0,0) | (1,0) | (2,0) |
/// | o  | (0,1) | (1,1) | (2,1) |
/// | w  | (0,2) | (1,2) | (2,2) |
///
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct Matrix3x3 {
    /// The internal contents of te matrix
    pub inner: [f32; 9],
}

impl Matrix3x3 {
    /// Create a new [`Matrix3x3`](struct.Matrix3x3.html) from a list of floats in column-major
    /// order.
    pub const fn new(
        x0: f32, y0: f32, z0: f32,
        x1: f32, y1: f32, z1: f32,
        x2: f32, y2: f32, z2: f32
    ) -> Self {
        Matrix3x3 {
            inner: [
                x0, x1, x2,
                y0, y1, y2,
                z0, z1, z2,
            ]
        }
    }

    /// Returns a column as a 3x1 matrix.
    /// Panics if the `col` is greater than 2
    pub fn col(&self, col: usize) -> Matrix3x1 {
        if col > 2 {
            panic!("column index is {} but the column length is 2", col);
        } else {
            Matrix3x1::new(self[(col,0)], self[(col,1)], self[(col,2)])
        }
    }

    fn from_cols(col0: Matrix3x1, col1: Matrix3x1, col2: Matrix3x1) -> Self {
        matrix3x3![
            col0[0], col1[0], col2[0];
            col0[1], col1[1], col2[1];
            col0[2], col1[2], col2[2];
        ]
    }
}

impl Index<usize> for Matrix3x3 {
    type Output = f32;
    fn index(&self, idx: usize) -> &Self::Output {
       &self.inner[idx]
    }
}

impl Index<(usize, usize)> for Matrix3x3 {
    type Output = f32;
    fn index(&self, idx: (usize, usize)) -> &Self::Output {
        if idx.0 > 2 {
            panic!("index out of bounds: the width is 3, but the width index is {}", idx.0);
        }

        if idx.1 > 2 {
            panic!("index out of bounds: the height is 3, but the height index is {}", idx.1);
        }

        &self.inner[idx.0 * 3 + idx.1]
    }
}

/// An iterator over the values in a matrix in column-major order
pub struct MatrixIter<'a> {
    values: Vec<&'a f32>,
    index: usize,
}

impl<'a> Iterator for MatrixIter<'a> {
    type Item = &'a f32;
    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.values.len() {
            self.index += 1;
            Some(self.values[self.index - 1])
        } else {
            None
        }
    }
}

impl<'a> IntoIterator for &'a Matrix3x3 {
    type Item = &'a f32;
    type IntoIter = MatrixIter<'a>;
    fn into_iter(self) -> Self::IntoIter {
        MatrixIter {
            values: self.inner.iter().collect(),
            index: 0,
        }
    }
}

impl <'a> IntoIterator for &'a Matrix3x1 {
    type Item = &'a f32;
    type IntoIter = MatrixIter<'a>;
    fn into_iter(self) -> Self::IntoIter {
        MatrixIter {
            values: self.inner.iter().collect(),
            index: 0,
        }
    }
}

#[test]
fn matrix_iter() {
    let mut iter = TEST_MATRIX_3X3.into_iter();
    assert_eq!(iter.next(), Some(&0.0));
    assert_eq!(iter.next(), Some(&0.1));
    assert_eq!(iter.next(), Some(&0.2));
    assert_eq!(iter.next(), Some(&1.0));
    assert_eq!(iter.next(), Some(&1.1));
    assert_eq!(iter.next(), Some(&1.2));
    assert_eq!(iter.next(), Some(&2.0));
    assert_eq!(iter.next(), Some(&2.1));
    assert_eq!(iter.next(), Some(&2.2));
    assert_eq!(iter.next(), None);
}

impl AlmostEq<Self, f32> for Matrix3x3 {
    const TOLERANCE: f32 = f32::TOLERANCE;
    fn almost_eq(&self, rhs: &Self) -> bool {
        self.into_iter()
            .zip(rhs.into_iter())
            .all(|(a, b)| a.almost_eq(b))
    }
}

impl AlmostEq<Self, f32> for Matrix3x1 {
    const TOLERANCE: f32 = f32::TOLERANCE;
    fn almost_eq(&self, rhs: &Self) -> bool {
        self.into_iter()
            .zip(rhs.into_iter())
            .all(|(a, b)| a.almost_eq(b))
    }
}

#[cfg(test)]
const TEST_MATRIX_3X3: Matrix3x3 = matrix3x3![
    0.0, 1.0, 2.0;
    0.1, 1.1, 2.1;
    0.2, 1.2, 2.2;
];

#[cfg(test)]
const TEST_MATRIX_3X1: Matrix3x1 = Matrix3x1::new(0.0, 0.1, 0.2);

#[cfg(test)]
const TEST_MATRIX_3X1_ANSWER: Matrix3x1 = Matrix3x1::new(0.50, 0.53, 0.56);

#[cfg(test)]
const TEST_MATRIX_3X3_ANSWER: Matrix3x3 = matrix3x3![
    0.5, 3.5, 6.5;
    0.53, 3.83, 7.13;
    0.56, 4.16, 7.76;
];

#[test]
fn matrix_index() {
    assert_eq!(TEST_MATRIX_3X3[0], 0.0);
    assert_eq!(TEST_MATRIX_3X3[1], 0.1);
    assert_eq!(TEST_MATRIX_3X3[2], 0.2);
    assert_eq!(TEST_MATRIX_3X3[3], 1.0);
    assert_eq!(TEST_MATRIX_3X3[4], 1.1);
    assert_eq!(TEST_MATRIX_3X3[5], 1.2);
    assert_eq!(TEST_MATRIX_3X3[6], 2.0);
    assert_eq!(TEST_MATRIX_3X3[7], 2.1);
    assert_eq!(TEST_MATRIX_3X3[8], 2.2);

    assert_eq!(TEST_MATRIX_3X3[(0,0)], 0.0);
    assert_eq!(TEST_MATRIX_3X3[(0,1)], 0.1);
    assert_eq!(TEST_MATRIX_3X3[(0,2)], 0.2);
    assert_eq!(TEST_MATRIX_3X3[(1,0)], 1.0);
    assert_eq!(TEST_MATRIX_3X3[(1,1)], 1.1);
    assert_eq!(TEST_MATRIX_3X3[(1,2)], 1.2);
    assert_eq!(TEST_MATRIX_3X3[(2,0)], 2.0);
    assert_eq!(TEST_MATRIX_3X3[(2,1)], 2.1);
    assert_eq!(TEST_MATRIX_3X3[(2,2)], 2.2);
}

macro_rules! index_panics {
    ($name:ident, $index:expr) => {
        #[test]
        #[should_panic]
        fn $name() {
            let _panic = TEST_MATRIX_3X3[$index];
        }
    }
}

index_panics!(index_panic_9, 9);
index_panics!(index_panic_3_0, (3,0));
index_panics!(index_panic_0_3, (0,3));
index_panics!(index_panic_3_3, (3,3));

/// A 3x1 Matrix for color conversion calculations
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct Matrix3x1 {
    inner: [f32; 3],
}

impl Matrix3x1 {
    /// Construct a new Matrix3x1 from 3 floats
    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        Matrix3x1 {
            inner: [x, y, z]
        }
    }
}

impl From<&[f32; 3]> for Matrix3x1 {
    fn from(slice: &[f32; 3]) -> Self {
        Matrix3x1::new(slice[0], slice[1], slice[2])
    }
}

impl From<XyzValue> for Matrix3x1 {
    fn from(xyz: XyzValue) -> Self {
        Matrix3x1::new(xyz.x, xyz.y, xyz.z)
    }
}

impl From<RgbNominalValue> for Matrix3x1 {
    fn from(rgb: RgbNominalValue) -> Self {
        Matrix3x1::new(rgb.r, rgb.g, rgb.b)
    }
}

impl From<RgbValue> for Matrix3x1 {
    fn from(rgb: RgbValue) -> Self {
        Matrix3x1::from(rgb.nominalize())
    }
}

impl From<Matrix3x1> for RgbValue {
    fn from(matrix: Matrix3x1) -> Self {
        RgbNominalValue::from(matrix).into()
    }
}

impl From<Matrix3x1> for RgbNominalValue {
    fn from(matrix: Matrix3x1) -> Self {
        RgbNominalValue {
            r: matrix[0],
            g: matrix[1],
            b: matrix[2],
        }.clamp()
    }
}

impl From<Matrix3x1> for XyzValue {
    fn from(matrix: Matrix3x1) -> Self {
        XyzValue {
            x: matrix[0],
            y: matrix[1],
            z: matrix[2],
        }
    }
}

impl Index<usize> for Matrix3x1 {
    type Output = f32;
    fn index(&self, idx: usize) -> &Self::Output {
        &self.inner[idx]
    }
}


impl Mul<Self> for Matrix3x3 {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Matrix3x3::from_cols(
            self.col(0) * rhs,
            self.col(1) * rhs,
            self.col(2) * rhs,
        )
    }
}

#[test]
fn matrix3x3_mul_matrix3x3() {
    assert_almost_eq!(
        TEST_MATRIX_3X3 * TEST_MATRIX_3X3,
        TEST_MATRIX_3X3_ANSWER
    )
}

impl Mul<Matrix3x1> for Matrix3x3 {
    type Output = Matrix3x1;
    fn mul(self, rhs: Matrix3x1) -> Self::Output {
        let a = self[(0,0)] * rhs[0]
              + self[(1,0)] * rhs[1]
              + self[(2,0)] * rhs[2];

        let b = self[(0,1)] * rhs[0]
              + self[(1,1)] * rhs[1]
              + self[(2,1)] * rhs[2];
                         
        let c = self[(0,2)] * rhs[0]
              + self[(1,2)] * rhs[1]
              + self[(2,2)] * rhs[2];

        Matrix3x1::new(a, b, c)
    }
}

impl Mul<Matrix3x3> for Matrix3x1 {
    type Output = Matrix3x1;
    fn mul(self, rhs: Matrix3x3) -> Self::Output {
        rhs * self
    }
}

#[test]
fn matrix3x3_mul_matrix3x1() {
    assert_almost_eq!(TEST_MATRIX_3X3 * TEST_MATRIX_3X1, TEST_MATRIX_3X1_ANSWER);
    assert_almost_eq!(TEST_MATRIX_3X1 * TEST_MATRIX_3X3, TEST_MATRIX_3X1_ANSWER);
}

impl RgbNominalValue {
    fn compand_srgb_inv(self) -> Self {
        RgbNominalValue {
            r: compand_srgb_inv(self.r),
            g: compand_srgb_inv(self.g),
            b: compand_srgb_inv(self.b),
        }
    }

    fn compand_srgb(self) -> Self {
        RgbNominalValue {
            r: compand_srgb(self.r),
            g: compand_srgb(self.g),
            b: compand_srgb(self.b),
        }
    }
}

fn compand_srgb_inv(val: f32) -> f32 {
    if val <= 0.04045 {
        val / 12.92
    } else {
        ((val + 0.055) / 1.055).powf(2.4)
    }
}

fn compand_srgb(val: f32) -> f32 {
    if val <= 0.0031308 {
        val * 12.92
    } else {
        1.055 * val.powf(1.0/2.4) - 0.055
    }
}

/// Trait to raise a value to a power `T`
pub trait Pow<T> {
    /// The product of raising a value to a power
    type Output;
    /// Rases a value to a power
    fn pow(self, power: T) -> Self::Output;
}

impl Pow<f32> for Matrix3x3 {
    type Output = Self;
    fn pow(self, power: f32) -> Self::Output {
        matrix3x3![
            self[0].powf(power), self[3].powf(power), self[6].powf(power);
            self[1].powf(power), self[4].powf(power), self[7].powf(power);
            self[2].powf(power), self[5].powf(power), self[8].powf(power);
        ]
    }
}

impl Pow<f32> for Matrix3x1 {
    type Output = Self;
    fn pow(self, power: f32) -> Self::Output {
        Matrix3x1::new(
            self[0].powf(power),
            self[1].powf(power),
            self[2].powf(power),
        )
    }
}

// From Bruce Lindbloom
// http://www.brucelindbloom.com/Eqn_RGB_XYZ_Matrix.html
/// Matrix for converting AdobeRGB to XYZ with D65 Illuminant
pub const ADOBERGB_1998_D65_RGB2XYZ: Matrix3x3 = matrix3x3![
    0.5767309, 0.1855540, 0.1881852;
    0.2973769, 0.6273491, 0.0752741;
    0.0270343, 0.0706872, 0.9911085;
];
/// Matrix for converting XYZ to AdobeRGB with D65 Illuminant
pub const ADOBERGB_1998_D65_XYZ2RGB: Matrix3x3 = matrix3x3![
    2.0413690, -0.5649464, -0.3446944;
    -0.9692660, 1.8760108, 0.0415560;
    0.0134474, -0.1183897, 1.0154096;
];

/// Matrix for converting AppleRGB to XYZ with D65 Illuminant
pub const APPLERGB_D65_RGB2XYZ: Matrix3x3 = matrix3x3![
    0.4497288, 0.3162486, 0.1844926;
    0.2446525, 0.6720283, 0.0833192;
    0.0251848, 0.1411824, 0.9224628;
];
/// Matrix for converting XYZ to AppleRGB with D65 Illuminant
pub const APPLERGB_D65_XYZ2RGB: Matrix3x3 = matrix3x3![
    2.9515373, -1.2894116, -0.4738445;
    -1.0851093, 1.9908566, 0.0372026;
    0.0854934, -0.2694964, 1.0912975;
];

/// Matrix for converting BestRGB to XYZ with D50 Illuminant
pub const BESTRGB_D50_RGB2XYZ: Matrix3x3 = matrix3x3![
    0.6326696, 0.2045558, 0.1269946;
    0.2284569, 0.7373523, 0.0341908;
    0.0000000, 0.0095142, 0.8156958;
];
/// Matrix for converting XYZ to BestRGB with D50 Illuminant
pub const BESTRGB_D50_XYZ2RGB: Matrix3x3 = matrix3x3![
    1.7552599, -0.4836786, -0.2530000;
    -0.5441336, 1.5068789, 0.0215528;
    0.0063467, -0.0175761, 1.2256959;
];

/// Matrix for converting BetaRGB to XYZ with D50 Illuminant
pub const BETARGB_D50_RGB2XYZ: Matrix3x3 = matrix3x3![
    0.6712537, 0.1745834, 0.1183829;
    0.3032726, 0.6637861, 0.0329413;
    0.0000000, 0.0407010, 0.7845090;
];
/// Matrix for converting XYZ to BetaRGB with D50 Illuminant
pub const BETARGB_D50_XYZ2RGB: Matrix3x3 = matrix3x3![
    1.6832270, -0.4282363, -0.2360185;
    -0.7710229, 1.7065571, 0.0446900;
    0.0400013, -0.0885376, 1.2723640;
];

/// Matrix for converting BruceRGB to XYZ with D65 Illuminant
pub const BRUCERGB_D65_RGB2XYZ: Matrix3x3 = matrix3x3![
    0.4674162, 0.2944512, 0.1886026;
    0.2410115, 0.6835475, 0.0754410;
    0.0219101, 0.0736128, 0.9933071;
];
/// Matrix for converting XYZ to BruceRGB with D65 Illuminant
pub const BRUCERGB_D65_XYZ2RGB: Matrix3x3 = matrix3x3![
    2.7454669, -1.1358136, -0.4350269;
    -0.9692660, 1.8760108, 0.0415560;
    0.0112723, -0.1139754, 1.0132541;
];

/// Matrix for converting CIERGB to XYZ with E Illuminant
pub const CIERGB_E_RGB2XYZ: Matrix3x3 = matrix3x3![
    0.4887180, 0.3106803, 0.2006017;
    0.1762044, 0.8129847, 0.0108109;
    0.0000000, 0.0102048, 0.9897952;
];
/// Matrix for converting XYZ to CIERGB with E Illuminant
pub const CIERGB_E_XYZ2RGB: Matrix3x3 = matrix3x3![
    2.3706743, -0.9000405, -0.4706338;
    -0.5138850, 1.4253036, 0.0885814;
    0.0052982, -0.0146949, 1.0093968;
];

/// Matrix for converting ColorMatchRGB to XYZ with D50 Illuminant
pub const COLORMATCHRGB_D50_RGB2XYZ: Matrix3x3 = matrix3x3![
    0.5093439, 0.3209071, 0.1339691;
    0.2748840, 0.6581315, 0.0669845;
    0.0242545, 0.1087821, 0.6921735;
];
/// Matrix for converting XYZ to ColorMatchRGB with D50 Illuminant
pub const COLORMATCHRGB_D50_XYZ2RGB: Matrix3x3 = matrix3x3![
    2.6422874, -1.2234270, -0.3930143;
    -1.1119763, 2.0590183, 0.0159614;
    0.0821699, -0.2807254, 1.4559877;
];

/// Matrix for converting DonRGB4 to XYZ with D50 Illuminant
pub const DONRGB4_D50_RGB2XYZ: Matrix3x3 = matrix3x3![
    0.6457711, 0.1933511, 0.1250978;
    0.2783496, 0.6879702, 0.0336802;
    0.0037113, 0.0179861, 0.8035125;
];
/// Matrix for converting XYZ to DonRGB4 with D50 Illuminant
pub const DONRGB4_D50_XYZ2RGB: Matrix3x3 = matrix3x3![
    1.7603902, -0.4881198, -0.2536126;
    -0.7126288, 1.6527432, 0.0416715;
    0.0078207, -0.0347411, 1.2447743;
];

/// Matrix for converting ECIRGB to XYZ with D50 Illuminant
pub const ECIRGB_D50_RGB2XYZ: Matrix3x3 = matrix3x3![
    0.6502043, 0.1780774, 0.1359384;
    0.3202499, 0.6020711, 0.0776791;
    0.0000000, 0.0678390, 0.7573710;
];
/// Matrix for converting XYZ to ECIRGB with D50 Illuminant
pub const ECIRGB_D50_XYZ2RGB: Matrix3x3 = matrix3x3![
    1.7827618, -0.4969847, -0.2690101;
    -0.9593623, 1.9477962, -0.0275807;
    0.0859317, -0.1744674, 1.3228273;
];

/// Matrix for converting EktaSpace to XYZ with D50 Illuminant
pub const EKTASPACE_PS5_D50_RGB2XYZ: Matrix3x3 = matrix3x3![
    0.5938914, 0.2729801, 0.0973485;
    0.2606286, 0.7349465, 0.0044249;
    0.0000000, 0.0419969, 0.7832131;
];
/// Matrix for converting XYZ to EktaSpace with D50 Illuminant
pub const EKTASPACE_PS5_D50_XYZ2RGB: Matrix3x3 = matrix3x3![
    2.0043819, -0.7304844, -0.2450052;
    -0.7110285, 1.6202126, 0.0792227;
    0.0381263, -0.0868780, 1.2725438;
];

/// Matrix for converting NTSCRGB to XYZ with C Illuminant
pub const NTSCRGB_C_RGB2XYZ: Matrix3x3 = matrix3x3![
    0.6068909, 0.1735011, 0.2003480;
    0.2989164, 0.5865990, 0.1144845;
    0.0000000, 0.0660957, 1.1162243;
];
/// Matrix for converting XYZ to NTSCRGB with C Illuminant
pub const NTSCRGB_C_XYZ2RGB: Matrix3x3 = matrix3x3![
    1.9099961, -0.5324542, -0.2882091;
    -0.9846663, 1.9991710, -0.0283082;
    0.0583056, -0.1183781, 0.8975535;
];

/// Matrix for converting PAL/SECAM RGB to XYZ with D65 Illuminant
pub const PALSECAMRGB_D65_RGB2XYZ: Matrix3x3 = matrix3x3![
    0.4306190, 0.3415419, 0.1783091;
    0.2220379, 0.7066384, 0.0713236;
    0.0201853, 0.1295504, 0.9390944;
];
/// Matrix for converting XYZ to PAL/SECAM RGB with D65 Illuminant
pub const PALSECAMRGB_D65_XYZ2RGB: Matrix3x3 = matrix3x3![
    3.0628971, -1.3931791, -0.4757517;
    -0.9692660, 1.8760108, 0.0415560;
    0.0678775, -0.2288548, 1.0693490;
];

/// Matrix for converting ProPhotoRGB to XYZ with D50 Illuminant
pub const PROPHOTORGB_D50_RGB2XYZ: Matrix3x3 = matrix3x3![
    0.7976749, 0.1351917, 0.0313534;
    0.2880402, 0.7118741, 0.0000857;
    0.0000000, 0.0000000, 0.8252100;
];
/// Matrix for converting XYZ to ProPhotoRGB with D50 Illuminant
pub const PROPHOTORGB_D50_XYZ2RGB: Matrix3x3 = matrix3x3![
    1.3459433, -0.2556075, -0.0511118;
    -0.5445989, 1.5081673, 0.0205351;
    0.0000000, 0.0000000, 1.2118128;
];

/// Matrix for converting SMPTE RGB to XYZ with D65 Illuminant
pub const SMPTERGB_D65_RGB2XYZ: Matrix3x3 = matrix3x3![
    0.3935891, 0.3652497, 0.1916313;
    0.2124132, 0.7010437, 0.0865432;
    0.0187423, 0.1119313, 0.9581563;
];
/// Matrix for converting XYZ to SMPTE RGB with D65 Illuminant
pub const SMPTERGB_D65_XYZ2RGB: Matrix3x3 = matrix3x3![
    3.5053960, -1.7394894, -0.5439640;
    -1.0690722, 1.9778245, 0.0351722;
    0.0563200, -0.1970226, 1.0502026;
];

/// Matrix for converting sRGB to XYZ with D65 Illuminant
pub const SRGB_D65_RGB2XYZ: Matrix3x3 = matrix3x3![
    0.4124564, 0.3575761, 0.1804375;
    0.2126729, 0.7151522, 0.0721750;
    0.0193339, 0.1191920, 0.9503041;
];
/// Matrix for converting XYZ to sRGB with D65 Illuminant
pub const SRGB_D65_XYZ2RGB: Matrix3x3 = matrix3x3![
    3.2404542, -1.5371385, -0.4985314;
    -0.9692660, 1.8760108, 0.0415560;
    0.0556434, -0.2040259, 1.0572252;
];

/// Matrix for converting WideGamutRGB to XYZ with D65 Illuminant
pub const WIDEGAMUTRGB_D50_RGB2XYZ: Matrix3x3 = matrix3x3![
    0.7161046, 0.1009296, 0.1471858;
    0.2581874, 0.7249378, 0.0168748;
    0.0000000, 0.0517813, 0.7734287;
];
/// Matrix for converting XYZ to WideGamutRGB with D65 Illuminant
pub const WIDEGAMUTRGB_D50_XYZ2RGB: Matrix3x3 = matrix3x3![
    1.4628067, -0.1840623, -0.2743606;
    -0.5217933, 1.4472381, 0.0677227;
    0.0349342, -0.0968930, 1.2884099;
];
