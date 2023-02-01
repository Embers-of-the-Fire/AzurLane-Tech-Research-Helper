

use crate::*;

#[derive(Debug, Clone, Copy)]
pub struct ActualRatio {
    pub super_rare_5_blp: f64,
    pub ultra_rare_5_blp: f64,
    pub ssr_blp_direct: f64,
    pub ur_blp_direct: f64,
    pub refresh: f64, // '数据表'!AF34
}

impl ActualRatio {
    pub fn new() -> ActualRatio {
        ActualRatio { super_rare_5_blp: 0.0, ultra_rare_5_blp: 0.0, refresh: 0.0, ssr_blp_direct: 0.0, ur_blp_direct: 0.0 }
    }

    pub fn from(f: &restriction::Restriction) -> ActualRatio {
        let fres_5_ssr = 3 - f.fni_5_super_r;
        let fres_5_ur = 2 - f.fni_5_ultra_r;
        let fres_5 = fres_5_ssr + fres_5_ur;
        let fres_f_ship = 10 - f.fni_f;
        let res_5_ssr = if fres_5 == 0 { 3 } else { fres_5_ssr };
        let res_5_ur = if fres_5 == 0 { 2 } else { fres_5_ur };
        let res_f_ship = if fres_f_ship == 0 { 10 } else { fres_f_ship };
        let ssr_blp_direct = res_5_ssr as f64 / 3.0;
        let ur_blp_direct = res_5_ur as f64 / 2.0;
        let all_ship = res_5_ssr + res_5_ur + res_f_ship;
        let super_rare_5_blp = (res_5_ssr as f64) / ((res_5_ssr + res_5_ur) as f64);
        let ultra_rare_5_blp = (res_5_ur as f64) / ((res_5_ssr + res_5_ur) as f64);
        let refresh = 4.0 / (all_ship as f64) * ((res_5_ssr + res_5_ur) as f64);
        ActualRatio { super_rare_5_blp, ultra_rare_5_blp, ssr_blp_direct, ur_blp_direct, refresh }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct ActualResearch {
    pub data: dataset::ResearchData,
    pub single_income: f64,
    pub cost: f64,
    pub refer_income: f64,
    pub tap_income: f64
}

impl ActualResearch {
    pub fn new(data: dataset::ResearchData) -> ActualResearch {
        ActualResearch { data, single_income: 0.0, cost: 0., refer_income: 0.0, tap_income: 0.0 }
    }

    pub fn generate_single_income(&mut self, r: &dataset::ReferenceValue, ar: &ActualRatio) {
        let single_income = if self.data.is_none_direct() {
            self.data.super_rare_blp * r.super_rare * ar.ssr_blp_direct
            + self.data.ultra_rare_blp * r.ultra_rare * ar.ur_blp_direct
            + self.data.ultra_rare_equip * r.ultra_equip
            + self.data.cognitive_chips * r.cong_chips
        } else {
            self.data.super_rare_blp * r.super_rare 
            + self.data.ultra_rare_blp * r.ultra_rare
            + self.data.ultra_rare_equip * r.ultra_equip
            + self.data.cognitive_chips * r.cong_chips
        } * self.data.time;
        self.single_income = single_income;
    }

    pub fn generate_cost(&mut self, r: &dataset::ReferenceValue)  {
        let cost = self.data.doubloon as f64 * r.doubloon
            + self.data.cube as f64 * r.cube
            + self.data.time * r.time_of_an_hour
            + r.time_ratio;
        self.cost = cost;
    }

    /// r: reference cost performance('数据表'!V3~31 均性价比)
    pub fn generate_refer(&mut self, r: &f64)  {
        self.refer_income = r * self.cost / 100.0;
    }

    pub fn generate_tap(&mut self, do_data_collection: &bool, do_research_assignment: &bool)  {
        if (dataset::is_data_collection(self.data.proj_type) && !*do_data_collection) || (dataset::is_research_assignment(self.data.proj_type) && !*do_research_assignment) {
            self.tap_income = self.single_income - self.refer_income * 10.0;
        } else {
            self.tap_income = self.single_income - self.refer_income;
        }
    }
}

#[derive(Debug, Clone)]
pub struct ActualResearches {
    pub data: Vec<ActualResearch>
}

impl ActualResearches {
    pub fn new() -> ActualResearches {
        ActualResearches { data: Vec::new() }
    }

    pub fn from(d: &dataset::Data) -> ActualResearches {
        let mut v: Vec<ActualResearch> = Vec::new();
        for i in &d.data {
            v.push(ActualResearch::new(i.clone()))
        };
        ActualResearches { data: v }
    }

    pub fn generate_single_income(&mut self, r: &dataset::ReferenceValue, ar: &ActualRatio)  {
        for i in self.data.iter_mut() {
            i.generate_single_income(&r, &ar);
        };
    }

    pub fn generate_cost(&mut self, r: &dataset::ReferenceValue)  {
        for i in self.data.iter_mut() {
            i.generate_cost(&r);
        };
    }

    pub fn generate_refer(&mut self, r: &f64)  {
        for i in self.data.iter_mut() {
            i.generate_refer(&r);
        };
    }

    pub fn generate_tap(&mut self, r: &restriction::Restriction)  {
        let ref do_data_collection = r.do_data_collection;
        let ref do_research_assignment = r.do_research_assignment;
        for i in self.data.iter_mut() {
            i.generate_tap(do_data_collection, do_research_assignment);
        };
    }

    pub fn sort(&mut self) {
        /*
        let mut v: Vec<(f64, i8)> = Vec::new();
        let mut k: i8 = 0;
        for i in &self.data {
            v.push((i.tap_income, k));
            k += 1;
        };
        v.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());
        let mut fv: Vec<ActualResearch> =Vec::new();
        for i in v {
            fv.push(self.data[i.1 as usize])
        }
        self.data = fv;
        */
        self.data.sort_by(|a, b| b.tap_income.partial_cmp(&a.tap_income).unwrap());
    }
}