use crate::*;
use illuminant::Illuminant;
use std::iter::FromIterator;

/// A 3x3 matrix of 9 floats
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct Matrix3x3 {
    inner: [f32; 9],
}

impl Matrix3x3 {
    /// Create a new `Matrix3x3` from 3 `XyzValue`s
    pub fn new(xyz_0: XyzValue, xyz_1: XyzValue, xyz_2: XyzValue) -> Self {
        Matrix3x3 {
            inner: [
                xyz_0.x, xyz_0.y, xyz_0.z,
                xyz_1.x, xyz_1.y, xyz_1.z,
                xyz_2.x, xyz_2.y, xyz_2.z,
            ]
        }
    }

    /// Return the red primary component as an `XyzValue`
    pub fn red(&self) -> ValueResult<XyzValue> {
        XyzValue::try_from(&[
            self.inner[0], self.inner[1], self.inner[2]
        ])
    }

    /// Return the green primary component as an `XyzValue`
    pub fn green(&self) -> ValueResult<XyzValue> {
        XyzValue::try_from(&[
            self.inner[3], self.inner[4], self.inner[5]
        ])
    }

    /// Return the blue primary component as an `XyzValue`
    pub fn blue(&self) -> ValueResult<XyzValue> {
        XyzValue::try_from(&[
            self.inner[6], self.inner[7], self.inner[8]
        ])
    }
}

/// Create a new `Matrix3x3` from a list of floats
///
/// # Example
/// ```
/// use deltae::*;
///
/// let matrix = matrix3x3![
///     0.5767309, 0.1855540, 0.1881852,
///     0.2973769, 0.6273491, 0.0752741,
///     0.0270343, 0.0706872, 0.9911085,
/// ];
/// ```
#[macro_use]
macro_rules! matrix3x3 {
    [
        $x0:expr, $y0:expr, $z0:expr,
        $x1:expr, $y1:expr, $z1:expr,
        $x2:expr, $y2:expr, $z2:expr
    ] => {
        Matrix3x3 {
            inner: [
                $x0, $y0, $z0,
                $x1, $y1, $z1,
                $x2, $y2, $z2,
            ]
        }
    };
    [
        $x0:expr, $y0:expr, $z0:expr,
        $x1:expr, $y1:expr, $z1:expr,
        $x2:expr, $y2:expr, $z2:expr,
    ] => {
        Matrix3x3 {
            inner: [
                $x0, $y0, $z0,
                $x1, $y1, $z1,
                $x2, $y2, $z2,
            ]
        }
    }
}

impl<'a> FromIterator<&'a f32> for Matrix3x3 {
    fn from_iter<I: IntoIterator<Item=&'a f32>>(iter: I) -> Self {
        let vec = iter.into_iter().take(9).collect::<Vec<_>>();

        let mut inner = [1_f32; 9];
        for i in 0..9 {
            if let Some(val) = vec.get(i) {
                inner[i] = **val;
            }
        }
            
        Matrix3x3 { inner }
    }
}

#[test]
fn matrix_from_iter() {
    let arr = [1.0_f32, 2.0, 3.0, 4.0, 5.0];
    let matrix = arr.iter().collect::<Matrix3x3>();
    let exp = matrix3x3![
        1.0, 2.0, 3.0, 4.0, 5.0, 1.0, 1.0, 1.0, 1.0
    ];
    assert_eq!(matrix, exp);
}

fn xyz_to_rgb(xyz: XyzValue, illum: Illuminant) -> RgbValue {
    todo!()
}

/// A nominalized RGB value on a scale from 0 to 1
#[derive(Debug, Clone, PartialEq)]
pub struct RgbNominalValue {
    r: f32,
    g: f32,
    b: f32,
}

#[test]
fn nominalize_rgb() {
    let rgb = RgbValue::new(64, 128, 255);
    let nom = rgb.nominalize();
    let exp = RgbNominalValue {
        r: 0.2509804,
        g: 0.5019608,
        b: 1.0,
    };
    assert_eq!(nom, exp);
}

/// A trait to nominalize values on a scale from 0 to 1
pub trait Nominalize {
    /// The type to convert to when nominalizing
    type NominalType;
    /// Defines how to nominalize a type
    fn nominalize(&self) -> Self::NominalType;
}

impl Nominalize for RgbValue {
    type NominalType = RgbNominalValue;
    fn nominalize(&self) -> Self::NominalType {
        RgbNominalValue {
            r: self.r as f32 / 255.0,
            g: self.g as f32 / 255.0,
            b: self.b as f32 / 255.0,
        }
    }
}

// From Bruce Lindbloom
// http://www.brucelindbloom.com/Eqn_RGB_XYZ_Matrix.html

pub const ADOBE_RGB_1998_D65_RGB2XYZ: Matrix3x3 = matrix3x3! [
    0.5767309, 0.1855540, 0.1881852,
    0.2973769, 0.6273491, 0.0752741,
    0.0270343, 0.0706872, 0.9911085
];
pub const ADOBE_RGB_1998_D65_XYZ2RGB: Matrix3x3 = matrix3x3! [
    2.0413690, -0.5649464, -0.3446944,
    -0.9692660, 1.8760108, 0.0415560,
    0.0134474, -0.1183897, 1.0154096
];

pub const APPLERGB_D65_RGB2XYZ: Matrix3x3 = matrix3x3! [
    0.4497288, 0.3162486, 0.1844926,
    0.2446525, 0.6720283, 0.0833192,
    0.0251848, 0.1411824, 0.9224628
];
pub const APPLERGB_D65_XYZ2RGB: Matrix3x3 = matrix3x3! [
    2.9515373, -1.2894116, -0.4738445,
    -1.0851093, 1.9908566, 0.0372026,
    0.0854934, -0.2694964, 1.0912975
];

pub const BEST_RGB_D50_RGB2XYZ: Matrix3x3 = matrix3x3! [
    0.6326696, 0.2045558, 0.1269946,
    0.2284569, 0.7373523, 0.0341908,
    0.0000000, 0.0095142, 0.8156958
];
pub const BEST_RGB_D50_XYZ2RGB: Matrix3x3 = matrix3x3! [
    1.7552599, -0.4836786, -0.2530000,
    -0.5441336, 1.5068789, 0.0215528,
    0.0063467, -0.0175761, 1.2256959
];

pub const BETA_RGB_D50_RGB2XYZ: Matrix3x3 = matrix3x3! [
    0.6712537, 0.1745834, 0.1183829,
    0.3032726, 0.6637861, 0.0329413,
    0.0000000, 0.0407010, 0.7845090
];
pub const BETA_RGB_D50_XYZ2RGB: Matrix3x3 = matrix3x3! [
    1.6832270, -0.4282363, -0.2360185,
    -0.7710229, 1.7065571, 0.0446900,
    0.0400013, -0.0885376, 1.2723640
];

pub const BRUCE_RGB_D65_RGB2XYZ: Matrix3x3 = matrix3x3! [
    0.4674162, 0.2944512, 0.1886026,
    0.2410115, 0.6835475, 0.0754410,
    0.0219101, 0.0736128, 0.9933071
];
pub const BRUCE_RGB_D65_XYZ2RGB: Matrix3x3 = matrix3x3! [
    2.7454669, -1.1358136, -0.4350269,
    -0.9692660, 1.8760108, 0.0415560,
    0.0112723, -0.1139754, 1.0132541
];

pub const CIE_RGB_E_RGB2XYZ: Matrix3x3 = matrix3x3! [
    0.4887180, 0.3106803, 0.2006017,
    0.1762044, 0.8129847, 0.0108109,
    0.0000000, 0.0102048, 0.9897952
];
pub const CIE_RGB_E_XYZ2RGB: Matrix3x3 = matrix3x3! [
    2.3706743, -0.9000405, -0.4706338,
    -0.5138850, 1.4253036, 0.0885814,
    0.0052982, -0.0146949, 1.0093968
];

pub const COLORMATCH_RGB_D50_RGB2XYZ: Matrix3x3 = matrix3x3! [
    0.5093439, 0.3209071, 0.1339691,
    0.2748840, 0.6581315, 0.0669845,
    0.0242545, 0.1087821, 0.6921735
];
pub const COLORMATCH_RGB_D50_XYZ2RGB: Matrix3x3 = matrix3x3! [
    2.6422874, -1.2234270, -0.3930143,
    -1.1119763, 2.0590183, 0.0159614,
    0.0821699, -0.2807254, 1.4559877
];

pub const DON_RGB_4_D50_RGB2XYZ: Matrix3x3 = matrix3x3! [
    0.6457711, 0.1933511, 0.1250978,
    0.2783496, 0.6879702, 0.0336802,
    0.0037113, 0.0179861, 0.8035125
];
pub const DON_RGB_4_D50_XYZ2RGB: Matrix3x3 = matrix3x3! [
    1.7603902, -0.4881198, -0.2536126,
    -0.7126288, 1.6527432, 0.0416715,
    0.0078207, -0.0347411, 1.2447743
];

pub const ECI_RGB_D50_RGB2XYZ: Matrix3x3 = matrix3x3! [
    0.6502043, 0.1780774, 0.1359384,
    0.3202499, 0.6020711, 0.0776791,
    0.0000000, 0.0678390, 0.7573710
];
pub const ECI_RGB_D50_XYZ2RGB: Matrix3x3 = matrix3x3! [
    1.7827618, -0.4969847, -0.2690101,
    -0.9593623, 1.9477962, -0.0275807,
    0.0859317, -0.1744674, 1.3228273
];

pub const EKTA_SPACE_PS5_D50_RGB2XYZ: Matrix3x3 = matrix3x3! [
    0.5938914, 0.2729801, 0.0973485,
    0.2606286, 0.7349465, 0.0044249,
    0.0000000, 0.0419969, 0.7832131
];
pub const EKTA_SPACE_PS5_D50_XYZ2RGB: Matrix3x3 = matrix3x3! [
    2.0043819, -0.7304844, -0.2450052,
    -0.7110285, 1.6202126, 0.0792227,
    0.0381263, -0.0868780, 1.2725438
];

pub const NTSC_RGB_C_RGB2XYZ: Matrix3x3 = matrix3x3! [
    0.6068909, 0.1735011, 0.2003480,
    0.2989164, 0.5865990, 0.1144845,
    0.0000000, 0.0660957, 1.1162243
];
pub const NTSC_RGB_C_XYZ2RGB: Matrix3x3 = matrix3x3! [
    1.9099961, -0.5324542, -0.2882091,
    -0.9846663, 1.9991710, -0.0283082,
    0.0583056, -0.1183781, 0.8975535
];

pub const PAL_SECAM_RGB_D65_RGB2XYZ: Matrix3x3 = matrix3x3! [
    0.4306190, 0.3415419, 0.1783091,
    0.2220379, 0.7066384, 0.0713236,
    0.0201853, 0.1295504, 0.9390944
];
pub const PAL_SECAM_RGB_D65_XYZ2RGB: Matrix3x3 = matrix3x3! [
    3.0628971, -1.3931791, -0.4757517,
    -0.9692660, 1.8760108, 0.0415560,
    0.0678775, -0.2288548, 1.0693490
];

pub const PROPHOTO_RGB_D50_RGB2XYZ: Matrix3x3 = matrix3x3! [
    0.7976749, 0.1351917, 0.0313534,
    0.2880402, 0.7118741, 0.0000857,
    0.0000000, 0.0000000, 0.8252100
];
pub const PROPHOTO_RGB_D50_XYZ2RGB: Matrix3x3 = matrix3x3! [
    1.3459433, -0.2556075, -0.0511118,
    -0.5445989, 1.5081673, 0.0205351,
    0.0000000, 0.0000000, 1.2118128
];

pub const SMPTE_C_RGB_D65_RGB2XYZ: Matrix3x3 = matrix3x3! [
    0.3935891, 0.3652497, 0.1916313,
    0.2124132, 0.7010437, 0.0865432,
    0.0187423, 0.1119313, 0.9581563
];
pub const SMPTE_C_RGB_D65_XYZ2RGB: Matrix3x3 = matrix3x3! [
    3.5053960, -1.7394894, -0.5439640,
    -1.0690722, 1.9778245, 0.0351722,
    0.0563200, -0.1970226, 1.0502026
];

pub const SRGB_D65_RGB2XYZ: Matrix3x3 = matrix3x3! [
    0.4124564, 0.3575761, 0.1804375,
    0.2126729, 0.7151522, 0.0721750,
    0.0193339, 0.1191920, 0.9503041
];
pub const SRGB_D65_XYZ2RGB: Matrix3x3 = matrix3x3! [
    3.2404542, -1.5371385, -0.4985314,
    -0.9692660, 1.8760108, 0.0415560,
    0.0556434, -0.2040259, 1.0572252
];

pub const WIDE_GAMUT_RGB_D50_RGB2XYZ: Matrix3x3 = matrix3x3! [
    0.7161046, 0.1009296, 0.1471858,
    0.2581874, 0.7249378, 0.0168748,
    0.0000000, 0.0517813, 0.7734287
];
pub const WIDE_GAMUT_RGB_D50_XYZ2RGB: Matrix3x3 = matrix3x3! [
    1.4628067, -0.1840623, -0.2743606,
    -0.5217933, 1.4472381, 0.0677227,
    0.0349342, -0.0968930, 1.2884099
];
