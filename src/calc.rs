use std::ops::{Add, AddAssign};

use crate::{
    actual::{ActualRatio, ActualResearches},
    *,
};

#[derive(Debug, Copy, Clone)]
pub struct RefreshProject {
    pub rate: f64,
    pub r1: f64,            // 刷出率1
    pub r2: f64,            // 刷出率2
    pub false_: f64,        // 无
    pub true_: f64,         // 有
    pub select_rate: f64,   // 选取率
    pub select: bool,       //是否选取
    pub sr1: f64,           // 选取率1
    pub sr2: f64,           // 选取率2
    pub sr3: f64,           // 选取率3
    pub actual_select: f64, // 实际选取
}

impl RefreshProject {
    pub fn new() -> RefreshProject {
        RefreshProject {
            rate: 0.0,
            r1: 0.0,
            r2: 0.0,
            false_: 0.0,
            true_: 0.0,
            select_rate: 0.0,
            select: false,
            sr1: 0.0,
            sr2: 0.0,
            sr3: 0.0,
            actual_select: 0.0,
        }
    }

    /// return:
    ///  - :return: RefreshProject: RefreshProject built from the given args
    ///  - :return: f64: Remaining selection rate('刷新情况表'C2~30)
    ///  - :return: f64: Remaining selection rate('刷新情况表'H2~30)
    pub fn from(
        d: &dataset::ResearchData,
        acl: &actual::ActualRatio,
        remain: &f64,
        remain_all: &f64,
        if_select: bool,
    ) -> (RefreshProject, f64, f64) {
        let rate = d.rate;
        let r1 = rate / remain;
        let r2 = r1 / 4.0 * if d.is_none_direct() { acl.refresh } else { 1.0 };
        let false_ = (1.0 - r1).powi(3) * (1.0 - r2).powi(2);
        let true_ = 1.0 - false_;
        let select_rate = remain_all * true_;
        let select = if_select;
        let sr1 = if if_select { select_rate } else { 0.0 };
        // alert(format!("{}\n{}\n{}\n{}\n{}\n{}\n{}", rate, r1, r2, true_, remain_all - rate, remain * false_, d.research_id).as_str());
        // alert(format!("{}\n{}\n{}\n{}", true_, remain_all, select_rate, d.research_id).as_str());
        return (
            RefreshProject {
                rate,
                r1,
                r2,
                false_,
                true_,
                select_rate,
                select,
                sr1,
                sr2: 0.0,
                sr3: 0.0,
                actual_select: 0.0,
            },
            remain - rate,
            remain_all * false_,
        );
    }

    /// Generate RefreshProject.sr2 && RefreshProject.sr3
    pub fn generate_sr23(&mut self, total: f64)  {
        if self.select {
            self.sr2 = self.sr1 / total
        } else {
            self.sr3 = self.select_rate / (1.0 - total)
        };
    }

    pub fn generate_actual_select(&mut self, ref_fail: &f64)  {
        self.actual_select = self.sr2 * (1.0 - ref_fail) + ref_fail * self.sr3;
    }
}

#[derive(Debug, Copy, Clone)]
pub struct ProjectCost {
    pub doubloon: f64,
    pub cube: f64,
    pub time: f64,
}

impl Add for ProjectCost {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        ProjectCost {
            doubloon: self.doubloon + other.doubloon,
            cube: self.cube + other.cube,
            time: self.time + other.time,
        }
    }
}

impl AddAssign for ProjectCost {
    fn add_assign(&mut self, rhs: Self) {
        self.doubloon += rhs.doubloon;
        self.cube += rhs.cube;
        self.time += rhs.time;
    }
}

impl ProjectCost {
    pub fn new() -> Self {
        ProjectCost {
            doubloon: 0.0,
            cube: 0.0,
            time: 0.0,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct ProjectIncome {
    pub ssr_blp: f64,
    pub ur_blp: f64,
    pub ur_equip: f64,
    pub cogn_chips: f64,
}

impl Add for ProjectIncome {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        ProjectIncome {
            ssr_blp: self.ssr_blp + other.ssr_blp,
            ur_blp: self.ur_blp + other.ur_blp,
            cogn_chips: self.cogn_chips + other.cogn_chips,
            ur_equip: self.ur_equip + other.ur_equip,
        }
    }
}

impl AddAssign for ProjectIncome {
    fn add_assign(&mut self, rhs: Self) {
        self.ssr_blp += rhs.ssr_blp;
        self.ur_blp += rhs.ur_blp;
        self.ur_equip += rhs.ur_equip;
        self.cogn_chips += rhs.cogn_chips;
    }
}

impl ProjectIncome {
    pub fn new() -> Self {
        ProjectIncome {
            ssr_blp: 0.0,
            ur_blp: 0.0,
            ur_equip: 0.0,
            cogn_chips: 0.0,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct ProjectResult {
    pub cost: ProjectCost,
    pub income: ProjectIncome,
}

impl Add for ProjectResult {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        ProjectResult {
            cost: self.cost + rhs.cost,
            income: self.income + rhs.income,
        }
    }
}

impl AddAssign for ProjectResult {
    fn add_assign(&mut self, rhs: Self) {
        self.cost += rhs.cost;
        self.income += rhs.income;
    }
}

impl ProjectResult {
    pub fn new() -> ProjectResult {
        ProjectResult {
            cost: ProjectCost::new(),
            income: ProjectIncome::new(),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct RefProjContent {
    pub data: actual::ActualResearch,
    pub ref_proj: RefreshProject,
}

impl RefProjContent {
    pub fn new(data: actual::ActualResearch, ref_proj: RefreshProject) -> Self {
        RefProjContent { data, ref_proj }
    }
}

#[derive(Debug, Clone)]
#[repr(C)]
pub struct RefreshProjects {
    pub data: Vec<RefProjContent>,
    pub average_time: f64,
    pub cost_performance: f64,         // '刷新情况表'!G112
    pub cost_refresh_performance: f64, // '刷新情况表'!J112
    pub research_time_per_day: f64,    // '刷新情况表'!I32
    pub total_select_rate: f64,        // '刷新情况表'!K31
    pub refresh_rate: f64,             // '刷新情况表'!K33
    pub refresh_fail: f64,             // '刷新情况表'!K34
}

impl RefreshProjects {
    pub fn new() -> Self {
        RefreshProjects {
            data: Vec::new(),
            average_time: 0.0,
            cost_performance: 0.0,
            cost_refresh_performance: 0.0,
            research_time_per_day: 0.0,
            total_select_rate: 0.0, // '数据表'!K31
            refresh_fail: 0.0, // K34
            refresh_rate: 0.0, // K33
        }
    }

    pub fn from(ar: &ActualResearches, art: &ActualRatio, limit: i8) -> Self {
        let mut rp = RefreshProjects::new();
        let mut remain = 1.0;
        let mut remain_all = 1.0;
        let mut k = 0;
        for i in &ar.data {
            k += 1;
            let rp_tpl = RefreshProject::from(&i.data, &art, &remain, &remain_all, k <= limit);
            rp.data.push(RefProjContent::new(*i, rp_tpl.0));
            remain = rp_tpl.1;
            remain_all = rp_tpl.2;
        }
        rp
    }

    pub fn generate(&mut self) {
        let datv = &self.data;
        let mut time_ref_income: Vec<f64> = Vec::new();
        let mut time_ref_cost: Vec<f64> = Vec::new();
        let mut time_v: Vec<f64> = Vec::new();
        let mut rate_v: Vec<f64> = Vec::new();
        for i in datv {
            let rate = i.ref_proj.select_rate;
            // alert(format!("{}-{}", i.data.single_income, rate).as_str());
            time_ref_income.push(i.data.single_income * rate);
            time_ref_cost.push(i.data.cost * rate);
            time_v.push(i.data.data.time * rate);
            if i.ref_proj.select {
                rate_v.push(i.ref_proj.sr1)
            }
        };
        // alert(format!("{:?}", time_ref_income).as_str());
        // alert(format!("{:?}", time_ref_cost).as_str());
        // alert(format!("{:?}", time_v).as_str());
        // alert(format!("{:?}", rate_v).as_str());
        self.average_time = 0.0;
        for i in time_v {
            self.average_time += i;
        }
        self.cost_performance = time_ref_income.iter().sum::<f64>() / time_ref_cost.iter().sum::<f64>() * 100.0;
        self.research_time_per_day = 24.0 / self.average_time;
        // alert(format!("{:?}", rate_v).as_str());
        self.total_select_rate = rate_v.iter().sum();
        self.refresh_rate = 1.0 - self.total_select_rate.powf(self.research_time_per_day);
        self.refresh_fail = (self.research_time_per_day * (1.0 - self.total_select_rate)
            - self.refresh_rate * self.total_select_rate)
            / self.research_time_per_day;
        // alert(self.total_select_rate.to_string().as_str());
        for i in self.data.iter_mut() {
            i.ref_proj.generate_sr23(self.total_select_rate);
            i.ref_proj.generate_actual_select(&self.refresh_fail);
            // alert(i.ref_proj.select_rate.to_string().as_str());
        }
    }

    pub fn generate_refresh_perf(&mut self) {
        let datv = &self.data;
        let mut time_ref_income: Vec<f64> = Vec::new();
        let mut time_ref_cost: Vec<f64> = Vec::new();
        for i in datv {
            time_ref_income.push(i.data.single_income * i.ref_proj.actual_select);
            time_ref_cost.push(i.data.cost * i.ref_proj.actual_select);
        }
        let mut refc = 0.0;
        for i in time_ref_cost {
            refc += i;
        }
        let mut refi = 0.0;
        for i in time_ref_income {
            refi += i;
        }
        self.cost_refresh_performance = (refc / refi) * 100.0;
    }

    pub fn get_result(&self) -> ResearchResult {
        let mut r = ProjectResult::new();
        for i in &self.data {
            let rate = i.ref_proj.actual_select;
            let tm = i.data.data.time;
            r += ProjectResult {
                cost: ProjectCost {
                    doubloon: i.data.data.doubloon as f64 * rate,
                    cube: i.data.data.cube as f64 * rate,
                    time: tm * rate,
                },
                income: ProjectIncome {
                    ssr_blp: i.data.data.super_rare_blp * rate * tm,
                    ur_blp: i.data.data.ultra_rare_blp * rate * tm,
                    ur_equip: i.data.data.ultra_rare_equip * rate * tm,
                    cogn_chips: i.data.data.cognitive_chips * rate * tm,
                },
            }
        };
        ResearchResult::from(&r, self.cost_performance, self.cost_refresh_performance)
    }
}

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct ResearchResult {
    pub doubloon: f64,
    pub cube: f64,
    pub time: f64,
    pub ssr_blp: f64,
    pub ur_blp: f64,
    pub ur_equip: f64,
    pub cogn_chips: f64,
    pub cost_performance: f64,         // '刷新情况表'!G112
    pub cost_refresh_performance: f64, // '刷新情况表'!J112
}

impl ResearchResult {
    pub fn new() -> ResearchResult {
        ResearchResult {
            doubloon: 0.0,
            cube: 0.0,
            time: 0.0,
            ssr_blp: 0.0,
            ur_blp: 0.0,
            ur_equip: 0.0,
            cogn_chips: 0.0,
            cost_performance: 0.0,
            cost_refresh_performance: 0.0,
        }
    }

    pub fn from(f: &ProjectResult, cost_performance: f64, cost_refresh_performance: f64) -> ResearchResult {
        ResearchResult {
            doubloon: f.cost.doubloon,
            cube: f.cost.cube,
            time: f.cost.time,
            ssr_blp: f.income.ssr_blp,
            ur_blp: f.income.ur_blp,
            ur_equip: f.income.ur_equip,
            cogn_chips: f.income.cogn_chips,
            cost_performance,
            cost_refresh_performance,
        }
    }
}

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct ResultPerDay {
    pub doubloon: f64,
    pub cube: f64,
    pub research_per_day: f64,
    pub ssr_blp: f64,
    pub ur_blp: f64,
    pub ur_equip: f64,
    pub cogn_chips: f64,
    pub cost_performance: f64,         // '刷新情况表'!G112
    pub cost_refresh_performance: f64, // '刷新情况表'!J112
}

impl ResultPerDay {
    pub fn new() -> Self {
        ResultPerDay {
            doubloon: 0.0,
            cube: 0.0,
            research_per_day: 0.0,
            ssr_blp: 0.0,
            ur_blp: 0.0,
            ur_equip: 0.0,
            cogn_chips: 0.0,
            cost_performance: 0.0,
            cost_refresh_performance: 0.0,
        }
    }

    pub fn from(d: &ResearchResult) -> Self {
        let rtime = 24.0 / d.time;
        ResultPerDay {
            doubloon: d.doubloon * rtime,
            cube: d.cube * rtime,
            research_per_day: rtime,
            ssr_blp: d.ssr_blp * rtime,
            ur_blp: d.ur_blp * rtime,
            ur_equip: d.ur_equip * rtime,
            cogn_chips: d.cogn_chips * rtime,
            cost_performance: d.cost_performance,
            cost_refresh_performance: d.cost_refresh_performance,
        }
    }
}

#[derive(Debug, Clone)]
// #[repr(C)]
pub struct ResultPlan {
    pub result_average: ResearchResult,
    pub result: ResultPerDay,
    pub projects: RefreshProjects
}

impl ResultPlan {
    pub fn new() -> ResultPlan {
        ResultPlan { result_average: ResearchResult::new(), result: ResultPerDay::new(), projects: RefreshProjects::new() }
    }

    pub fn build(result_average: ResearchResult, result: ResultPerDay, projects: RefreshProjects) -> ResultPlan {
        let mut s = ResultPlan::new();
        s.result_average = result_average;
        s.result = result;
        s.projects = projects;
        s
    }
}

