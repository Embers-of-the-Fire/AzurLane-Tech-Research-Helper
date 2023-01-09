//! This file contains data nodes which are focused on the __rate__ part of a project solution.

use std::rc::Rc;

use crate::dataset;

#[derive(Debug, Clone, Copy)]
pub struct RateNode {
    pub id: u8,
    pub rate: f64,
    pub is_direct: bool,
    pub direct_factor: f64,
    pub select: bool,
    pub select_rate: f64,
    pub rate1: f64,
    pub rate2: f64,
    pub rate3: f64,
    pub actual_rate: f64,
}

impl RateNode {
    /// Create an empty `RateNode` object.
    pub fn empty() -> RateNode {
        Self::new(0, 0.0, false, 0.0, false)
    }

    /// Create a basic `RateNode` object.
    pub fn new(id: u8, rate: f64, is_direct: bool, direct_factor: f64, select: bool) -> RateNode {
        RateNode { id, rate, is_direct, direct_factor, select, rate1: 0.0, rate2: 0.0, rate3: 0.0, actual_rate: 0.0, select_rate: 0.0 }
    }

    /// Generate the basic select rate(`RateNode::select_rate`) using `remain` rate and `remain_all` rate, and return the next `remain`&`remain_all` rate.
    pub fn base_rate(&mut self, remain_all: &f64, remain: &f64) -> (f64, f64) {
        let r1 = remain_all * self.rate;
        let r2 = r1 / 4.0 * if self.is_direct { self.direct_factor } else { 0.0 };
        let nhave = (1.0 - r1).powi(3) * (1.0 - r2).powi(2);
        let sr = (1.0 - nhave) * remain;
        self.select_rate = sr;
        if self.select { self.rate1 = sr };
        (remain_all - self.rate, remain * nhave)
    }

    /// Generate complete rate with sumed rate(`RateNode::rate1`) and refresh related rate, and return self.
    pub fn final_rate(&mut self, all_rate: &f64, refresh_failure: &f64) -> &mut Self {
        if self.select {
            self.rate2 = self.rate1 / all_rate;
            self.rate3 = 0.0;
        } else {
            self.rate3 = self.select_rate / (1.0 - all_rate);
            self.rate2 = 0.0;
        }
        self.actual_rate = self.rate2 * (1.0 - refresh_failure) + self.rate3 * refresh_failure;
        self
    }
}

/// # Meta data handler for `RateNode`
/// 
///  - `nodes`: Node array.
///  - `total_select_rate`: Total select rate of rate1(`RateNode::rate1`). Also see `RateNode::final_rate{param: all_rate: f64}`.
///  - `refresh_success`: The rate to use refresh successfully. Also see `RateNode::final_rate{param: refresh_success: f64}`.
///  - `refresh_failure`: The rate to use refresh but failed. Also see `RateNode::final_rate{param: refresh_failure: f64}`.
///  - `research_time_per_day`: The number of projects a day. Though this shouldn't be constructed here, but since the rate calculation needs it,
/// I still put it here, and the other will clone this from here.
#[derive(Debug, Clone)]
pub struct RateMeta {
    pub nodes: [RateNode; 29],
    pub data_ptr: Rc<dataset::ResearchDataCollection>,
    pub total_select_rate: f64,
    pub use_refresh: f64,
    pub refresh_failure: f64,
    pub research_time_per_day: f64,
}

impl RateMeta {
    /// Create a new `RateMeta` object with data list(`[u8; 29]`).
    /// 
    /// Here `direct_factor` is produced from the finished tech ship, which should be constructed from the input values just as `select_limit`.
    pub fn new(data: [u8; 29], data_ptr: Rc<dataset::ResearchDataCollection>, direct_factor: f64, select_limit: &u8) -> RateMeta {
        let mut rl = [RateNode::empty(); 29];
        for i in 0..data.len() {
            let dat = data_ptr.get_data_by_id(data[i] as usize).unwrap();
            rl[i] = RateNode::new(i as u8, dat.rate, dat.is_direct_blp(), direct_factor, select_limit > &(i as u8));
        };
        RateMeta {
            nodes: rl,
            data_ptr,
            total_select_rate: 0.0,
            use_refresh: 0.0,
            refresh_failure: 0.0,
            research_time_per_day: 0.0,
        }
    }

    /// This function will generate all rate data of the children nodes, as well as fill the `research_time_per_day`.
    pub fn generate(&mut self) -> &mut Self {
        let mut remain_all = 1.0;
        let mut remain = 1.0;
        let mut totalr = 0.0;
        let mut all_time = 0.0;
        for i in self.nodes.iter_mut() {
            (remain_all, remain) = i.base_rate(&remain_all, &remain);
            totalr += i.select_rate;
            all_time += i.rate * self.data_ptr.get_data_by_id(i.id as usize).unwrap().time;
        };
        self.research_time_per_day = 24.0 / all_time;
        self.use_refresh = 1.0 - totalr.powf(self.research_time_per_day);
        self.refresh_failure = (self.research_time_per_day * (1.0 - self.use_refresh) + self.use_refresh * totalr) / self.research_time_per_day;
        for i in self.nodes.iter_mut() {
            i.final_rate(&totalr, &self.refresh_failure);
        }
        self
    }
}
