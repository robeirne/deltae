//! Definitions for manipulating types into a 3x3 matrix for color conversions

use crate::*;
use nominalize::*;
use std::ops::{Index, Mul};

/// Create a new [`Matrix3x3`] from a list of floats
///
/// # Example
///
/// ```
/// use deltae::{matrix3x3, matrix::Matrix3x3};
///
/// let matrix = matrix3x3![
///     0.5767309, 0.1855540, 0.1881852,
///     0.2973769, 0.6273491, 0.0752741,
///     0.0270343, 0.0706872, 0.9911085,
/// ];
/// ```
///
/// [`Matrix3x3`]: matrix/struct.Matrix3x3.html
#[macro_use]
macro_rules! matrix3x3 {
    [
        $x0:expr, $y0:expr, $z0:expr,
        $x1:expr, $y1:expr, $z1:expr,
        $x2:expr, $y2:expr, $z2:expr $(,)?
    ] => {
        Matrix3x3::new(
            $x0, $y0, $z0,
            $x1, $y1, $z1,
            $x2, $y2, $z2,
        )
    };
}

/// # A 3x3 matrix of 9 floats
///
/// | Color | X     | Y     | Z     |
/// |:----- |:----- |:----- |:----- |
/// | Red   | (0,0) | (0,1) | (0,2) |
/// | Green | (1,0) | (1,1) | (1,2) |
/// | Blue  | (2,0) | (2,1) | (2,2) |
///
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct Matrix3x3 {
    /// The internal contents of te matrix
    pub inner: [f32; 9],
}

impl Matrix3x3 {
    /// Create a new [`Matrix3x3`] from 3 [`XyzValue`]s
    ///
    /// [`Matrix3x3`]: struct.Matrix3x3.html
    /// [`XyzValue`]: ../struct.XyzValue.html
    pub fn from_xyz(red: XyzValue, green: XyzValue, blue: XyzValue) -> Self {
        Matrix3x3 {
            inner: [
                red.x, red.y, red.z,
                green.x, green.y, green.z,
                blue.x, blue.y, blue.z,
            ]
        }
    }

    /// Create a new [`Matrix3x3`](struct.Matrix3x3.html) from a list of floats
    pub const fn new(
        x0: f32, y0: f32, z0: f32,
        x1: f32, y1: f32, z1: f32,
        x2: f32, y2: f32, z2: f32
    ) -> Self {
        Matrix3x3 {
            inner: [
                x0, y0, z0,
                x1, y1, z1,
                x2, y2, z2,
            ]
        }
    }

    /// Return the red primary component as an `XyzValue`
    pub fn xyz_red(&self) -> ValueResult<XyzValue> {
        XyzValue::try_from(&[
            self.inner[0], self.inner[1], self.inner[2]
        ])
    }

    /// Return the green primary component as an `XyzValue`
    pub fn xyz_green(&self) -> ValueResult<XyzValue> {
        XyzValue::try_from(&[
            self.inner[3], self.inner[4], self.inner[5]
        ])
    }

    /// Return the blue primary component as an `XyzValue`
    pub fn xyz_blue(&self) -> ValueResult<XyzValue> {
        XyzValue::try_from(&[
            self.inner[6], self.inner[7], self.inner[8]
        ])
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

        &self.inner[idx.1 * 3 + idx.0]
    }
}

#[cfg(test)]
const TEST_MATRIX: Matrix3x3 = matrix3x3![
    0.0, 1.1, 2.2,
    3.3, 4.4, 5.5,
    6.6, 7.7, 8.8,
];

#[test]
fn matrix_index() {
    assert_eq!(TEST_MATRIX[0], 0.0);
    assert_eq!(TEST_MATRIX[4], 4.4);
    assert_eq!(TEST_MATRIX[8], 8.8);

    assert_eq!(TEST_MATRIX[(0,0)], 0.0);
    assert_eq!(TEST_MATRIX[(1,1)], 4.4);
    assert_eq!(TEST_MATRIX[(2,1)], 5.5);
    assert_eq!(TEST_MATRIX[(1,2)], 7.7);
    assert_eq!(TEST_MATRIX[(2,2)], 8.8);
}

macro_rules! index_panics {
    ($name:ident, $index:expr) => {
        #[test]
        #[should_panic]
        fn $name() {
            let _panic = TEST_MATRIX[$index];
        }
    }
}

index_panics!(index_panic_9, 9);
index_panics!(index_panic_3_0, (3,0));
index_panics!(index_panic_0_3, (0,3));
index_panics!(index_panic_3_3, (3,3));

impl Mul<RgbValue> for Matrix3x3 {
    type Output = XyzValue;
    fn mul(self, rhs: RgbValue) -> Self::Output {
        let nom = rhs.nominalize().compand_srgb_inv();

        let x = self[(0,0)] * nom.r
              + self[(1,0)] * nom.g
              + self[(2,0)] * nom.b;

        let y = self[(0,1)] * nom.r
              + self[(1,1)] * nom.g
              + self[(2,1)] * nom.b;


        let z = self[(0,2)] * nom.r
              + self[(1,2)] * nom.g
              + self[(2,2)] * nom.b;

        XyzValue { x, y, z, }.bradford()
    }
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

impl XyzValue {
    fn bradford(self) -> Self {
        let x = self.x * BRADFORD[(0,0)]
              + self.x * BRADFORD[(1,0)]
              + self.x * BRADFORD[(2,0)];

        let y = self.y * BRADFORD[(0,1)]
              + self.y * BRADFORD[(1,1)]
              + self.y * BRADFORD[(2,1)];

        let z = self.z * BRADFORD[(0,2)]
              + self.z * BRADFORD[(1,2)]
              + self.z * BRADFORD[(2,2)];

        XyzValue { x, y, z }
    }

    fn bradford_inv(self) -> Self {
        let x = self.x * BRADFORD_INV[(0,0)]
              + self.x * BRADFORD_INV[(1,0)]
              + self.x * BRADFORD_INV[(2,0)];

        let y = self.y * BRADFORD_INV[(0,1)]
              + self.y * BRADFORD_INV[(1,1)]
              + self.y * BRADFORD_INV[(2,1)];

        let z = self.z * BRADFORD_INV[(0,2)]
              + self.z * BRADFORD_INV[(1,2)]
              + self.z * BRADFORD_INV[(2,2)];

        XyzValue { x, y, z }
    }
}

const BRADFORD: Matrix3x3 = matrix3x3![
    0.8951000, 0.2664000, -0.1614000,
    -0.7502000, 1.7135000, 0.0367000,
    0.0389000,  -0.0685000, 1.0296000,
];

const BRADFORD_INV: Matrix3x3 = matrix3x3![
    0.9869929, -0.1470543, 0.1599627,
    0.4323053, 0.5183603, 0.0492912,
    -0.0085287, 0.0400428, 0.9684867,
];

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

impl Mul<XyzValue> for Matrix3x3 {
    type Output = RgbValue;
    fn mul(self, rhs: XyzValue) -> Self::Output {
        let rhs = rhs.bradford_inv();

        let r = self[(0,0)] * rhs.x
              + self[(1,0)] * rhs.y
              + self[(2,0)] * rhs.z;

        let g = self[(0,1)] * rhs.x
              + self[(1,1)] * rhs.y
              + self[(2,1)] * rhs.z;

        let b = self[(0,2)] * rhs.x
              + self[(1,2)] * rhs.y
              + self[(2,2)] * rhs.z;

        RgbNominalValue { r, g, b, }.compand_srgb().denominalize()
    }
}

// From Bruce Lindbloom
// http://www.brucelindbloom.com/Eqn_RGB_XYZ_Matrix.html
/// Matrix for converting AdobeRGB to XYZ with D65 Illuminant
pub const ADOBERGB_1998_D65_RGB2XYZ: Matrix3x3 = matrix3x3![
    0.5767309, 0.1855540, 0.1881852,
    0.2973769, 0.6273491, 0.0752741,
    0.0270343, 0.0706872, 0.9911085,
];
/// Matrix for converting XYZ to AdobeRGB with D65 Illuminant
pub const ADOBERGB_1998_D65_XYZ2RGB: Matrix3x3 = matrix3x3![
    2.0413690, -0.5649464, -0.3446944,
    -0.9692660, 1.8760108, 0.0415560,
    0.0134474, -0.1183897, 1.0154096,
];

/// Matrix for converting AppleRGB to XYZ with D65 Illuminant
pub const APPLERGB_D65_RGB2XYZ: Matrix3x3 = matrix3x3![
    0.4497288, 0.3162486, 0.1844926,
    0.2446525, 0.6720283, 0.0833192,
    0.0251848, 0.1411824, 0.9224628,
];
/// Matrix for converting XYZ to AppleRGB with D65 Illuminant
pub const APPLERGB_D65_XYZ2RGB: Matrix3x3 = matrix3x3![
    2.9515373, -1.2894116, -0.4738445,
    -1.0851093, 1.9908566, 0.0372026,
    0.0854934, -0.2694964, 1.0912975,
];

/// Matrix for converting BestRGB to XYZ with D50 Illuminant
pub const BESTRGB_D50_RGB2XYZ: Matrix3x3 = matrix3x3![
    0.6326696, 0.2045558, 0.1269946,
    0.2284569, 0.7373523, 0.0341908,
    0.0000000, 0.0095142, 0.8156958,
];
/// Matrix for converting XYZ to BestRGB with D50 Illuminant
pub const BESTRGB_D50_XYZ2RGB: Matrix3x3 = matrix3x3![
    1.7552599, -0.4836786, -0.2530000,
    -0.5441336, 1.5068789, 0.0215528,
    0.0063467, -0.0175761, 1.2256959,
];

/// Matrix for converting BetaRGB to XYZ with D50 Illuminant
pub const BETARGB_D50_RGB2XYZ: Matrix3x3 = matrix3x3![
    0.6712537, 0.1745834, 0.1183829,
    0.3032726, 0.6637861, 0.0329413,
    0.0000000, 0.0407010, 0.7845090,
];
/// Matrix for converting XYZ to BetaRGB with D50 Illuminant
pub const BETARGB_D50_XYZ2RGB: Matrix3x3 = matrix3x3![
    1.6832270, -0.4282363, -0.2360185,
    -0.7710229, 1.7065571, 0.0446900,
    0.0400013, -0.0885376, 1.2723640,
];

/// Matrix for converting BruceRGB to XYZ with D65 Illuminant
pub const BRUCERGB_D65_RGB2XYZ: Matrix3x3 = matrix3x3![
    0.4674162, 0.2944512, 0.1886026,
    0.2410115, 0.6835475, 0.0754410,
    0.0219101, 0.0736128, 0.9933071,
];
/// Matrix for converting XYZ to BruceRGB with D65 Illuminant
pub const BRUCERGB_D65_XYZ2RGB: Matrix3x3 = matrix3x3![
    2.7454669, -1.1358136, -0.4350269,
    -0.9692660, 1.8760108, 0.0415560,
    0.0112723, -0.1139754, 1.0132541,
];

/// Matrix for converting CIERGB to XYZ with E Illuminant
pub const CIERGB_E_RGB2XYZ: Matrix3x3 = matrix3x3![
    0.4887180, 0.3106803, 0.2006017,
    0.1762044, 0.8129847, 0.0108109,
    0.0000000, 0.0102048, 0.9897952,
];
/// Matrix for converting XYZ to CIERGB with E Illuminant
pub const CIERGB_E_XYZ2RGB: Matrix3x3 = matrix3x3![
    2.3706743, -0.9000405, -0.4706338,
    -0.5138850, 1.4253036, 0.0885814,
    0.0052982, -0.0146949, 1.0093968,
];

/// Matrix for converting ColorMatchRGB to XYZ with D50 Illuminant
pub const COLORMATCHRGB_D50_RGB2XYZ: Matrix3x3 = matrix3x3![
    0.5093439, 0.3209071, 0.1339691,
    0.2748840, 0.6581315, 0.0669845,
    0.0242545, 0.1087821, 0.6921735,
];
/// Matrix for converting XYZ to ColorMatchRGB with D50 Illuminant
pub const COLORMATCHRGB_D50_XYZ2RGB: Matrix3x3 = matrix3x3![
    2.6422874, -1.2234270, -0.3930143,
    -1.1119763, 2.0590183, 0.0159614,
    0.0821699, -0.2807254, 1.4559877,
];

/// Matrix for converting DonRGB4 to XYZ with D50 Illuminant
pub const DONRGB4_D50_RGB2XYZ: Matrix3x3 = matrix3x3![
    0.6457711, 0.1933511, 0.1250978,
    0.2783496, 0.6879702, 0.0336802,
    0.0037113, 0.0179861, 0.8035125,
];
/// Matrix for converting XYZ to DonRGB4 with D50 Illuminant
pub const DONRGB4_D50_XYZ2RGB: Matrix3x3 = matrix3x3![
    1.7603902, -0.4881198, -0.2536126,
    -0.7126288, 1.6527432, 0.0416715,
    0.0078207, -0.0347411, 1.2447743,
];

/// Matrix for converting ECIRGB to XYZ with D50 Illuminant
pub const ECIRGB_D50_RGB2XYZ: Matrix3x3 = matrix3x3![
    0.6502043, 0.1780774, 0.1359384,
    0.3202499, 0.6020711, 0.0776791,
    0.0000000, 0.0678390, 0.7573710,
];
/// Matrix for converting XYZ to ECIRGB with D50 Illuminant
pub const ECIRGB_D50_XYZ2RGB: Matrix3x3 = matrix3x3![
    1.7827618, -0.4969847, -0.2690101,
    -0.9593623, 1.9477962, -0.0275807,
    0.0859317, -0.1744674, 1.3228273,
];

/// Matrix for converting EktaSpace to XYZ with D50 Illuminant
pub const EKTASPACE_PS5_D50_RGB2XYZ: Matrix3x3 = matrix3x3![
    0.5938914, 0.2729801, 0.0973485,
    0.2606286, 0.7349465, 0.0044249,
    0.0000000, 0.0419969, 0.7832131,
];
/// Matrix for converting XYZ to EktaSpace with D50 Illuminant
pub const EKTASPACE_PS5_D50_XYZ2RGB: Matrix3x3 = matrix3x3![
    2.0043819, -0.7304844, -0.2450052,
    -0.7110285, 1.6202126, 0.0792227,
    0.0381263, -0.0868780, 1.2725438,
];

/// Matrix for converting NTSCRGB to XYZ with C Illuminant
pub const NTSCRGB_C_RGB2XYZ: Matrix3x3 = matrix3x3![
    0.6068909, 0.1735011, 0.2003480,
    0.2989164, 0.5865990, 0.1144845,
    0.0000000, 0.0660957, 1.1162243,
];
/// Matrix for converting XYZ to NTSCRGB with C Illuminant
pub const NTSCRGB_C_XYZ2RGB: Matrix3x3 = matrix3x3![
    1.9099961, -0.5324542, -0.2882091,
    -0.9846663, 1.9991710, -0.0283082,
    0.0583056, -0.1183781, 0.8975535,
];

/// Matrix for converting PAL/SECAM RGB to XYZ with D65 Illuminant
pub const PALSECAMRGB_D65_RGB2XYZ: Matrix3x3 = matrix3x3![
    0.4306190, 0.3415419, 0.1783091,
    0.2220379, 0.7066384, 0.0713236,
    0.0201853, 0.1295504, 0.9390944,
];
/// Matrix for converting XYZ to PAL/SECAM RGB with D65 Illuminant
pub const PALSECAMRGB_D65_XYZ2RGB: Matrix3x3 = matrix3x3![
    3.0628971, -1.3931791, -0.4757517,
    -0.9692660, 1.8760108, 0.0415560,
    0.0678775, -0.2288548, 1.0693490,
];

/// Matrix for converting ProPhotoRGB to XYZ with D50 Illuminant
pub const PROPHOTORGB_D50_RGB2XYZ: Matrix3x3 = matrix3x3![
    0.7976749, 0.1351917, 0.0313534,
    0.2880402, 0.7118741, 0.0000857,
    0.0000000, 0.0000000, 0.8252100,
];
/// Matrix for converting XYZ to ProPhotoRGB with D50 Illuminant
pub const PROPHOTORGB_D50_XYZ2RGB: Matrix3x3 = matrix3x3![
    1.3459433, -0.2556075, -0.0511118,
    -0.5445989, 1.5081673, 0.0205351,
    0.0000000, 0.0000000, 1.2118128,
];

/// Matrix for converting SMPTE RGB to XYZ with D65 Illuminant
pub const SMPTERGB_D65_RGB2XYZ: Matrix3x3 = matrix3x3![
    0.3935891, 0.3652497, 0.1916313,
    0.2124132, 0.7010437, 0.0865432,
    0.0187423, 0.1119313, 0.9581563,
];
/// Matrix for converting XYZ to SMPTE RGB with D65 Illuminant
pub const SMPTERGB_D65_XYZ2RGB: Matrix3x3 = matrix3x3![
    3.5053960, -1.7394894, -0.5439640,
    -1.0690722, 1.9778245, 0.0351722,
    0.0563200, -0.1970226, 1.0502026,
];

/// Matrix for converting sRGB to XYZ with D65 Illuminant
pub const SRGB_D65_RGB2XYZ: Matrix3x3 = matrix3x3![
    0.4124564, 0.3575761, 0.1804375,
    0.2126729, 0.7151522, 0.0721750,
    0.0193339, 0.1191920, 0.9503041,
];
/// Matrix for converting XYZ to sRGB with D65 Illuminant
pub const SRGB_D65_XYZ2RGB: Matrix3x3 = matrix3x3![
    3.2404542, -1.5371385, -0.4985314,
    -0.9692660, 1.8760108, 0.0415560,
    0.0556434, -0.2040259, 1.0572252,
];

/// Matrix for converting WideGamutRGB to XYZ with D65 Illuminant
pub const WIDEGAMUTRGB_D50_RGB2XYZ: Matrix3x3 = matrix3x3![
    0.7161046, 0.1009296, 0.1471858,
    0.2581874, 0.7249378, 0.0168748,
    0.0000000, 0.0517813, 0.7734287,
];
/// Matrix for converting XYZ to WideGamutRGB with D65 Illuminant
pub const WIDEGAMUTRGB_D50_XYZ2RGB: Matrix3x3 = matrix3x3![
    1.4628067, -0.1840623, -0.2743606,
    -0.5217933, 1.4472381, 0.0677227,
    0.0349342, -0.0968930, 1.2884099,
];
