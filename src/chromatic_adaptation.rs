use crate::*;
use matrix::*;
use illuminant::*;

pub use ChromaticAdaptationMethod::*;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ChromaticAdaptationMethod {
    XyzScaling,
    Bradford,
    VonKries,
}

pub struct ConeResponseDomain {
    rho: f32,
    gamma: f32,
    beta: f32,
}

impl ConeResponseDomain {
    pub fn scaled_component_matrix(self, dest: Self) -> Matrix3x3 {
        matrix3x3![
            dest.rho/self.rho, 0.0, 0.0;
            0.0, dest.gamma/self.gamma, 0.0;
            0.0, 0.0, dest.beta/self.beta;
        ]
    }
}

impl From<Matrix3x1> for ConeResponseDomain {
    fn from(matrix: Matrix3x1) -> Self {
        ConeResponseDomain {
            rho: matrix[0],
            gamma: matrix[1],
            beta: matrix[2],
        }
    }
}

impl From<ConeResponseDomain> for Matrix3x1 {
    fn from(crd: ConeResponseDomain) -> Self {
        Matrix3x1::new(crd.rho, crd.gamma, crd.beta)
    }
}

impl XyzValue {
    /// Adapt an `XyzValue` to another Illuminant using a given
    /// `ChromaticAdaptationMethod`
    pub fn chrom_adapt(
        self,
        method: ChromaticAdaptationMethod,
        source: Illuminant,
        dest: Illuminant
    ) -> Self {
        let method_matrix = match method {
            XyzScaling => XYZ_SCALING,
            Bradford => BRADFORD,
            VonKries => VON_KRIES,
        };

        let method_matrix_inv = match method {
            XyzScaling => XYZ_SCALING,
            Bradford => BRADFORD_INV,
            VonKries => VON_KRIES_INV,
        };

        let crd_source: ConeResponseDomain =
            (method_matrix * Matrix3x1::from(source)).into();
        let crd_dest: ConeResponseDomain =
            (method_matrix * Matrix3x1::from(dest)).into();

        let scm = crd_source.scaled_component_matrix(crd_dest);

        let matrix = method_matrix_inv * scm * method_matrix;

        (matrix * Matrix3x1::from(source)).into()
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
