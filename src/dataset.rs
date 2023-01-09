//! Basic data for research projects, including products, costs and research types.

pub const PROJECT_TYPE: [&str; 12] = ["魔方解析", "心智补全", "舰装解析", "金船定向", "彩船定向", "资金募集", "试验品募集蓝", "试验品募集紫", "数据收集紫", "数据收集金", "基础研究", "研究委托"];
pub const PROJECT_ID: [u8; 29] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28];
pub const PROJECT_NAME: [&str; 29] = ["魔方解析1h", "魔方解析2h", "魔方解析4h", "魔方解析0.5h", "舰装解析1h", "舰装解析2h", "舰装解析4h", "舰装解析0.5h", "金船定向2.5h", "金船定向5h", "金船定向8h", "金船定向0.5h", "彩船定向2.5h", "彩船定向5h", "彩船定向8h", "彩船定向0.5h", "资金募集1.5h", "资金募集2.5h", "资金募集4h", "蓝试验品募集2h", "紫试验品募集2h", "紫数据收集4h", "金数据收集4h", "基础研究6h", "基础研究8h", "基础研究12h", "研究委托3h", "研究委托4h", "研究委托6h"];

pub const NONE_DIRECT_BLP: [u8; 2] = [3, 4];

fn is_direct_blp(d: u8) -> bool {
    return NONE_DIRECT_BLP.contains(&d);
}

fn cvt_str2u8(proj_type: &str) -> u8 {
    let mut k: u8 = 0;
    for i in PROJECT_TYPE {
        if i == proj_type {
            return k
        }
        k += 1;
    }
    panic!("Invalid project type: {}", proj_type);
}

fn is_ssr(proj_type: u8) -> bool {
    proj_type == 3
}

fn is_ur(proj_type: u8) -> bool {
    proj_type == 4
}

fn is_data_collection(proj_type: u8) -> bool {
    proj_type == 8 || proj_type == 9
}

fn is_research_assignment(proj_type: u8) -> bool {
    proj_type == 11
}

#[derive(Debug, Clone, Copy)]
pub struct ResearchData {
    pub research_id: u8,
    pub proj_type: u8,
    pub time: f64,
    pub rate: f64,
    pub doubloon: u16,
    pub cube: u8,
    pub super_rare_blp: f64,
    pub ultra_rare_blp: f64,
    pub ultra_rare_equip: f64,
    pub cognitive_chips: f64
}


impl ResearchData {
    pub fn empty() -> ResearchData {
        ResearchData {
            research_id: 0,
            proj_type: 0,
            time: 0.0,
            rate: 0.0,
            doubloon: 0,
            cube: 0,
            super_rare_blp: 0.0,
            ultra_rare_blp: 0.0,
            ultra_rare_equip: 0.0,
            cognitive_chips: 0.0
        }
    }
    pub fn new(research_id: u8, proj_type: &str, time: f64, rate: f64, doubloon: u16, cube: u8, super_rare_blp: f64, ultra_rare_blp: f64, ultra_rare_equip: f64, cognitive_chips: f64) -> ResearchData {
        ResearchData { research_id, proj_type: cvt_str2u8(proj_type), time, rate, doubloon, cube, super_rare_blp, ultra_rare_blp, ultra_rare_equip, cognitive_chips }
    }
    
    /// 是否为**定向蓝图**
    pub fn is_direct_blp(&self) -> bool {
        return is_direct_blp(self.proj_type);
    }

    pub fn is_data_collection(&self) -> bool {
        return is_data_collection(self.proj_type);
    }

    pub fn is_research_assignment(&self) -> bool {
        return is_research_assignment(self.proj_type);
    }
}


#[derive(Debug, Clone, Copy)]
pub struct ResearchDataCollection {
    pub data: [ResearchData; 29],
    pub len: usize,
}


impl ResearchDataCollection {
    
    pub fn empty() -> ResearchDataCollection {
        ResearchDataCollection {
            data: [ResearchData::empty(); 29],
            len: 29
        }
    }

    
    pub fn load() -> ResearchDataCollection {
        let m = [
        ResearchData::new(0, "魔方解析", 1.0, 0.116201788833198, 0, 3, 2.77, 0.66, 0.0, 40.0),
        ResearchData::new(1, "魔方解析", 2.0, 0.0726765528249297, 0, 6, 2.03, 0.48, 0.0, 30.16),
        ResearchData::new(2, "魔方解析", 4.0, 0.0112721787344704, 0, 10, 1.39, 0.33, 0.0, 20.77),
        ResearchData::new(3, "心智补全", 0.5, 0.000857391671078282, 8000, 3, 9.88, 2.34, 0.0, 194.49),
        ResearchData::new(4, "舰装解析", 1.0, 0.0924217786618207, 0, 0, 0.0, 0.0, 0.042, 0.0),
        ResearchData::new(5, "舰装解析", 2.0, 0.0448869757211571, 0, 0, 0.0, 0.0, 0.042, 0.0),
        ResearchData::new(6, "舰装解析", 4.0, 0.0229982707065704, 0, 0, 0.0, 0.0, 0.042, 0.0),
        ResearchData::new(7, "舰装解析", 0.5, 0.00900261254632197, 5000, 0, 0.0, 0.0, 0.693, 0.0),
        ResearchData::new(8, "金船定向", 2.5, 0.119338247215807, 3000, 0, 0.91, 0.0, 0.016, 0.0),
        ResearchData::new(9, "金船定向", 5.0, 0.0761934876473689, 5000, 0, 0.75, 0.0, 0.011, 0.0),
        ResearchData::new(10, "金船定向", 8.0, 0.012284819566333, 8000, 0, 0.75, 0.0, 0.011, 0.0),
        ResearchData::new(11, "金船定向", 0.5, 0.00179441184676775, 5000, 5, 18.0, 0.0, 0.25, 0.0),
        ResearchData::new(12, "彩船定向", 2.5, 0.119338247215807, 3000, 0, 0.0, 0.61, 0.016, 0.0),
        ResearchData::new(13, "彩船定向", 5.0, 0.0761934876473689, 5000, 0, 0.0, 0.51, 0.011, 0.0),
        ResearchData::new(14, "彩船定向", 8.0, 0.012284819566333, 8000, 0, 0.0, 0.51, 0.011, 0.0),
        ResearchData::new(15, "彩船定向", 0.5, 0.00179441184676775, 5000, 5, 0.0, 11.77, 0.25, 0.0),
        ResearchData::new(16, "资金募集", 1.5, 0.0998104774743483, 1500, 0, 0.55, 0.13, 0.018, 0.0),
        ResearchData::new(17, "资金募集", 2.5, 0.0650861147956779, 3000, 0, 0.46, 0.11, 0.014, 0.0),
        ResearchData::new(18, "资金募集", 4.0, 0.050838282614524, 6000, 0, 0.52, 0.12, 0.03, 0.0),
        ResearchData::new(19, "试验品募集蓝", 2.0, 0.0182573991135493, 0, 0, 0.0, 0.0, 0.019, 0.0),
        ResearchData::new(20, "试验品募集紫", 2.0, 0.0134913101184377, 0, 0, 0.0, 0.0, 0.025, 0.0),
        ResearchData::new(21, "数据收集紫", 4.0, 0.0171226160194751, 0, 0, 0.2, 0.05, 0.015, 0.0),
        ResearchData::new(22, "数据收集金", 4.0, 0.00716174219371271, 0, 0, 0.2, 0.05, 0.018, 0.0),
        ResearchData::new(23, "基础研究", 6.0, 0.0479130639720217, 0, 0, 0.0, 0.0, 0.011, 0.0),
        ResearchData::new(24, "基础研究", 8.0, 0.0342200146368597, 0, 0, 0.06, 0.014, 0.011, 0.0),
        ResearchData::new(25, "基础研究", 12.0, 0.00857391671078283, 0, 0, 0.06, 0.014, 0.011, 0.0),
        ResearchData::new(26, "研究委托", 3.0, 0.0174504422466521, 0, 0, 0.0, 0.0, 0.012, 0.0),
        ResearchData::new(27, "研究委托", 4.0, 0.0139200059539768, 0, 0, 0.0, 0.0, 0.012, 0.0),
        ResearchData::new(28, "研究委托", 6.0, 0.00887652553586928, 0, 0, 0.0, 0.0, 0.012, 0.0),
        ];
        ResearchDataCollection { data: m, len: 29 }
        // ResearchDataCollection{ data: m, len, data_ptr: v }
    }

    pub fn get_data_by_id(&self, id: usize) -> Option<&ResearchData> {
        self.data.get(id)
    }
}
