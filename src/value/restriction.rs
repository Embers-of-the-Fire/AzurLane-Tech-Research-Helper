//! This file includes the restriction inputted by the user to reproduce. All basic restriction parts are all wrapped by `enum_val::Strict`.

use crate::enum_val::*;

/// # Struct ResourceRestriction
/// > all fields ar `Strict<f64>` except `select`
///
/// fields:
///  - `doubloon`: maximum doubloon cost per day.
///  - `cube`: maximum cube cost per day.
///  - `ssr_blp`: minimum ssr_blp produced per day.
///  - `ur_blp`: minimum ur_blp produced per day.
///  - `ur_equip`: minimum ur_equip produced per day.
///  - `cogn_chip`: minimum cogn_chip produced per day.
///  - `time`: maximum time a user can be online.
///  - `select`: limit number of projects to select.
#[derive(Debug, Clone, Copy)]
pub struct ResourceRestriction {
    pub doubloon: Strict<f64>,
    pub cube: Strict<f64>,
    pub ssr_blp: Strict<f64>,
    pub ur_blp: Strict<f64>,
    pub ur_equip: Strict<f64>,
    pub cogn_chip: Strict<f64>,
    pub time: Strict<f64>,
    pub select: u8,
}

impl ResourceRestriction {
    
    /// Create a default ResourceRestriction. All fields are filled with `Strict{ data: 0.0_f64, strict: false }` except `select` as `10u8`.
    pub fn empty() -> ResourceRestriction {
        ResourceRestriction {
            doubloon: Strict::None,
            cube: Strict::None,
            ssr_blp: Strict::None,
            ur_blp: Strict::None,
            ur_equip: Strict::None,
            cogn_chip: Strict::None,
            time: Strict::None,
            select: 10,
        }
    }

    pub fn new(doubloon: Strict<f64>, cube: Strict<f64>, ssr_blp: Strict<f64>, ur_blp: Strict<f64>, ur_equip: Strict<f64>, cogn_chip: Strict<f64>, timei: Strict<f64>, select: u8) -> ResourceRestriction {
        ResourceRestriction { doubloon, cube, ssr_blp, ur_blp, ur_equip, cogn_chip, time: match timei {
            Strict::None => Strict::Loose(24.0),
            _ => timei,
        }, select }
    }
}
