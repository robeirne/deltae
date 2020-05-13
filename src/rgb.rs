//! Definitions for working with `RgbValue`s

use crate::*;
use matrix::*;

/// A reference RGB system, typically associated with an ICC Profile
#[derive(Debug, Copy, Clone)]
pub enum RgbSystem {
    /// Adobe Systems
    Adobe1998,
    /// Apple
    Apple,
    /// Like DonRGB but with a modified red coordinate
    Best,
    /// A compromise between ColorMatch and Adobe
    Bruce,
    /// Commission internationale de l'éclairage
    CIE,
    /// Slightly larger gamut than sRGB with D50 and 1.8 gamma
    ColorMatch,
    /// Wide-gamut working space featuring industry-standard D-50 white point and 2.2 gamma
    Don,
    /// European Color Initiative
    ECI,
    /// Ekta Space PS 5 / EktaChrome Space
    EktaSpace,
    /// National Television System Committee analog color television encoding
    NTSC,
    /// PAL: Phase Alternating Line;
    /// SECAM: Séquentiel couleur à mémoire (Sequential colour with memory)
    PalSecam,
    /// Large gamut RGB space developed by Kodak,
    /// also known as ROMM RGB (Reference Output Medium Metric)
    ProPhoto,
    /// Society of Motion Picture and Television Engineers
    SMPTE,
    /// Standard Red, Green, and Blue. Developed by Microsoft and HP
    SRgb,
    /// Like AdobeRGB but with larger gamut
    WideGamut,
}

impl Default for RgbSystem {
    fn default() -> Self {
        RgbSystem::SRgb
    }
}

#[derive(Debug, Default, Copy, Clone)]
/// An `RgbValue` within an `RgbSystem`
pub struct RgbSystemValue {
    value: RgbValue,
    system: RgbSystem,
}

impl RgbSystemValue {
    /// Create a new RgbSystemValue
    pub fn new(value: RgbValue, system: RgbSystem) -> Self {
        RgbSystemValue { value, system }
    }
}

pub(crate) fn xyz_to_rgb(xyz: XyzValue, rgb_system: RgbSystem) -> RgbValue {
    let matrix = match rgb_system {
        RgbSystem::Adobe1998 => ADOBERGB_1998_D65_XYZ2RGB,
        RgbSystem::Apple => APPLERGB_D65_XYZ2RGB,
        RgbSystem::Best => BESTRGB_D50_XYZ2RGB,
        RgbSystem::Bruce => BRUCERGB_D65_XYZ2RGB,
        RgbSystem::CIE => CIERGB_E_XYZ2RGB,
        RgbSystem::ColorMatch => COLORMATCHRGB_D50_XYZ2RGB,
        RgbSystem::Don => DONRGB4_D50_XYZ2RGB,
        RgbSystem::ECI => ECIRGB_D50_XYZ2RGB,
        RgbSystem::EktaSpace => EKTASPACE_PS5_D50_XYZ2RGB,
        RgbSystem::NTSC => NTSCRGB_C_XYZ2RGB,
        RgbSystem::PalSecam => PALSECAMRGB_D65_XYZ2RGB,
        RgbSystem::ProPhoto => PROPHOTORGB_D50_XYZ2RGB,
        RgbSystem::SMPTE => SMPTERGB_D65_XYZ2RGB,
        RgbSystem::SRgb => SRGB_D65_XYZ2RGB,
        RgbSystem::WideGamut => WIDEGAMUTRGB_D50_XYZ2RGB,
    };

    (matrix * Matrix3x1::from(xyz)).into()
}

pub(crate) fn rgb_to_xyz(rgb: RgbValue, rgb_system: RgbSystem) -> XyzValue {
    let matrix = match rgb_system {
        RgbSystem::Adobe1998 => ADOBERGB_1998_D65_RGB2XYZ,
        RgbSystem::Apple => APPLERGB_D65_RGB2XYZ,
        RgbSystem::Best => BESTRGB_D50_RGB2XYZ,
        RgbSystem::Bruce => BRUCERGB_D65_RGB2XYZ,
        RgbSystem::CIE => CIERGB_E_RGB2XYZ,
        RgbSystem::ColorMatch => COLORMATCHRGB_D50_RGB2XYZ,
        RgbSystem::Don => DONRGB4_D50_RGB2XYZ,
        RgbSystem::ECI => ECIRGB_D50_RGB2XYZ,
        RgbSystem::EktaSpace => EKTASPACE_PS5_D50_RGB2XYZ,
        RgbSystem::NTSC => NTSCRGB_C_RGB2XYZ,
        RgbSystem::PalSecam => PALSECAMRGB_D65_RGB2XYZ,
        RgbSystem::ProPhoto => PROPHOTORGB_D50_RGB2XYZ,
        RgbSystem::SMPTE => SMPTERGB_D65_RGB2XYZ,
        RgbSystem::SRgb => SRGB_D65_RGB2XYZ,
        RgbSystem::WideGamut => WIDEGAMUTRGB_D50_RGB2XYZ,
    };

    (matrix * Matrix3x1::from(rgb)).into()
}

#[test]
fn convert_rgb() {
    use chromatic_adaptation::*;
    use illuminant::*;

    let xyz = XyzValue::new(0.208137, 0.215861, 0.178130).unwrap();
    let lab = LabValue::from(xyz);
    dbg!(lab);
    let exp = LabValue::new(53.5850, 0.0, 0.0).unwrap();
    //assert_almost_eq!(lab, exp);

    dbg!(xyz.chrom_adapt(Bradford, Illuminant::D50, Illuminant::D65));
}
