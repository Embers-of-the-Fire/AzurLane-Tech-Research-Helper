use serde::{Deserialize, Serialize};

use crate::{actual, restriction::Restriction};

pub const PROJECT_TYPE: [&str; 12] = [
    "魔方解析",
    "心智补全",
    "舰装解析",
    "金船定向",
    "彩船定向",
    "资金募集",
    "试验品募集蓝",
    "试验品募集紫",
    "数据收集紫",
    "数据收集金",
    "基础研究",
    "研究委托",
];

pub const NONE_DIRECT_BLP: [u8; 2] = [3, 4];

pub fn if_none_direct_blp(d: u8) -> bool {
    NONE_DIRECT_BLP.contains(&d)
}

fn cvt_str2u8(proj_type: &str) -> u8 {
    PROJECT_TYPE
        .iter()
        .enumerate()
        .find(|(_, s)| **s == proj_type)
        .map(|s| s.0 as u8)
        .expect("invalid project type")
}

pub fn is_ssr(proj_type: u8) -> bool {
    proj_type == 3
}

pub fn is_ur(proj_type: u8) -> bool {
    proj_type == 4
}

pub fn is_data_collection(proj_type: u8) -> bool {
    proj_type == 8 || proj_type == 9
}

pub fn is_research_assignment(proj_type: u8) -> bool {
    proj_type == 11
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Default)]
pub struct ResearchData {
    pub research_id: u8,
    pub proj_type: u8,
    pub time: f64,
    pub rate: f64,
    pub doubloon: i16,
    pub cube: i8,
    pub super_rare_blp: f64,
    pub ultra_rare_blp: f64,
    pub ultra_rare_equip: f64,
    pub cognitive_chips: f64,
}

impl ResearchData {
    pub fn with_projtype(mut self, proj_type: &str) -> Self {
        self.proj_type = cvt_str2u8(proj_type);
        self
    }

    /// 是否为**定向蓝图**
    pub fn is_none_direct(&self) -> bool {
        if_none_direct_blp(self.proj_type)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Data {
    pub data: Vec<ResearchData>,
}

impl Data {
    pub fn load() -> Self {
        let mut m = Vec::new();
        include!(concat!(env!("OUT_DIR"), "/dataset.tera.rs"));
        Data { data: m }
    }

    pub fn refresh_with_actual_ratio(&mut self, a: &actual::ActualRatio) {
        for i in self.data.iter_mut() {
            if is_ssr(i.proj_type) {
                i.rate *= a.super_rare_6_blp;
            } else if is_ur(i.proj_type) {
                i.rate *= a.ultra_rare_6_blp;
            }
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default)]
pub struct ReferenceValue {
    pub doubloon: f64,
    pub cube: f64,
    pub time_of_an_hour: f64,
    pub super_rare: f64,
    pub ultra_rare: f64,
    pub ultra_equip: f64,
    pub cong_chips: f64,
    pub time_ratio: f64,
}

impl ReferenceValue {
    pub fn predef() -> ReferenceValue {
        ReferenceValue {
            doubloon: 0.5,
            cube: 1000.0,
            time_of_an_hour: 1500.0,
            super_rare: 1200.0,
            ultra_rare: 3000.0,
            ultra_equip: 36000.0,
            cong_chips: 60.0,
            time_ratio: 0.0,
        }
    }

    pub fn actual(&mut self, rest: &Restriction) {
        self.doubloon *= rest.doubloon_ratio / 100.0;
        self.cube *= rest.cube_ratio / 100.0;
        self.time_of_an_hour *= (100.0 - rest.time_ratio) / 100.0;
        self.super_rare *= 5.9 / (1.0 + rest.ultra_blp_ratio + rest.ultra_equip_ratio / 5.0);
        self.ultra_rare = self.super_rare * rest.ultra_blp_ratio;
        self.ultra_equip = self.ultra_rare * rest.ultra_equip_ratio;
        self.cong_chips *= rest.cong_chips_ratio / 100.0;
        self.time_ratio = 72.0 * rest.time_ratio;
    }
}
