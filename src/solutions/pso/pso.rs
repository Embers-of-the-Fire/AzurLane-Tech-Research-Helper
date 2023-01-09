use crate::{dataset, node::meta::{self, PerformanceMode}, value};

use super::pso_coef::*;
use std::{fmt::Debug, rc::Rc};

pub trait PsoSupport {
    fn generate(
        &mut self,
        resource_value: &value::resource_value::ResourceValue,
        rest: &value::restriction::ResourceRestriction,
    );
    fn get_performance(&self) -> f64;
    fn change_position(&mut self, new_position: [f64; 29], resv: &value::resource_value::ResourceValue, rest: &value::restriction::ResourceRestriction);
    fn get_position(&self) -> [f64; 29];
}

/// This structure focuses on the indeed PSO solution, using cost performace provider but does not care about what they actually mean.
#[derive(Debug, Clone)]
pub struct PsoNode<T>
where
    T: PsoSupport + Debug + Clone,
{
    pub position: [f64; 29],
    pub velocity: [f64; 29],
    pub best: [f64; 29],
    pub best_perf: f64,
    pub data: T,
}

impl<T> PsoNode<T>
where
    T: PsoSupport + Debug + Clone,
{
    pub fn new(data: T) -> PsoNode<T> {
        let pos = data.get_position();
        PsoNode {
            position: pos,
            velocity: [0.0; 29],
            data,
            best: pos,
            best_perf: 0.0,
        }
    }

    pub fn mv_pso(
        &mut self,
        c1: &f64,
        c2: &f64,
        omg: &f64,
        global: &[f64; 29],
        vmax: &f64,
        resv: &value::resource_value::ResourceValue,
        rest: &value::restriction::ResourceRestriction
    ) -> &mut Self {
        let mut vp = [0.0; 29];
        for i in 0..29 {
            let mut v = omg * self.velocity[i]
                + c1 * fastrand::f64() * (self.best[i] - self.position[i])
                + c2 * fastrand::f64() * (global[i] - self.position[i]);
            if &v >= vmax {
                v = *vmax / 2.0
            } else if &v <= &-vmax {
                v = -*vmax / 2.0
            };
            vp[i] = v;
            self.velocity[i] = v;
        }
        for i in 0..29 {
            let mut np = self.position[i] + vp[i];
            if np > 1.0 {
                np = 1.0
            } else if np < 0.0 {
                np = 0.0
            };
            self.position[i] = np;
        }
        self.data.change_position(self.position, resv, rest);
        self
    }

    pub fn set_best(&mut self, perf: f64) -> &mut Self {
        self.best_perf = perf;
        self.best = self.position;
        self
    }

    pub fn get_performance(
        &mut self,
        resource_value: &value::resource_value::ResourceValue,
        rest: &value::restriction::ResourceRestriction,
    ) -> f64 {
        self.data.generate(resource_value, rest);
        self.data.get_performance()
    }

    pub fn best_performance(&self) -> f64 {
        self.best_perf
    }
}

/// The performance calculation uses `index=id, value=index` to calculate the performance.
/// And in the case of PSO calculation, uses `index=index, value=id`.
///
/// Here `position` represents the former one while `transformed` represents the later one.
/// For example, `position = [1, 2, 0]` means that position 1 is the 2nd object, and position 3 is the 1st object.
/// And when turning to PSO, it will be transformed into an array liks `[2, 0, 1]`, which means that the 1st object is set at position 3, and the 2rd object is set at position 1.
#[derive(Debug, Clone)]
pub struct DataNode {
    pub data: meta::MetaData,
    pub position: [u8; 29],
    pub transformed: [u8; 29],
}

impl DataNode {
    pub fn new(position: [u8; 29], ptr: Rc<dataset::ResearchDataCollection>, mode: PerformanceMode) -> DataNode {
        DataNode {
            position,
            data: meta::MetaData::empty(ptr, mode),
            transformed: Self::into_transform(position),
        }
    }

    /// Process the `transformed` data and return the vaild version of data, in `position` mode.
    pub fn vaildary(pos: &[u8; 29]) -> [u8; 29] {
        let mut res = [0u8; 29];
        let mut k: [(u8, u8); 29] = [(0, 0); 29];
        for i in 0..29 {
            k[i] = (i as u8, pos[i]);
        }
        k.sort_by(|a, b| a.1.cmp(&b.1));
        for i in 0..29 {
            res[i] = k[i].0;
        }
        res
    }

    pub fn into_transform(pos: [u8; 29]) -> [u8; 29] {
        let mut res = [0u8; 29];
        for i in 0..29 {
            let p = i;
            let idx = pos[i];
            res[idx as usize] = p as u8;
        }
        res
    }

    pub fn from_transform(pos: [u8; 29]) -> [u8; 29] {
        let mut res = [0u8; 29];
        for i in 0..29 {
            let idx = i;
            let p = pos[i];
            res[p as usize] = idx as u8;
        }
        res
    }
}

impl PsoSupport for DataNode {
    fn get_performance(&self) -> f64 {
        self.data.performance.get_modified()
    }

    fn generate(
        &mut self,
        resource_value: &value::resource_value::ResourceValue,
        rest: &value::restriction::ResourceRestriction,
    ) {
        self.data.generate().cost(resource_value, rest);
    }

    fn get_position(&self) -> [f64; 29] {
        const STEP: f64 = 1.0 / 29.0;
        let mut res = [0.0_f64; 29];
        for i in 0..29 {
            let mut h = STEP / 2.0 + self.transformed[i] as f64 * STEP;
            if h >= 1.0 {
                h = 1.0;
            } else if h <= 0.0 {
                h = 0.0;
            }
            res[i] = h;
        }
        res
    }

    fn change_position(&mut self, new_position: [f64; 29], resv: &value::resource_value::ResourceValue, rest: &value::restriction::ResourceRestriction) {
        let mut res = [0u8; 29];
        for i in 0..29 {
            let mut h = (new_position[i] * 29.0) as u8;
            if h >= 29 {
                h = 28;
            }
            res[i] = h;
        }
        self.transformed = res;
        // println!("{:?}", res);
        self.position = Self::vaildary(&res);
        let dpr = Rc::clone(&self.data.rate.data_ptr);
        let md = self.data.mode.clone();
        self.data = meta::MetaData::new(self.position, dpr, resv, rest, md);
    }
}

#[derive(Debug, Clone)]
pub struct PsoHandler {
    pub nodes: Vec<PsoNode<DataNode>>,
    pub inertia: PsoInertia,
    pub acc1: f64,
    pub acc2: f64,
    pub generation: usize,
    pub global_best: PsoNode<DataNode>,
    pub resource_value: value::resource_value::ResourceValue,
    pub restriction: value::restriction::ResourceRestriction,
    pub data_ptr: Rc<dataset::ResearchDataCollection>,
    pub vmax: f64,
}

impl PsoHandler {
    pub fn new(
        node_amo: usize,
        generation: usize,
        resource_value: value::resource_value::ResourceValue,
        restriction: value::restriction::ResourceRestriction,
        acc1: f64,
        acc2: f64,
        ine_max: f64,
        ine_min: f64,
        data_ptr: Rc<dataset::ResearchDataCollection>,
        vmax: f64,
        mode: PerformanceMode,
    ) -> PsoHandler {
        let ine = PsoInertia::new(ine_max, ine_min, generation);
        let mut v: Vec<PsoNode<DataNode>> = Vec::new();
        let mut ps = dataset::PROJECT_ID.clone();
        for _ in 0..node_amo {
            fastrand::shuffle(&mut ps);
            v.push(PsoNode::new(DataNode::new(ps.clone(), Rc::clone(&data_ptr), mode)))
        };
        let gb = v[0].clone();
        PsoHandler {
            nodes: v,
            inertia: ine,
            acc1,
            acc2,
            generation,
            global_best: gb,
            resource_value,
            restriction,
            data_ptr: Rc::clone(&data_ptr),
            vmax,
        }
    }

    pub fn generate(&mut self) {
        for i in 0..self.generation {
            let ine = self.inertia.get(i).unwrap();
            let mut best_this = self.global_best.clone();
            for node in self.nodes.iter_mut() {
                node.mv_pso(&self.acc1, &self.acc2, &ine, &self.global_best.best, &self.vmax, &self.resource_value, &self.restriction);
                let perf = node.get_performance(&self.resource_value, &self.restriction);
                // println!("====\n{}, {}\n{:?}\n====", perf, node.best_performance(), node.data.position);
                if perf > node.best_performance() {
                    node.set_best(perf);
                }
                if perf > best_this.best_performance() {
                    best_this = node.clone();
                }
            }
            if best_this.best_performance() > self.global_best.best_performance() {
                self.global_best = best_this.clone();
            }
        }
    }
}
