use std::{env, fs};

use anyhow::Result;
use serde::{Deserialize, Serialize};
use tera::{Context, Tera};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
struct ResearchData {
    pub research_id: u8,
    pub proj_type: String,
    pub time: f64,
    pub rate: f64,
    pub doubloon: i16,
    pub cube: i8,
    pub super_rare_blp: f64,
    pub ultra_rare_blp: f64,
    pub ultra_rare_equip: f64,
    pub cognitive_chips: f64,
}

fn main() -> Result<()> {
    let mut rdr = csv::Reader::from_path(concat!(env!("CARGO_MANIFEST_DIR"), "/dataset.csv"))?;
    let mut v = Vec::new();
    for record_res in rdr.records() {
        let mut resd = ResearchData::default();
        let record_line = record_res?;
        let mut record = record_line.iter();
        resd.research_id = record.next().unwrap().parse()?;
        resd.proj_type = record.next().unwrap().to_string();
        resd.time = record.next().unwrap().parse()?;
        resd.rate = record.next().unwrap().parse()?;
        resd.doubloon = record.next().unwrap().parse()?;
        resd.cube = record.next().unwrap().parse()?;
        resd.super_rare_blp = record.next().unwrap().parse()?;
        resd.ultra_rare_blp = record.next().unwrap().parse()?;
        resd.ultra_rare_equip = record.next().unwrap().parse()?;
        resd.cognitive_chips = record.next().unwrap().parse()?;
        v.push(resd);
    }
    let tera = {
        let mut t = Tera::default();
        t.add_template_file(
            concat!(env!("CARGO_MANIFEST_DIR"), "/template/dataset.tera.rs"),
            Some("dataset"),
        )?;
        t
    };
    #[derive(Debug, Clone, Serialize, Deserialize)]
    struct Data {
        dataset: Vec<ResearchData>,
    }
    let data = Data { dataset: v };
    let rendered = tera.render("dataset", &Context::from_serialize(data)?)?;
    fs::write(env::var("OUT_DIR")? + "/dataset.tera.rs", rendered)?;
    Ok(())
}
