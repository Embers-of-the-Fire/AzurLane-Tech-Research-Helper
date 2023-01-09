//! This file contains nodes focus on the __product__ of a project.

use std::rc::Rc;

use crate::dataset;

use super::rate;

/// Here `select_rate` refers to the actual select rate, see `crate::node::rate::RateNode::actual_rate`.
#[derive(Debug, Clone)]
pub struct ProductNode {
    pub id: u8,
    pub data_ptr: Rc<dataset::ResearchDataCollection>,
    pub select_rate: f64,
    pub time: f64,
    pub doubloon: f64,
    pub cube: f64,
    pub is_direct: bool,
    pub super_rare_blp: f64,
    pub ultra_rare_blp: f64,
    pub ultra_rare_equip: f64,
    pub cognitive_chips: f64,
}

impl ProductNode {
    /// Create a new `ProductNode` from the given research id. All other fields are filled with `0.0_f64`.
    pub fn new(id: u8, data_ptr: Rc<dataset::ResearchDataCollection>) -> ProductNode {
        ProductNode {
            id,
            data_ptr: Rc::clone(&data_ptr),
            select_rate: 0.0,
            doubloon: 0.0,
            cube: 0.0,
            super_rare_blp: 0.0,
            ultra_rare_blp: 0.0,
            ultra_rare_equip: 0.0,
            cognitive_chips: 0.0,
            time: 0.0,
            is_direct: match data_ptr.get_data_by_id(id as usize){
                Some(v) => v.is_direct_blp(),
                None => {println!("{}", id);Option::None.unwrap()}
            }
        }
    }

    /// Set the `select_rate`.
    pub fn set_rate(&mut self, rate: f64) -> &mut Self {
        self.select_rate = rate;
        self
    }

    /// Generate the real data relative to the actual select rate.
    pub fn generate(&mut self) -> &mut Self {
        if self.select_rate == 0.0 {
            return self;
        };
        let rate = self.select_rate;
        let dat = self.data_ptr.get_data_by_id(self.id as usize).unwrap();
        self.doubloon = dat.doubloon as f64 * rate;
        self.cube = dat.cube as f64 * rate;
        self.super_rare_blp = dat.super_rare_blp * rate * dat.time;
        self.ultra_rare_blp = dat.ultra_rare_blp * rate * dat.time;
        self.ultra_rare_equip = dat.ultra_rare_equip * rate * dat.time;
        self.cognitive_chips = dat.cognitive_chips * rate * dat.time;
        self.time = rate * dat.time;
        self
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ProductAverage {
    pub average_time: f64,
    pub doubloon: f64,
    pub cube: f64,
    pub super_rare_blp: f64,
    pub ultra_rare_blp: f64,
    pub ultra_rare_equip: f64,
    pub cognitive_chips: f64,
    pub indirect_ssr_blp: f64,
    pub indirect_ur_blp: f64,
}

impl ProductAverage {
    /// Create an empty `ProductAverage` with all fields set to `0.0_f64`.
    pub fn empty() -> ProductAverage {
        ProductAverage {
            average_time: 0.0,
            doubloon: 0.0,
            cube: 0.0,
            super_rare_blp: 0.0,
            ultra_rare_blp: 0.0,
            ultra_rare_equip: 0.0,
            cognitive_chips: 0.0,
            indirect_ssr_blp: 0.0,
            indirect_ur_blp: 0.0,
        }
    }

    /// Create a new `ProductAverage` from the given data.
    pub fn new(
        average_time: f64,
        doubloon: f64,
        cube: f64,
        super_rare_blp: f64,
        ultra_rare_blp: f64,
        ultra_rare_equip: f64,
        cognitive_chips: f64,
        indirect_ssr_blp: f64,
        indirect_ur_blp: f64,
    ) -> ProductAverage {
        ProductAverage {
            average_time,
            doubloon,
            cube,
            super_rare_blp,
            ultra_rare_blp,
            ultra_rare_equip,
            cognitive_chips,
            indirect_ssr_blp,
            indirect_ur_blp,
        }
    }

    /// Create a new `ProductAverage` from the given `ProductNode`s.
    pub fn from(prodns: &[ProductNode; 29]) -> ProductAverage {
        let mut pa = ProductAverage::empty();
        for i in prodns {
            pa.average_time += i.time;
            pa.doubloon += i.doubloon;
            pa.cube += i.cube;
            if i.is_direct {
                pa.super_rare_blp += i.super_rare_blp;
                pa.ultra_rare_blp += i.ultra_rare_blp;
            } else {
                pa.indirect_ssr_blp += i.super_rare_blp;
                pa.indirect_ur_blp += i.ultra_rare_blp;
            }
            pa.ultra_rare_equip += i.ultra_rare_equip;
            pa.cognitive_chips += i.cognitive_chips;
        }
        pa
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ProductPerDay {
    pub research_time: f64,
    pub doubloon: f64,
    pub cube: f64,
    pub super_rare_blp: f64,
    pub ultra_rare_blp: f64,
    pub ultra_rare_equip: f64,
    pub cognitive_chips: f64,
    pub indirect_ssr_blp: f64,
    pub indirect_ur_blp: f64,
}

impl ProductPerDay {
    /// Create an empty `ProductPerDay` with all fields set to `0.0_f64`.
    pub fn empty() -> ProductPerDay {
        ProductPerDay {
            research_time: 0.0,
            doubloon: 0.0,
            cube: 0.0,
            super_rare_blp: 0.0,
            ultra_rare_blp: 0.0,
            ultra_rare_equip: 0.0,
            cognitive_chips: 0.0,
            indirect_ssr_blp: 0.0,
            indirect_ur_blp: 0.0,
        }
    }

    /// Create a new `ProductPerDay` from the given data.
    pub fn new(
        research_time: f64,
        doubloon: f64,
        cube: f64,
        super_rare_blp: f64,
        ultra_rare_blp: f64,
        ultra_rare_equip: f64,
        cognitive_chips: f64,
        indirect_ssr_blp: f64,
        indirect_ur_blp: f64,
    ) -> ProductPerDay {
        ProductPerDay {
            research_time,
            doubloon,
            cube,
            super_rare_blp,
            ultra_rare_blp,
            ultra_rare_equip,
            cognitive_chips,
            indirect_ssr_blp,
            indirect_ur_blp,
        }
    }

    /// Create a new `ProductAverage` from the given `ProductAverage`.
    pub fn from(pa: &ProductAverage) -> ProductPerDay {
        let tms = 24.0 / pa.average_time;
        ProductPerDay::new(
            tms,
            pa.doubloon * tms,
            pa.cube * tms,
            pa.super_rare_blp * tms,
            pa.ultra_rare_blp * tms,
            pa.ultra_rare_equip * tms,
            pa.cognitive_chips * tms,
            pa.indirect_ssr_blp * tms,
            pa.indirect_ur_blp * tms,
        )
    }
}

#[derive(Debug, Clone)]
pub struct ProductMeta {
    pub nodes: [ProductNode; 29],
    pub average: ProductAverage,
    pub per_day: ProductPerDay,
}

impl ProductMeta {
    pub fn new(data: [u8; 29], data_ptr: Rc<dataset::ResearchDataCollection>) -> ProductMeta {
        ProductMeta {
            nodes: [
                ProductNode::new(data[0], Rc::clone(&data_ptr)),
                ProductNode::new(data[1], Rc::clone(&data_ptr)),
                ProductNode::new(data[2], Rc::clone(&data_ptr)),
                ProductNode::new(data[3], Rc::clone(&data_ptr)),
                ProductNode::new(data[4], Rc::clone(&data_ptr)),
                ProductNode::new(data[5], Rc::clone(&data_ptr)),
                ProductNode::new(data[6], Rc::clone(&data_ptr)),
                ProductNode::new(data[7], Rc::clone(&data_ptr)),
                ProductNode::new(data[8], Rc::clone(&data_ptr)),
                ProductNode::new(data[9], Rc::clone(&data_ptr)),
                ProductNode::new(data[10], Rc::clone(&data_ptr)),
                ProductNode::new(data[11], Rc::clone(&data_ptr)),
                ProductNode::new(data[12], Rc::clone(&data_ptr)),
                ProductNode::new(data[13], Rc::clone(&data_ptr)),
                ProductNode::new(data[14], Rc::clone(&data_ptr)),
                ProductNode::new(data[15], Rc::clone(&data_ptr)),
                ProductNode::new(data[16], Rc::clone(&data_ptr)),
                ProductNode::new(data[17], Rc::clone(&data_ptr)),
                ProductNode::new(data[18], Rc::clone(&data_ptr)),
                ProductNode::new(data[19], Rc::clone(&data_ptr)),
                ProductNode::new(data[20], Rc::clone(&data_ptr)),
                ProductNode::new(data[21], Rc::clone(&data_ptr)),
                ProductNode::new(data[22], Rc::clone(&data_ptr)),
                ProductNode::new(data[23], Rc::clone(&data_ptr)),
                ProductNode::new(data[24], Rc::clone(&data_ptr)),
                ProductNode::new(data[25], Rc::clone(&data_ptr)),
                ProductNode::new(data[26], Rc::clone(&data_ptr)),
                ProductNode::new(data[27], Rc::clone(&data_ptr)),
                ProductNode::new(data[28], Rc::clone(&data_ptr)),
            ],
            average: ProductAverage::empty(),
            per_day: ProductPerDay::empty(),
        }
    }

    pub fn produce(&mut self, rates: &rate::RateMeta) -> &mut Self {
        for i in 0..29 {
            let rate = rates.nodes[i];
            self.nodes[i].set_rate(rate.actual_rate).generate();
        };
        self.average = ProductAverage::from(&self.nodes);
        self.per_day = ProductPerDay::from(&self.average);
        self
    }
}
