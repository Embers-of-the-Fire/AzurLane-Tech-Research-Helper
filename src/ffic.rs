use core::slice;

use crate::calc::{RefProjContent, RefreshProjects, ResearchResult, ResultPerDay, ResultPlan};

#[derive(Debug, Clone)]
#[repr(C)]
pub struct ResultPlanRepV {
    pub result_average: ResearchResult,
    pub result: ResultPerDay,
    pub projects: RefreshProjectsRepV,
}

impl ResultPlanRepV {
    pub fn from(r: ResultPlan) -> ResultPlanRepV {
        ResultPlanRepV { result_average: r.result_average, result: r.result, projects: RefreshProjectsRepV::from(r.projects) }
    }

    pub fn to_rust(&self) -> ResultPlan {
        ResultPlan { result_average: self.result_average, result: self.result, projects: self.projects.to_rust() }
    }
}

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct RefreshProjectsRepV {
    pub data: VecRep<RefProjContent>,
    pub average_time: f64,
    pub cost_performance: f64,
    pub cost_refresh_performance: f64,
    pub research_time_per_day: f64,
    pub total_select_rate: f64,
    pub refresh_rate: f64,
    pub refresh_fail: f64,
}

impl RefreshProjectsRepV {
    pub fn from(r: RefreshProjects) -> RefreshProjectsRepV {
        RefreshProjectsRepV {
            data: VecRep::from(r.data),
            average_time: r.average_time,
            cost_performance: r.cost_performance,
            cost_refresh_performance: r.cost_refresh_performance,
            research_time_per_day: r.research_time_per_day,
            total_select_rate: r.total_select_rate,
            refresh_rate: r.refresh_rate,
            refresh_fail: r.refresh_fail,
        }
    }

    pub fn to_rust(&self) -> RefreshProjects {
        RefreshProjects {
            data: self.data.to_rust(),
            average_time: self.average_time,
            cost_performance: self.cost_performance,
            cost_refresh_performance: self.cost_refresh_performance,
            research_time_per_day: self.research_time_per_day,
            total_select_rate: self.total_select_rate,
            refresh_rate: self.refresh_rate,
            refresh_fail: self.refresh_fail,
        }
    }
}

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct VecRep<T> {
    pub ptr: *const T,
    pub size: usize,
}

impl<T> VecRep<T>
where
    T: Clone,
{
    pub fn from(v: Vec<T>) -> VecRep<T> {
        let ref ptr = v[0];
        let raw_ptr = ptr as *const T;
        let len = v.len();
        VecRep {
            ptr: raw_ptr,
            size: len,
        }
    }

    pub fn to_rust(&self) -> Vec<T> {
        Vec::from(unsafe { slice::from_raw_parts(self.ptr, self.size) })
    }
}
