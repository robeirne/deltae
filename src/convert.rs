use crate::*;
use illuminant::*;
pub use std::convert::TryFrom;

// To Lab /////////////////////////////////////////////////////////////////////
impl From<LchValue> for CieLabValue {
    fn from(lch: LchValue) -> CieLabValue {
        CieLabValue {
            l: lch.l,
            a: lch.c * lch.h.to_radians().cos(),
            b: lch.c * lch.h.to_radians().sin(),
        }
    }
}

impl From<&LchValue> for CieLabValue {
    fn from(lch: &LchValue) -> CieLabValue {
        CieLabValue::from(*lch)
    }
}

impl From<&CieLabValue> for CieLabValue {
    fn from(lab: &CieLabValue) -> CieLabValue {
        *lab
    }
}

impl From<CieXyzValue> for CieLabValue {
    fn from(xyz: CieXyzValue) -> CieLabValue {
        let x = xyz_to_lab_map(xyz.x / D50[0]);
        let y = xyz_to_lab_map(xyz.y / D50[1]);
        let z = xyz_to_lab_map(xyz.z / D50[2]);

        CieLabValue {
            l: (116.0 * y) - 16.0,
            a: 500.0 * (x - y),
            b: 200.0 * (y - z),
        }
    }
}

impl From<&CieXyzValue> for CieLabValue {
    fn from(xyz: &CieXyzValue) -> CieLabValue {
        CieLabValue::from(*xyz)
    }
}

impl From<RgbValue> for CieLabValue {
    fn from(rgb: RgbValue) -> CieLabValue {
        CieLabValue::from(CieXyzValue::from(rgb))
    }
}

impl TryFrom<&[f64; 3]> for CieLabValue {
    type Error = ValueError;
    fn try_from(slice: &[f64; 3]) -> ValueResult<CieLabValue> {
        CieLabValue {
            l: slice[0],
            a: slice[1],
            b: slice[2]
        }.validate()
    }
}

impl TryFrom<(f64, f64, f64)> for CieLabValue {
    type Error = ValueError;
    fn try_from(tuple: (f64, f64, f64)) -> ValueResult<CieLabValue> {
        CieLabValue {
            l: tuple.0,
            a: tuple.1,
            b: tuple.2,
        }.validate()
    }
}

impl TryFrom<&(f64, f64, f64)> for CieLabValue {
    type Error = ValueError;
    fn try_from(tuple: &(f64, f64, f64)) -> ValueResult<CieLabValue> {
        CieLabValue {
            l: tuple.0,
            a: tuple.1,
            b: tuple.2,
        }.validate()
    }
}

// To Lch /////////////////////////////////////////////////////////////////////
impl From<&LchValue> for LchValue {
    fn from(lch: &LchValue) -> LchValue {
        LchValue::from(*lch)
    }
}

impl From<CieLabValue> for LchValue {
    fn from(lab: CieLabValue) -> LchValue {
        LchValue {
            l: lab.l,
            c: ( lab.a.powi(2) + lab.b.powi(2) ).sqrt(),
            h: get_h_prime(lab.a, lab.b),
        }
    }
}

impl From<&CieLabValue> for LchValue {
    fn from(lab: &CieLabValue) -> LchValue {
        LchValue::from(*lab)
    }
}

impl From<CieXyzValue> for LchValue {
    fn from(xyz: CieXyzValue) -> LchValue {
        LchValue::from(CieLabValue::from(xyz))
    }
}

impl From<&CieXyzValue> for LchValue {
    fn from(xyz: &CieXyzValue) -> LchValue {
        LchValue::from(*xyz)
    }
}

impl TryFrom<&[f64; 3]> for LchValue {
    type Error = ValueError;
    fn try_from(slice: &[f64; 3]) -> ValueResult<LchValue> {
        LchValue {
            l: slice[0],
            c: slice[1],
            h: slice[2]
        }.validate()
    }
}

impl TryFrom<(f64, f64, f64)> for LchValue {
    type Error = ValueError;
    fn try_from(tuple: (f64, f64, f64)) -> ValueResult<LchValue> {
        LchValue {
            l: tuple.0,
            c: tuple.1,
            h: tuple.2,
        }.validate()
    }
}

impl TryFrom<&(f64, f64, f64)> for LchValue {
    type Error = ValueError;
    fn try_from(tuple: &(f64, f64, f64)) -> ValueResult<LchValue> {
        LchValue {
            l: tuple.0,
            c: tuple.1,
            h: tuple.2,
        }.validate()
    }
}

// To Xyz /////////////////////////////////////////////////////////////////////
impl From<CieLabValue> for XyzRefValue {
    fn from(lab: CieLabValue) -> XyzRefValue {
        let fy = (lab.l + 16.0) / 116.0;
        let fx = (lab.a / 500.0) + fy;
        let fz = fy - (lab.b / 200.0);
        let xr = if fx > CBRT_EPSILON {
            fx.powi(3)
        } else {
            ((fx * 116.0) - 16.0) / KAPPA
        };
        let yr = if lab.l > EPSILON * KAPPA {
            fy.powi(3)
        } else {
            lab.l / KAPPA
        };
        let zr = if fz > CBRT_EPSILON {
            fz.powi(3)
        } else {
            ((fz * 116.0) - 16.0) / KAPPA
        };

        let white = Illuminant::D50;
        let white_xyz = white.xyz();

        let xyz = CieXyzValue {
            x: xr * white_xyz.x,
            y: yr * white_xyz.y,
            z: zr * white_xyz.z,
        };

        XyzRefValue::new(xyz, white)
    }
}

impl From<CieLabValue> for CieXyzValue {
    fn from(lab: CieLabValue) -> Self {
        XyzRefValue::from(lab).xyz().clone()
    }
}

impl From<&CieLabValue> for CieXyzValue {
    fn from(lab: &CieLabValue) -> CieXyzValue {
        CieXyzValue::from(*lab)
    }
}

impl From<&CieXyzValue> for CieXyzValue {
    fn from(xyz: &CieXyzValue) -> CieXyzValue {
        CieXyzValue::from(*xyz)
    }
}

impl From<LchValue> for CieXyzValue {
    fn from(lch: LchValue) -> CieXyzValue {
        CieXyzValue::from(CieLabValue::from(lch))
    }
}

impl From<&LchValue> for CieXyzValue {
    fn from(lch: &LchValue) -> CieXyzValue {
        CieXyzValue::from(*lch)
    }
}

impl From<RgbValue> for CieXyzValue {
    fn from(rgb: RgbValue) -> CieXyzValue {
        rgb.to_xyz(rgb::RgbSystem::default())
    }
}

impl From<&RgbValue> for CieXyzValue {
    fn from(rgb: &RgbValue) -> CieXyzValue {
        CieXyzValue::from(*rgb)
    }
}

impl TryFrom<&[f64; 3]> for CieXyzValue {
    type Error = ValueError;
    fn try_from(slice: &[f64; 3]) -> ValueResult<CieXyzValue> {
        CieXyzValue {
            x: slice[0],
            y: slice[1],
            z: slice[2]
        }.validate()
    }
}

impl TryFrom<(f64, f64, f64)> for CieXyzValue {
    type Error = ValueError;
    fn try_from(tuple: (f64, f64, f64)) -> ValueResult<CieXyzValue> {
        CieXyzValue {
            x: tuple.0,
            y: tuple.1,
            z: tuple.2,
        }.validate()
    }
}

impl TryFrom<&(f64, f64, f64)> for CieXyzValue {
    type Error = ValueError;
    fn try_from(tuple: &(f64, f64, f64)) -> ValueResult<CieXyzValue> {
        CieXyzValue {
            x: tuple.0,
            y: tuple.1,
            z: tuple.2,
        }.validate()
    }
}

// To RGB //////////////////////////////////////////////////////////////////////
impl From<CieLabValue> for RgbValue {
    fn from(lab: CieLabValue) -> Self {
        CieXyzValue::from(lab).to_rgb(rgb::RgbSystem::default())
    }
}

impl From<&CieLabValue> for RgbValue {
    fn from(lab: &CieLabValue) -> Self {
        RgbValue::from(*lab)
    }
}

// Helper Functions ////////////////////////////////////////////////////////////
const KAPPA: f64 = 24389.0 / 27.0; // CIE Standard: 903.3
const EPSILON: f64 = 216.0 / 24389.0; // CIE Standard: 0.008856
const CBRT_EPSILON: f64 = 0.20689655172413796;

pub(crate) fn get_h_prime(a: f64, b: f64) -> f64 {
    let h_prime = b.atan2(a).to_degrees();
    if h_prime < 0.0 {
        h_prime + 360.0
    } else {
        h_prime
    }
}

#[inline]
fn xyz_to_lab_map(c: f64) -> f64 {
    if c > EPSILON {
        c.powf(1.0/3.0)
    } else {
        (KAPPA * c + 16.0) / 116.0
    }
}
