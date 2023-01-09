//! This file is the meta data container for `basic`, `product` and `rate`, as an interface to any kind of solutions.

use std::rc::Rc;

use crate::{dataset, value, strict_check_larger, strict_check_smaller};
use crate::enum_val::Strict;

use crate::dataset::PROJECT_ID;

use super::{product, rate};

#[derive(Debug, Clone, Copy)]
pub enum PerformanceMode {
    PureIncome,
    CostPerformance,
}

#[derive(Clone, Debug, Copy)]
pub struct Metaperformance {
    pub cost: f64,
    pub income: f64,
    pub time_per_research: f64,
    pub mode: PerformanceMode,
    pub pure_income: f64,
    pub cost_performance: f64,
    pub modified_cost: f64,
    pub modified_income: f64,
    pub modified_pure_income: f64,
    pub modified_cost_performance: f64,
}

impl PartialEq for Metaperformance {
    fn eq(&self, other: &Self) -> bool {
        self.cost_performance == other.cost_performance
    }

    fn ne(&self, other: &Self) -> bool {
        self.cost_performance != other.cost_performance
    }
}

impl PartialOrd for Metaperformance {
    fn ge(&self, other: &Self) -> bool {
        self.cost_performance >= other.cost_performance
    }

    fn gt(&self, other: &Self) -> bool {
        self.cost_performance > other.cost_performance
    }

    fn le(&self, other: &Self) -> bool {
        self.cost_performance <= other.cost_performance
    }

    fn lt(&self, other: &Self) -> bool {
        self.cost_performance < other.cost_performance
    }

    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.cost_performance.partial_cmp(&other.cost_performance)
    }
}

impl Metaperformance {
    /// Create an empty `Metaperformance`. Field `time_per_research` is filled with `1.0_f64`, and the rest are filled with `0.0_f64`.
    pub fn empty(mode: PerformanceMode) -> Metaperformance {
        Metaperformance {
            cost: 0.0,
            income: 0.0,
            time_per_research: 1.0,
            mode,
            cost_performance: 0.0,
            pure_income: 0.0,
            modified_pure_income: 0.0,
            modified_cost_performance: 0.0,
            modified_cost: 0.0,
            modified_income: 0.0,
        }
    }

    pub fn fresh(
        &mut self,
        ppd: &product::ProductPerDay,
        resv: &value::resource_value::ResourceValue,
        rest: &value::restriction::ResourceRestriction,
        mode: PerformanceMode,
    ) -> &mut Self {
        self.cost = ppd.doubloon * resv.doubloon + ppd.cube * resv.cube;
        self.income = ppd.super_rare_blp * resv.ssr_blp
            + ppd.ultra_rare_blp * resv.ur_blp
            + ppd.indirect_ssr_blp * resv.indirect_ssr_blp
            + ppd.indirect_ur_blp * resv.indirect_ur_blp
            + ppd.ultra_rare_equip * resv.ur_equip
            + ppd.cognitive_chips * resv.cogn_chip;
        self.time_per_research = 24.0 / ppd.research_time;
        self.mode = mode;
        self.cost_performance = self.income / self.cost;
        self.pure_income = self.income - self.cost;
        let mut ulim = false;
        let mut mcost = 0.0;
        let mut mincome = 0.0;
        mcost += strict_check_larger!(ppd.doubloon * resv.doubloon, rest.doubloon, ppd.doubloon, ulim);
        mcost += strict_check_larger!(ppd.cube * resv.cube, rest.cube, ppd.cube, ulim);
        mincome += strict_check_smaller!(ppd.super_rare_blp * resv.ssr_blp, rest.ssr_blp, ppd.super_rare_blp + ppd.indirect_ssr_blp, ulim);
        mincome += strict_check_smaller!(ppd.ultra_rare_blp * resv.ur_blp, rest.ur_blp, ppd.ultra_rare_blp + ppd.indirect_ur_blp, ulim);
        mincome += strict_check_smaller!(ppd.ultra_rare_equip * resv.ur_equip, rest.ur_equip, ppd.ultra_rare_equip, ulim);
        mincome += ppd.indirect_ssr_blp * resv.indirect_ssr_blp;
        mincome += ppd.indirect_ur_blp * resv.indirect_ur_blp;
        if ulim == true { mcost = 0.0; mincome = 0.0; };
        self.modified_cost = mcost;
        self.modified_income = mincome;
        self.modified_cost_performance = self.modified_income / self.modified_cost;
        self.modified_pure_income = self.modified_income - self.modified_cost;
        self
    }

    pub fn get_unmodified(&self) -> f64 {
        match self.mode {
            PerformanceMode::PureIncome => self.pure_income,
            PerformanceMode::CostPerformance => self.cost_performance,
        }
    }

    pub fn get_modified(&self) -> f64 {
        match self.mode {
            PerformanceMode::PureIncome => self.modified_pure_income,
            PerformanceMode::CostPerformance => self.modified_cost_performance,
        }
    }
}

#[derive(Debug, Clone)]
pub struct MetaData {
    pub product: product::ProductMeta,
    pub rate: rate::RateMeta,
    pub performance: Metaperformance,
    pub mode: PerformanceMode,
}

impl MetaData {
    pub fn empty(ptr: Rc<dataset::ResearchDataCollection>, mode: PerformanceMode) -> MetaData {
        MetaData {
            product: product::ProductMeta::new(PROJECT_ID, Rc::clone(&ptr)),
            rate: rate::RateMeta::new(PROJECT_ID, Rc::clone(&ptr), 1.0, &1),
            performance: Metaperformance::empty(mode),
            mode: PerformanceMode::PureIncome,
        }
    }

    
    pub fn new(
        data: [u8; 29],
        data_ptr: Rc<dataset::ResearchDataCollection>,
        resource_value: &value::resource_value::ResourceValue,
        restriction: &value::restriction::ResourceRestriction,
        mode: PerformanceMode,
    ) -> MetaData {
        MetaData {
            product: product::ProductMeta::new(data.clone(), Rc::clone(&data_ptr)),
            rate: rate::RateMeta::new(
                data.clone(),
                Rc::clone(&data_ptr),
                resource_value.rate_factor,
                &restriction.select,
            ),
            performance: Metaperformance::empty(mode),
            mode,
        }
    }

    
    pub fn generate(&mut self) -> &mut Self {
        self.rate.generate();
        self.product.produce(&self.rate);
        self
    }

    pub fn cost(&mut self, resource_value: &value::resource_value::ResourceValue, rest: &value::restriction::ResourceRestriction) -> &mut Self {
        self.performance
            .fresh(&self.product.per_day, resource_value, rest, self.mode);
        self
    }
}
