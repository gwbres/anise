/*
 * ANISE Toolkit
 * Copyright (C) 2021-2022 Christopher Rabotin <christopher.rabotin@gmail.com> et al. (cf. AUTHORS.md)
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 *
 * Documentation: https://nyxspace.com/
 */

use std::{collections::HashMap, str::FromStr};

use log::warn;

use crate::{
    prelude::AniseError,
    structure::planetocentric::{phaseangle::PhaseAngle, planetary_constant::PlanetaryConstant},
};

use super::{parser::Assignment, KPLItem};

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum Parameter {
    NutPrecRa,
    NutPrecDec,
    NutPrecPm,
    NutPrecAngles,
    LongAxis,
    PoleRa,
    PoleDec,
    Radii,
    PrimeMeridian,
    GeoMagNorthPoleCenterDipoleLatitude,
    GeoMagNorthPoleCenterDipoleLongitude,
    GravitationalParameter,
}

impl FromStr for Parameter {
    type Err = AniseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "NUT_PREC_RA" => Ok(Self::NutPrecRa),
            "NUT_PREC_DEC" => Ok(Self::NutPrecDec),
            "NUT_PREC_PM" => Ok(Self::NutPrecPm),
            "LONG_AXIS" => Ok(Self::LongAxis),
            "POLE_DEC" => Ok(Self::PoleDec),
            "POLE_RA" => Ok(Self::PoleRa),
            "RADII" => Ok(Self::Radii),
            "PM" => Ok(Self::PrimeMeridian),
            "NUT_PREC_ANGLES" => Ok(Self::NutPrecAngles),
            "N_GEOMAG_CTR_DIPOLE_LAT" => Ok(Self::GeoMagNorthPoleCenterDipoleLatitude),
            "N_GEOMAG_CTR_DIPOLE_LON" => Ok(Self::GeoMagNorthPoleCenterDipoleLongitude),
            "GM" => Ok(Self::GravitationalParameter),
            _ => {
                println!("WHAT? `{s}`");
                Err(AniseError::ParameterNotSpecified)
            }
        }
    }
}

#[derive(Debug, Default)]
pub struct TPCItem {
    pub body_id: Option<i32>,
    pub data: HashMap<Parameter, Vec<f64>>,
}

impl KPLItem for TPCItem {
    type Parameter = Parameter;

    fn extract_key(keyword: &str) -> i32 {
        if keyword.starts_with("BODY") {
            let parts: Vec<&str> = keyword.split('_').collect();
            parts[0][4..].parse::<i32>().unwrap()
        } else {
            -1
        }
    }

    fn data(&self) -> &HashMap<Self::Parameter, Vec<f64>> {
        &self.data
    }

    fn parse(&mut self, data: Assignment) {
        if data.keyword.starts_with("BODY") {
            if let Some((body_info, param)) = data.keyword.split_once('_') {
                let body_id = body_info[4..].parse::<i32>().ok();
                if self.body_id.is_some() && self.body_id != body_id {
                    warn!("Got body {body_id:?} but expected {:?}", self.body_id);
                } else {
                    self.body_id = body_id;
                }
                if let Ok(param) = Parameter::from_str(param) {
                    self.data.insert(param, data.value_to_vec_f64());
                } else {
                    warn!("Unknown parameter `{param}` -- ignoring");
                }
            }
        }
    }
}

#[test]
fn test_parse_pck() {
    use crate::naif::kpl::parser::parse_file;
    let assignments = parse_file::<_, TPCItem>("data/pck00008.tpc", false).unwrap();

    // Tests that we can parse single and multi-line data for Earth related info
    let expt_nutprec = [
        125.045,
        -1935.5364525,
        250.089,
        -3871.072905,
        260.008,
        475263.3328725,
        176.625,
        487269.629985,
        357.529,
        35999.0509575,
        311.589,
        964468.49931,
        134.963,
        477198.869325,
        276.617,
        12006.300765,
        34.226,
        63863.5132425,
        15.134,
        -5806.6093575,
        119.743,
        131.84064,
        239.961,
        6003.1503825,
        25.053,
        473327.79642,
    ];

    assert_eq!(
        assignments[&3].data[&Parameter::NutPrecAngles],
        expt_nutprec
    );
    let expt_pole_ra = [0.0, -0.641, 0.0];
    assert_eq!(assignments[&399].data[&Parameter::PoleRa], expt_pole_ra);

    // Check the same for Jupiter, especially since it has a plus sign in front of the f64
    let expt_pole_pm = [284.95, 870.5366420, 0.0];
    assert_eq!(
        assignments[&599].data[&Parameter::PrimeMeridian],
        expt_pole_pm
    );

    let expt_nutprec = [
        73.32, 91472.9, 24.62, 45137.2, 283.90, 4850.7, 355.80, 1191.3, 119.90, 262.1, 229.80,
        64.3, 352.35, 2382.6, 113.35, 6070.0, 146.64, 182945.8, 49.24, 90274.4,
    ];
    assert_eq!(
        assignments[&5].data[&Parameter::NutPrecAngles],
        expt_nutprec
    );
}

#[test]
fn test_parse_gm() {
    use crate::naif::kpl::parser::parse_file;
    let assignments = parse_file::<_, TPCItem>("data/gm_de431.tpc", false).unwrap();

    // Basic values testing
    assert_eq!(
        assignments[&1].data[&Parameter::GravitationalParameter],
        vec![2.2031780000000021E+04]
    );

    assert_eq!(
        assignments[&399].data[&Parameter::GravitationalParameter],
        vec![3.9860043543609598E+05]
    );
}

#[test]
fn test_anise_conversion() {
    use crate::naif::kpl::parser::parse_file;
    let gravity_data = parse_file::<_, TPCItem>("data/gm_de431.tpc", false).unwrap();
    let mut planetary_data = parse_file::<_, TPCItem>("data/pck00008.tpc", false).unwrap();

    for (key, value) in gravity_data {
        match planetary_data.get_mut(&key) {
            Some(planet_data) => {
                for (gk, gv) in value.data {
                    planet_data.data.insert(gk, gv);
                }
            }
            None => {}
        }
    }

    // Now that planetary_data has everything, we'll create a vector of the planetary data in the ANISE ASN1 format.

    let mut anise_data = vec![];
    for (body_id, planetary_data) in planetary_data {
        dbg!(body_id);
        let radii_km = &planetary_data.data[&Parameter::Radii];

        let pola_ra = &planetary_data.data[&Parameter::PoleRa];
        let pola_dec = &planetary_data.data[&Parameter::PoleDec];
        let prime_mer = &planetary_data.data[&Parameter::PrimeMeridian];

        let constants = PlanetaryConstant {
            semi_major_radii_km: radii_km[0],
            semi_minor_radii_km: radii_km[1],
            polar_radii_km: radii_km[2],
            pole_right_ascension: PhaseAngle {
                offset_deg: *(pola_ra.get(0).or(Some(&0.0)).unwrap()),
                rate_deg: *(pola_ra.get(1).or(Some(&0.0)).unwrap()),
                accel_deg: *(pola_ra.get(2).or(Some(&0.0)).unwrap()),
            },
            pole_declination: PhaseAngle {
                offset_deg: *(pola_dec.get(0).or(Some(&0.0)).unwrap()),
                rate_deg: *(pola_dec.get(1).or(Some(&0.0)).unwrap()),
                accel_deg: *(pola_dec.get(2).or(Some(&0.0)).unwrap()),
            },
            prime_meridian: PhaseAngle {
                offset_deg: *(prime_mer.get(0).or(Some(&0.0)).unwrap()),
                rate_deg: *(prime_mer.get(1).or(Some(&0.0)).unwrap()),
                accel_deg: *(prime_mer.get(2).or(Some(&0.0)).unwrap()),
            },
            nut_prec_angles: Default::default(),
        };
        anise_data.push(constants);
    }
}
