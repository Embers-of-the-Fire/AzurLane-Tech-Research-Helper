//! Resources reference value.

use crate::enum_val::*;

#[derive(Debug, Clone, Copy)]
pub struct ResourceValue {
    pub doubloon: f64,
    pub cube: f64,
    pub ssr_blp: f64,
    pub ur_blp: f64,
    pub indirect_ssr_blp: f64,
    pub indirect_ur_blp: f64,
    pub ur_equip: f64,
    pub cogn_chip: f64,
    pub rate_factor: f64,
}

impl ResourceValue {
    /// Create a default ResourceValue. All fields are filled with `0.0_f64`.
    
    pub fn empty() -> ResourceValue {
        ResourceValue {
            doubloon: 0.0,
            cube: 0.0,
            ssr_blp: 0.0,
            ur_blp: 0.0,
            indirect_ssr_blp: 0.0,
            indirect_ur_blp: 0.0,
            ur_equip: 0.0,
            cogn_chip: 0.0,
            rate_factor: 0.0,
        }
    }

    
    pub fn predef(ur_ship: u8, ssr_ship: u8, former_ship: u8) -> ResourceValue {
        ResourceValue {
            doubloon: 0.5,
            cube: 1000.0,
            ssr_blp: 1200.0,
            ur_blp: 3000.0,
            ur_equip: 36000.0,
            indirect_ssr_blp: if ssr_ship + ur_ship == 5 { 1.0 } else { ssr_ship as f64 / 3.0 } * 1200.0,
            indirect_ur_blp: if ssr_ship + ur_ship == 5 { 1.0 } else { ur_ship as f64 / 2.0 } * 3000.0,
            cogn_chip: 60.0,
            rate_factor: 4.0 * if ssr_ship + ur_ship == 5 { 5.0 } else { (ssr_ship + ur_ship) as f64 } / former_ship as f64,
        }
    }

    /// Create a new ResourceValue
    ///
    /// `ur_blp` can be given by enum ReferVal, and `ReferVal::Refer(f64)` represents the reference factor to `ssr_blp`.
    ///
    /// `ur_equip` can be given by enum ReferVal, and `ReferVal::Refer(f64)` represents the reference factor to `ur_blp`.
    pub fn new(
        doubloon: f64,
        cube: f64,
        ssr_blp: f64,
        ur_blp: ReferVal<f64, f64>,
        ur_equip: ReferVal<f64, f64>,
        cogn_chip: f64,
        ur_ship: u8,
        ssr_ship: u8,
        former_ship: u8,
    ) -> ResourceValue {
        let mut rv = ResourceValue {
            doubloon,
            cube,
            ssr_blp,
            ur_blp: match ur_blp {
                ReferVal::Refer(v) => ssr_blp * v,
                ReferVal::Value(v) => v,
            },
            indirect_ssr_blp: if ssr_ship + ur_ship == 5 { 1.0 } else { ssr_ship as f64 / 3.0 } * ssr_blp,
            indirect_ur_blp: 0.0,
            ur_equip: 0.0,
            cogn_chip,
            rate_factor: 4.0 * if ssr_ship + ur_ship == 5 { 5.0 } else { (ssr_ship + ur_ship) as f64 } / former_ship as f64,
        };
        rv.ur_equip = match ur_equip {
            ReferVal::Refer(v) => rv.ur_blp * v,
            ReferVal::Value(v) => v,
        };
        rv.indirect_ur_blp = if ssr_ship + ur_ship == 5 { 1.0 } else { ur_ship as f64 / 2.0 } * rv.ur_blp;
        rv
    }
}
