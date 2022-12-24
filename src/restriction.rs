use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
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
            doubloon_ratio: 0.0,
            cube_ratio: 0.0,
            cong_chips_ratio: 0.0,
            time_ratio: 0.0,
            ultra_blp_ratio: 0.0,
            ultra_equip_ratio: 0.0,
            fni_5_super_r: 0,
            fni_5_ultra_r: 0,
            fni_f: 0,
            do_data_collection: true,
            do_research_assignment: true,
        }
    }

    pub fn predef() -> Restriction {
        Restriction {
            doubloon_ratio: 0.0,
            cube_ratio: 110.0,
            cong_chips_ratio: 0.0,
            time_ratio: 0.0,
            ultra_blp_ratio: 3.0,
            ultra_equip_ratio: 20.0,
            fni_5_super_r: 0,
            fni_5_ultra_r: 0,
            fni_f: 11,
            do_data_collection: true,
            do_research_assignment: true,
        }
    }
    pub fn from(
        doubloon_ratio: f64,
        cube_ratio: f64,
        cong_chips_ratio: f64,
        time_ratio: f64,
        ultra_blp_ratio: f64,
        ultra_equip_ratio: f64,
        fni_5_super_r: i8,
        fni_5_ultra_r: i8,
        fni_f: i8,
        do_data_collection: bool,
        do_research_assignment: bool,
    ) -> Restriction {
        Restriction {
            doubloon_ratio,
            cube_ratio,
            cong_chips_ratio,
            time_ratio,
            ultra_blp_ratio,
            ultra_equip_ratio,
            fni_5_super_r,
            fni_5_ultra_r,
            fni_f,
            do_data_collection,
            do_research_assignment,
        }
    }
}


