use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default)]
pub struct Restriction {
    pub doubloon_ratio: f64,
    pub cube_ratio: f64,
    pub cong_chips_ratio: f64,
    pub time_ratio: f64,
    pub ultra_blp_ratio: f64,
    pub ultra_equip_ratio: f64,
    pub fni_5_super_r: i8,
    pub fni_5_ultra_r: i8,
    pub fni_f: i8,
    pub do_data_collection: bool,
    pub do_research_assignment: bool,
}

impl Restriction {
    pub fn new() -> Restriction {
        Restriction {
            do_data_collection: true,
            do_research_assignment: true,
            ..Default::default()
        }
    }

    pub fn predef() -> Restriction {
        Restriction {
            cube_ratio: 110.0,
            ultra_blp_ratio: 3.0,
            ultra_equip_ratio: 20.0,
            fni_f: 11,
            do_data_collection: true,
            do_research_assignment: true,
            ..Default::default()
        }
    }
}
