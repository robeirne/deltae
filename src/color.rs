//! Manipulate and convert CIE L\*a\*b\* and Lch colors.
//!
//! # Examples
//!
//! ```
//! use deltae::*;
//! use std::error::Error;
//!
//! fn main() -> Result<(), Box<dyn Error>> {
//!     let lab0: LabValue = "95.08, -0.17, -10.81".parse()?;
//!     let lch0 = LchValue {
//!         l: 95.08,
//!         c: 10.811337,
//!         h: 269.09903,
//!     };
//!
//!     assert!(lab0.delta_eq(lch0, &Tolerance::default()));
//!
//!     let lch0 = LchValue::from(lab0);
//!     let lab2 = LabValue::from(lch0);
//!
//!     println!("{}", lch0); // [L:89.73, c:7.2094, h:285.1157]
//!
//!     assert_eq!(lab0.round_to(4), lab2.round_to(4));
//!
//!     Ok(())
//! }
//! ```

use std::fmt;
use std::error::Error;

use crate::{
    ValueResult,
    rgb,
    validate::Validate,
};

/// # CIEL\*a\*b\*
///
/// | `Value` | `Color`               | `Range`          |
/// |:-------:|:---------------------:|:----------------:|
/// | `L*`    | `Light <---> Dark`    | `0 <---> 100`    |
/// | `a*`    | `Green <---> Magenta` | `-128 <---> 128` |
/// | `b*`    | `Blue  <---> Yellow`  | `-128 <---> 128` |
///
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct LabValue {
    /// Lightness
    pub l: f32,
    /// Green - Magenta
    pub a: f32,
    /// Blue - Yellow
    pub b: f32,
}

impl LabValue {
    /// Returns a result of a LabValue from 3 `f32`s.
    /// Will return `Err()` if the values are out of range
    pub fn new(l: f32, a: f32, b: f32) -> ValueResult<LabValue> {
        LabValue {l, a, b}.validate()
    }
}

impl Default for LabValue {
    fn default() -> LabValue {
        LabValue { l: 0.0, a: 0.0, b: 0.0 }
    }
}

impl fmt::Display for LabValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(p) = f.precision() {
            write!(f,
                "[L:{:.*}, a:{:.*}, b:{:.*}]",
                p, self.l, p, self.a, p, self.b
            )
        } else {
            write!(f, "[L:{}, a:{}, b:{}]", self.l, self.a, self.b)
        }
    }
}

/// # Lch: Luminance, Chroma, Hue
///
/// | `Value` | `Color`                    | `Range`            |
/// |:-------:|:--------------------------:|:------------------:|
/// | `L*`    | `Light <---> Dark`         | `0 <---> 100`      |
/// | `c`     | `Chroma (Amount of color)` | `0 <---> 181.0139` |
/// | `h`     | `Hue (Degrees)`            | `0 <---> 360°`     |
///
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct LchValue {
    /// Lightness
    pub l: f32,
    /// Chroma
    pub c: f32,
    /// Hue (in degrees)
    pub h: f32,
}

impl LchValue {
    /// Returns a result of an LchValue from 3 `f32`s.
    /// Will return `Err()` if the values are out of range
    pub fn new(l: f32, c: f32, h: f32) -> ValueResult<LchValue> {
        LchValue { l, c, h }.validate()
    }

    /// Returns the Hue as radians rather than degrees
    pub fn hue_radians(&self) -> f32 {
        self.h.to_radians()
    }
}

impl Default for LchValue {
    fn default() -> LchValue {
        LchValue { l: 0.0, c: 0.0, h: 0.0 }
    }
}

impl fmt::Display for LchValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(p) = f.precision() {
            write!(f,
                "[L:{:.*}, c:{:.*}, h:{:.*}]",
                p, self.l, p, self.c, p, self.h
            )
        } else {
            write!(f, "[L:{}, c:{}, h:{}]", self.l, self.c, self.h)
        }
    }
}

/// # XYZ
///
/// | `Value` | `Color` | `Range`     |
/// |:-------:|:-------:|:-----------:|
/// | `X`     | `Red`   | `0 <---> 1` |
/// | `Y`     | `Green` | `0 <---> 1` |
/// | `Z`     | `Blue`  | `0 <---> 1` |
///
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct XyzValue {
    /// X Value
    pub x: f32,
    /// Y Value
    pub y: f32,
    /// Z Value
    pub z: f32,
}

impl XyzValue {
    /// Returns a result of an XyzValue from 3 `f32`s.
    /// Will return `Err()` if the values are out of range
    pub fn new(x: f32, y: f32, z:f32) -> ValueResult<XyzValue> {
        XyzValue {x, y, z}.validate()
    }

    /// Convert an `XyzValue` to an `RgbValue` in a given `RgbSystem`
    pub fn to_rgb(&self, rgb_system: rgb::RgbSystem) -> RgbValue {
        rgb::xyz_to_rgb(*self, rgb_system)
    }
}

impl Default for XyzValue {
    fn default() -> XyzValue {
        XyzValue { x: 0.0, y: 0.0, z: 0.0 }
    }
}

impl fmt::Display for XyzValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(p) = f.precision() {
            write!(
                f,
                "[X:{:.*}, Y:{:.*}, Z:{:.*}]",
                p, self.x, p, self.y, p, self.z
            )
        } else {
            write!(f, "[X:{}, Y:{}, Z:{}]", self.x, self.y, self.z)
        }
    }
}

/// # RGB: Red, Green, Blue
///
/// | `Value` | `Color` | `Range`       |
/// |:-------:|:-------:|:-------------:|
/// | `R`     | `Red`   | `0 <---> 255` |
/// | `G`     | `Green` | `0 <---> 255` |
/// | `B`     | `Blue`  | `0 <---> 255` |
///
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct RgbValue {
    /// Red
    pub r: u8,
    /// Green
    pub g: u8,
    /// Blue
    pub b: u8,
}

impl RgbValue {
    /// Construct a new `RgbValue` from 3 `u8`s
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        RgbValue { r, g, b }
    }

    /// Convert an `RgbValue` wit a given `RgbSystem` to an `XyzValue`
    pub fn to_xyz(&self, rgb_system: rgb::RgbSystem) -> XyzValue {
        rgb::rgb_to_xyz(*self, rgb_system)
    }

    /// Invert the color
    pub fn invert(&self) -> RgbValue {
        RgbValue {
            r: 255 - self.r,
            g: 255 - self.g,
            b: 255 - self.b,
        }
    }
}

#[test]
fn rgb_invert() {
    let rgb = RgbValue::new(0, 0, 0).invert();
    let exp = RgbValue::new(255, 255, 255);
    assert_eq!(rgb, exp);

    let rgb = RgbValue::new(64, 128, 192).invert();
    let exp = RgbValue::new(191, 127, 63);
    assert_eq!(rgb, exp);
}

impl Default for RgbValue {
    fn default() -> RgbValue {
        RgbValue { r: 0, g: 0, b: 0 }
    }
}

impl fmt::Display for RgbValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[R:{}, G:{}, B:{}]", self.r, self.g, self.b)
    }
}

#[derive(Debug)]
/// Value validation Error type
pub enum ValueError {
    /// The value is outside the acceptable range
    OutOfBounds(String),
    /// The value is formatted incorrectly
    BadFormat(String),
}

impl ValueError {
    /// Wraps an item in a OutOfBounds error
    pub fn out_of_bounds<T: ToString>(t: T) -> Self {
        ValueError::OutOfBounds(t.to_string())
    }

    /// Wraps an item in a BadFormat error
    pub fn bad_format<T: ToString>(t: T) -> Self {
        ValueError::BadFormat(t.to_string())
    }
}

impl fmt::Display for ValueError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", 
            match self {
                ValueError::OutOfBounds(val) => format!("value is out of range: '{}'", val),
                ValueError::BadFormat(val) => format!("value is malformed: '{}'", val),
            }
        )
    }
}

impl Error for ValueError {}
