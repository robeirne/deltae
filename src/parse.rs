//! Definitions for how to parse various types from strings reside here.

use crate::*;
use std::str::FromStr;

impl FromStr for DEMethod {
    type Err = std::io::Error;
    fn from_str(s: &str) -> Result<DEMethod, Self::Err> {
        match s.to_lowercase().trim() {
            "de2000"  | "de00"  | "2000"  | "00"  => Ok(DEMethod::DE2000),
            "de1976"  | "de76"  | "1976"  | "76"  => Ok(DEMethod::DE1976),
            "de1994"  | "de94"  | "1994"  | "94" |
            "de1994g" | "de94g" | "1994g" | "94g" => Ok(DEMethod::DE1994G),
            "de1994t" | "de94t" | "1994t" | "94t" => Ok(DEMethod::DE1994T),
            "decmc"   | "decmc1"| "cmc1"  | "cmc" => Ok(DEMethod::DECMC(1.0, 1.0)),
            "decmc2"  | "cmc2"                    => Ok(DEMethod::DECMC(2.0, 1.0)),
            _ => Err(io::Error::from(io::ErrorKind::InvalidInput)),
        }
    }
}

impl FromStr for LabValue {
    type Err = ValueError;
    fn from_str(s: &str) -> ValueResult<LabValue> {
        let split = parse_str_to_vecf32(s, 3)?;

        LabValue {
            l: split[0],
            a: split[1],
            b: split[2],
        }.validate()
    }
}

impl FromStr for LchValue {
    type Err = ValueError;
    fn from_str(s: &str) -> ValueResult<LchValue> {
        let split = parse_str_to_vecf32(s, 3)?;

        LchValue {
            l: split[0],
            c: split[1],
            h: split[2],
        }.validate()
    }
}

impl FromStr for XyzValue {
    type Err = ValueError;
    fn from_str(s: &str) -> ValueResult<XyzValue> {
        let split = parse_str_to_vecf32(s, 3)?;

        XyzValue {
            x: split[0],
            y: split[1],
            z: split[2],
        }.validate()
    }

}

// Validate and convert strings to `LabValue`.
// Split string by comma (92.5,33.5,-18.8).
fn parse_str_to_vecf32(s: &str, length: usize) -> ValueResult<Vec<f32>> {
    let collection: Vec<&str> = s.split(",").collect();

    // Allow extraneous whitespace ("92.5, 33.5, -18.8")
    let mut v: Vec<&str> = Vec::new();
    for item in collection.iter() {
        if !item.is_empty() {
            v.push(item.trim());
        }
    }
    // Parse the f32's into a Vec
    let split: Vec<f32> = v.iter().filter_map(|s| s.parse().ok()).collect();

    // Check if it's the right number of items
    if v.len() != length || split.len() != length {
        return Err(ValueError::BadFormat);
    }

    Ok(split)
}

