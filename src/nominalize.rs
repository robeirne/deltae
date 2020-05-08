//! # Nominalize/DeNominalize
//!
//! Nominalizing converts a type's values to values within the range of 0 to 1. Denominalizing
//! converts the nominal values to the normal value range.
use crate::*;

/// A nominalized RGB value on a scale from 0 to 1
#[derive(Debug, Clone, PartialEq)]
pub struct RgbNominalValue {
    /// Red
    pub r: f32,
    /// Green
    pub g: f32,
    /// Blue
    pub b: f32,
}

impl Default for RgbNominalValue {
    fn default() -> Self {
        RgbNominalValue::from(RgbValue::default())
    }
}

impl From<RgbNominalValue> for RgbValue {
    fn from(rgb_nom: RgbNominalValue) -> Self {
        rgb_nom.denominalize()
    }
}

impl From<RgbValue> for RgbNominalValue {
    fn from(rgb: RgbValue) -> Self {
        rgb.nominalize()
    }
}

/// A trait to nominalize values on a scale from 0 to 1
pub trait Nominalize {
    /// The type to convert to when nominalizing
    type Output;
    /// Defines how to nominalize a type
    fn nominalize(&self) -> Self::Output;
}

impl Nominalize for RgbValue {
    type Output = RgbNominalValue;
    fn nominalize(&self) -> Self::Output {
        RgbNominalValue {
            r: self.r as f32 / 255.0,
            g: self.g as f32 / 255.0,
            b: self.b as f32 / 255.0,
        }
    }

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
    assert_eq!(RgbValue::default().nominalize().r, 0.0);
}

/// A trait to denominalize values from a 0 to 1 scale to something else
pub trait DeNominalize {
    /// The type to convert to when denominalizing
    type Output;
    /// Defines how to denominalize a type
    fn denominalize(&self) -> Self::Output;
}

impl DeNominalize for RgbNominalValue {
    type Output = RgbValue;
    fn denominalize(&self) -> Self::Output {
        RgbValue {
            r: (self.r * 255.0) as u8,
            g: (self.g * 255.0) as u8,
            b: (self.b * 255.0) as u8,
        }
    }
}

#[test]
fn denominalize_rgb() {
    let nom = RgbNominalValue {
        r: 0.2509804,
        g: 0.5019608,
        b: 1.0,
    };
    let rgb = nom.denominalize();
    let exp = RgbValue::new(64, 128, 255);
    assert_eq!(rgb, exp);
    assert_eq!(RgbNominalValue::default().denominalize().r, 0);
}

pub(crate) trait Clamp {
    fn clamp(self) -> Self;
}

impl Clamp for RgbNominalValue {
    fn clamp(self) -> Self {
        RgbNominalValue {
            r: clamp(self.r, 0.0, 1.0),
            g: clamp(self.g, 0.0, 1.0),
            b: clamp(self.b, 0.0, 1.0),
        }
    }
}

fn clamp(val: f32, low: f32, high: f32) -> f32 {
    if val < low {
        low
    } else if val > high {
        high
    } else {
        val
    }
}
