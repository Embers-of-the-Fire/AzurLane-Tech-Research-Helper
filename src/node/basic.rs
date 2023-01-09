//! Basic Node object with basic data. Contains the normal(`u8`) data array, the binary(`bool`) data array, and the pointer to the source data(`dataset::ResearchDataCollection`).

use std::rc::Rc;

use crate::dataset;

use super::error;


/// _Here `processed_data` is in `[0, 1]`, referring to the actual data_
#[derive(Debug)]
pub struct BasicMeta {
    pub data: [u8; 29],
    pub source_data_ptr: Rc<dataset::ResearchDataCollection>
}

impl Clone for BasicMeta {
    fn clone(&self) -> Self {
        BasicMeta { data: self.data.clone(), source_data_ptr: Rc::clone(&self.source_data_ptr) }
    }

    fn clone_from(&mut self, source: &Self) {
        self.data = source.data.clone();
        self.source_data_ptr = Rc::clone(&source.source_data_ptr);
    }
}

impl BasicMeta {
    pub fn new(data: [u8; 29], source_data_ptr: Rc<dataset::ResearchDataCollection>) -> Result<BasicMeta, error::NodeError> {
        Ok(BasicMeta { data, source_data_ptr })
    }
}
