

use crate::{restriction::Restriction, actual};

pub const PROJECT_TYPE: [&str; 12] = ["魔方解析", "心智补全", "舰装解析", "金船定向", "彩船定向", "资金募集", "试验品募集蓝", "试验品募集紫", "数据收集紫", "数据收集金", "基础研究", "研究委托"];

pub const NONE_DIRECT_BLP: [i8; 2] = [3, 4];

pub fn if_none_direct_blp(d: i8) -> bool {
    return NONE_DIRECT_BLP.contains(&d);
}

fn cvt_str2i8(proj_type: &str) -> i8 {
    let mut k: i8 = 0;
    for i in PROJECT_TYPE {
        if i == proj_type {
            return k
        }
        k += 1;
    }
    panic!("Invalid project type: {}", proj_type);
}

pub fn is_ssr(proj_type: i8) -> bool {
    proj_type == 3
}

pub fn is_ur(proj_type: i8) -> bool {
    proj_type == 4
}

pub fn is_data_collection(proj_type: i8) -> bool {
    proj_type == 8 || proj_type == 9
}

pub fn is_research_assignment(proj_type: i8) -> bool {
    proj_type == 11
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[derive()]
pub struct ResearchData {
    pub research_id: i8,
    pub proj_type: i8,
    pub time: f64,
    pub rate: f64,
    pub doubloon: i16,
    pub cube: i8,
    pub super_rare_blp: f64,
    pub ultra_rare_blp: f64,
    pub ultra_rare_equip: f64,
    pub cognitive_chips: f64
}

impl ResearchData {
    pub fn new(research_id: i8, proj_type: &str, time: f64, rate: f64, doubloon: i16, cube: i8, super_rare_blp: f64, ultra_rare_blp: f64, ultra_rare_equip: f64, cognitive_chips: f64) -> ResearchData {
        ResearchData { research_id, proj_type: cvt_str2i8(proj_type), time, rate, doubloon, cube, super_rare_blp, ultra_rare_blp, ultra_rare_equip, cognitive_chips }
    }
    
    /// 是否为**定向蓝图**
    pub fn is_none_direct(&self) -> bool {
        return if_none_direct_blp(self.proj_type);
    }
}

#[derive(Debug, Clone)]
pub struct Data{ pub data: Vec<ResearchData> }

impl Data {
    pub fn load() -> Self {
        let mut m = Vec::new();
        m.push(ResearchData::new(0, "魔方解析", 1.0, 0.116201788833198, 0, 3, 2.77, 0.66, 0.0, 40.0));
        m.push(ResearchData::new(1, "魔方解析", 2.0, 0.0726765528249297, 0, 6, 2.03, 0.48, 0.0, 30.16));
        m.push(ResearchData::new(2, "魔方解析", 4.0, 0.0112721787344704, 0, 10, 1.39, 0.33, 0.0, 20.77));
        m.push(ResearchData::new(3, "心智补全", 0.5, 0.000857391671078282, 8000, 3, 9.88, 2.34, 0.0, 194.49));
        m.push(ResearchData::new(4, "舰装解析", 1.0, 0.0924217786618207, 0, 0, 0.0, 0.0, 0.042, 0.0));
        m.push(ResearchData::new(5, "舰装解析", 2.0, 0.0448869757211571, 0, 0, 0.0, 0.0, 0.042, 0.0));
        m.push(ResearchData::new(6, "舰装解析", 4.0, 0.0229982707065704, 0, 0, 0.0, 0.0, 0.042, 0.0));
        m.push(ResearchData::new(7, "舰装解析", 0.5, 0.00900261254632197, 5000, 0, 0.0, 0.0, 0.693, 0.0));
        m.push(ResearchData::new(8, "金船定向", 2.5, 0.119338247215807, 3000, 0, 0.91, 0.0, 0.016, 0.0));
        m.push(ResearchData::new(9, "金船定向", 5.0, 0.0761934876473689, 5000, 0, 0.75, 0.0, 0.011, 0.0));
        m.push(ResearchData::new(10, "金船定向", 8.0, 0.012284819566333, 8000, 0, 0.75, 0.0, 0.011, 0.0));
        m.push(ResearchData::new(11, "金船定向", 0.5, 0.00179441184676775, 5000, 5, 18.0, 0.0, 0.25, 0.0));
        m.push(ResearchData::new(12, "彩船定向", 2.5, 0.119338247215807, 3000, 0, 0.0, 0.61, 0.016, 0.0));
        m.push(ResearchData::new(13, "彩船定向", 5.0, 0.0761934876473689, 5000, 0, 0.0, 0.51, 0.011, 0.0));
        m.push(ResearchData::new(14, "彩船定向", 8.0, 0.012284819566333, 8000, 0, 0.0, 0.51, 0.011, 0.0));
        m.push(ResearchData::new(15, "彩船定向", 0.5, 0.00179441184676775, 5000, 5, 0.0, 11.77, 0.25, 0.0));
        m.push(ResearchData::new(16, "资金募集", 1.5, 0.0998104774743483, 1500, 0, 0.55, 0.13, 0.018, 0.0));
        m.push(ResearchData::new(17, "资金募集", 2.5, 0.0650861147956779, 3000, 0, 0.46, 0.11, 0.014, 0.0));
        m.push(ResearchData::new(18, "资金募集", 4.0, 0.050838282614524, 6000, 0, 0.52, 0.12, 0.03, 0.0));
        m.push(ResearchData::new(19, "试验品募集蓝", 2.0, 0.0182573991135493, 0, 0, 0.0, 0.0, 0.019, 0.0));
        m.push(ResearchData::new(20, "试验品募集紫", 2.0, 0.0134913101184377, 0, 0, 0.0, 0.0, 0.025, 0.0));
        m.push(ResearchData::new(21, "数据收集紫", 4.0, 0.0171226160194751, 0, 0, 0.2, 0.05, 0.015, 0.0));
        m.push(ResearchData::new(22, "数据收集金", 4.0, 0.00716174219371271, 0, 0, 0.2, 0.05, 0.018, 0.0));
        m.push(ResearchData::new(23, "基础研究", 6.0, 0.0479130639720217, 0, 0, 0.0, 0.0, 0.011, 0.0));
        m.push(ResearchData::new(24, "基础研究", 8.0, 0.0342200146368597, 0, 0, 0.06, 0.014, 0.011, 0.0));
        m.push(ResearchData::new(25, "基础研究", 12.0, 0.00857391671078283, 0, 0, 0.06, 0.014, 0.011, 0.0));
        m.push(ResearchData::new(26, "研究委托", 3.0, 0.0174504422466521, 0, 0, 0.0, 0.0, 0.012, 0.0));
        m.push(ResearchData::new(27, "研究委托", 4.0, 0.0139200059539768, 0, 0, 0.0, 0.0, 0.012, 0.0));
        m.push(ResearchData::new(28, "研究委托", 6.0, 0.00887652553586928, 0, 0, 0.0, 0.0, 0.012, 0.0));
        Data{ data: m }
    }

    pub fn refresh_with_actual_ratio(&mut self, a: &actual::ActualRatio) {
        for i in self.data.iter_mut() {
            if is_ssr(i.proj_type) {
                i.rate *= a.super_rare_5_blp;
            } else if is_ur(i.proj_type) {
                i.rate *= a.ultra_rare_5_blp;
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct ReferenceValue {
    pub doubloon: f64,
    pub cube: f64,
    pub time_of_an_hour: f64,
    pub super_rare: f64,
    pub ultra_rare: f64,
    pub ultra_equip: f64,
    pub cong_chips: f64,
    pub time_ratio: f64
}

impl ReferenceValue {
    pub fn new() -> ReferenceValue {
        ReferenceValue {
            doubloon: 0.0,
            cube: 0.0,
            time_of_an_hour: 0.0,
            super_rare: 0.0,
            ultra_rare: 0.0,
            ultra_equip: 0.0,
            cong_chips: 0.0,
            time_ratio: 0.0
        }
    }
    pub fn predef() -> ReferenceValue {
        ReferenceValue {
            doubloon: 0.5,
            cube: 1000.0,
            time_of_an_hour: 1500.0,
            super_rare: 1200.0,
            ultra_rare: 3000.0,
            ultra_equip: 36000.0,
            cong_chips: 60.0,
            time_ratio: 0.0
        }
    }

    pub fn from(
        doubloon: f64,
        cube: f64,
        time_of_an_hour: f64,
        super_rare: f64,
        ultra_rare: f64,
        ultra_equip: f64,
        cong_chips: f64,
        time_ratio: f64,
    ) -> ReferenceValue {
        ReferenceValue { doubloon, cube, time_of_an_hour, super_rare, ultra_rare, ultra_equip, cong_chips, time_ratio }
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
