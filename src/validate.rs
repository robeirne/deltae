use super::*;

/// Trait to validate whether a type has appropriate values
pub trait Validate where Self: Sized {
    /// Return `Err()` if the values are invalid
    fn validate(self) -> ValueResult<Self>;
}

impl Validate for CieLabValue {
    fn validate(self) -> ValueResult<Self> {
        if self.l < 0.0    || self.l > 100.0 ||
           self.a < -128.0 || self.a > 128.0 ||
           self.b < -128.0 || self.b > 128.0
        {
            Err(ValueError::out_of_bounds(self))
        } else {
            Ok(self)
        }
    }
}

impl Validate for LchValue {
    fn validate(self) -> ValueResult<Self> {
        if self.l < 0.0 || self.l > 100.0 ||
           self.c < 0.0 || self.c > (128_f64.powi(2) + 128_f64.powi(2)).sqrt() ||
           self.h < 0.0 || self.h > 360.0
        {
            Err(ValueError::out_of_bounds(self))
        } else {
            Ok(self)
        }
    }
}

/// Not sure about the bounds on XYZ
impl Validate for CieXyzValue {
    fn validate(self) -> ValueResult<Self> {
        if self.x < 0.0 || self.x > 1.0 ||
           self.y < 0.0 || self.y > 1.0 ||
           self.z < 0.0 || self.z > 1.0
        {
            Err(ValueError::out_of_bounds(self))
        } else {
            Ok(self)
        }
    }
}
