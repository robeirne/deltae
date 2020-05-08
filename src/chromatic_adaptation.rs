use crate::*;
use matrix::*;
use illuminant::*;

pub enum ChromaticAdaptationMethod {
    XyzScaling,
    Bradford,
    VonKries,
}

impl XyzValue {
    fn chrom_adapt(self, source: Illuminant, dest: Illuminant) {

    }
}

/// Cone response domain matrix for the XYZ Scaling chromatic adaptation method (same for inverse)
pub const XYZ_SCALING: Matrix3x3 = matrix3x3![
    1.0000000, 0.0000000, 0.0000000;
    0.0000000, 1.0000000, 0.0000000;
    0.0000000, 0.0000000, 1.0000000;
];

/// Cone response domain matrix for the Bradford chromatic adaptation method
pub const BRADFORD: Matrix3x3 = matrix3x3![
    0.8951000, 0.2664000, -0.1614000;
    -0.7502000, 1.7135000, 0.0367000;
    0.0389000,  -0.0685000, 1.0296000;
];

#[test]
fn derp() {
    dbg!(BRADFORD.pow(-1.0));
}

/// Inverse cone response domain matrix for the Bradford chromatic adaptation method
pub const BRADFORD_INV: Matrix3x3 = matrix3x3![
    0.9869929, -0.1470543, 0.1599627;
    0.4323053, 0.5183603, 0.0492912;
    -0.0085287, 0.0400428, 0.9684867;
];

/// Cone response domain matrix for the Von Kries chromatic adaptation method
pub const VON_KRIES: Matrix3x3 = matrix3x3![
    0.4002400, 0.7076000, -0.0808100;
    -0.2263000, 1.1653200, 0.0457000;
    0.0000000, 0.0000000, 0.9182200;
];

/// Inverse cone response domain matrix for the Von Kries chromatic adaptation method
pub const VON_KRIES_INV: Matrix3x3 = matrix3x3![
    1.8599364, -1.1293816, 0.2198974;
    0.3611914, 0.6388125, -0.0000064;
    0.0000000, 0.0000000, 1.0890636;
];
