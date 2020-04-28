use crate::*;
use illuminant::D50;
pub use std::convert::TryFrom;

// To Lab /////////////////////////////////////////////////////////////////////
impl From<LchValue> for LabValue {
    fn from(lch: LchValue) -> LabValue {
        LabValue {
            l: lch.l,
            a: lch.c * lch.h.to_radians().cos(),
            b: lch.c * lch.h.to_radians().sin(),
        }
    }
}

impl From<&LchValue> for LabValue {
    fn from(lch: &LchValue) -> LabValue {
        LabValue::from(*lch)
    }
}

impl From<&LabValue> for LabValue {
    fn from(lab: &LabValue) -> LabValue {
        *lab
    }
}

impl From<XyzValue> for LabValue {
    fn from(xyz: XyzValue) -> LabValue {
        let x = xyz_to_lab_map(xyz.x / D50.x);
        let y = xyz_to_lab_map(xyz.y / D50.y);
        let z = xyz_to_lab_map(xyz.z / D50.z);

        LabValue {
            l: (116.0 * y) - 16.0,
            a: 500.0 * (x - y),
            b: 200.0 * (y - z),
        }
    }
}

impl From<&XyzValue> for LabValue {
    fn from(xyz: &XyzValue) -> LabValue {
        LabValue::from(*xyz)
    }
}

impl TryFrom<&[f32; 3]> for LabValue {
    type Error = ValueError;
    fn try_from(slice: &[f32; 3]) -> ValueResult<LabValue> {
        LabValue {
            l: slice[0],
            a: slice[1],
            b: slice[2]
        }.validate()
    }
}

impl TryFrom<(f32, f32, f32)> for LabValue {
    type Error = ValueError;
    fn try_from(tuple: (f32, f32, f32)) -> ValueResult<LabValue> {
        LabValue {
            l: tuple.0,
            a: tuple.1,
            b: tuple.2,
        }.validate()
    }
}

impl TryFrom<&(f32, f32, f32)> for LabValue {
    type Error = ValueError;
    fn try_from(tuple: &(f32, f32, f32)) -> ValueResult<LabValue> {
        LabValue {
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

impl From<LabValue> for LchValue {
    fn from(lab: LabValue) -> LchValue {
        LchValue {
            l: lab.l,
            c: ( lab.a.powi(2) + lab.b.powi(2) ).sqrt(),
            h: get_h_prime(lab.a, lab.b),
        }
    }
}

impl From<&LabValue> for LchValue {
    fn from(lab: &LabValue) -> LchValue {
        LchValue::from(*lab)
    }
}

impl From<XyzValue> for LchValue {
    fn from(xyz: XyzValue) -> LchValue {
        LchValue::from(LabValue::from(xyz))
    }
}

impl From<&XyzValue> for LchValue {
    fn from(xyz: &XyzValue) -> LchValue {
        LchValue::from(*xyz)
    }
}

impl TryFrom<&[f32; 3]> for LchValue {
    type Error = ValueError;
    fn try_from(slice: &[f32; 3]) -> ValueResult<LchValue> {
        LchValue {
            l: slice[0],
            c: slice[1],
            h: slice[2]
        }.validate()
    }
}

impl TryFrom<(f32, f32, f32)> for LchValue {
    type Error = ValueError;
    fn try_from(tuple: (f32, f32, f32)) -> ValueResult<LchValue> {
        LchValue {
            l: tuple.0,
            c: tuple.1,
            h: tuple.2,
        }.validate()
    }
}

impl TryFrom<&(f32, f32, f32)> for LchValue {
    type Error = ValueError;
    fn try_from(tuple: &(f32, f32, f32)) -> ValueResult<LchValue> {
        LchValue {
            l: tuple.0,
            c: tuple.1,
            h: tuple.2,
        }.validate()
    }
}

// To Xyz /////////////////////////////////////////////////////////////////////
impl From<LabValue> for XyzValue {
    fn from(lab: LabValue) -> XyzValue {
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

        XyzValue {
            x: xr * D50.x,
            y: yr * D50.y,
            z: zr * D50.z,
        }
    }
}

impl From<&XyzValue> for XyzValue {
    fn from(xyz: &XyzValue) -> XyzValue {
        XyzValue::from(*xyz)
    }
}

impl From<&LabValue> for XyzValue {
    fn from(lab: &LabValue) -> XyzValue {
        XyzValue::from(*lab)
    }
}

impl From<LchValue> for XyzValue {
    fn from(lch: LchValue) -> XyzValue {
        XyzValue::from(LabValue::from(lch))
    }
}

impl From<&LchValue> for XyzValue {
    fn from(lch: &LchValue) -> XyzValue {
        XyzValue::from(*lch)
    }
}

impl TryFrom<&[f32; 3]> for XyzValue {
    type Error = ValueError;
    fn try_from(slice: &[f32; 3]) -> ValueResult<XyzValue> {
        XyzValue {
            x: slice[0],
            y: slice[1],
            z: slice[2]
        }.validate()
    }
}

impl TryFrom<(f32, f32, f32)> for XyzValue {
    type Error = ValueError;
    fn try_from(tuple: (f32, f32, f32)) -> ValueResult<XyzValue> {
        XyzValue {
            x: tuple.0,
            y: tuple.1,
            z: tuple.2,
        }.validate()
    }
}

impl TryFrom<&(f32, f32, f32)> for XyzValue {
    type Error = ValueError;
    fn try_from(tuple: &(f32, f32, f32)) -> ValueResult<XyzValue> {
        XyzValue {
            x: tuple.0,
            y: tuple.1,
            z: tuple.2,
        }.validate()
    }
}

// To RGB //////////////////////////////////////////////////////////////////////
impl From<LabValue> for RgbValue {
    fn from(lab: LabValue) -> Self {
        todo!()
    }
}

impl From<&LabValue> for RgbValue {
    fn from(lab: &LabValue) -> Self {
        todo!()
    }
}

// Helper Functions ////////////////////////////////////////////////////////////
const KAPPA: f32 = 24389.0 / 27.0; // CIE Standard: 903.3
const EPSILON: f32 = 216.0 / 24389.0; // CIE Standard: 0.008856
const CBRT_EPSILON: f32 = 0.20689655172413796;

pub(crate) fn get_h_prime(a: f32, b: f32) -> f32 {
    let h_prime = b.atan2(a).to_degrees();
    if h_prime < 0.0 {
        h_prime + 360.0
    } else {
        h_prime
    }
}

#[inline]
fn xyz_to_lab_map(c: f32) -> f32 {
    if c > EPSILON {
        c.powf(1.0/3.0)
    } else {
        (KAPPA * c + 16.0) / 116.0
    }
}
