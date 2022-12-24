mod utils;
mod calc;
mod dataset;
mod restriction;
mod actual;

pub use dataset::Data;
pub use actual::{ActualRatio, ActualResearches};
pub use calc::{ResultPerDay, RefreshProjects, ResultPlan};
pub use dataset::ReferenceValue;
pub use restriction::Restriction;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn predef_restriction() -> JsValue {
    utils::set_panic_hook();
    serde_wasm_bindgen::to_value(&Restriction::predef()).unwrap()
}

#[wasm_bindgen]
pub fn predef_reference_value() -> JsValue {
    utils::set_panic_hook();
    serde_wasm_bindgen::to_value(&ReferenceValue::predef()).unwrap()
}

#[wasm_bindgen]
pub fn build_restriction(doubloon_ratio: f64, cube_ratio: f64, cong_chips_ratio: f64, time_ratio: f64, ultra_blp_ratio: f64, ultra_equip_ratio: f64, fni_5_super_r: i8, fni_5_ultra_r: i8, fni_f: i8, do_data_collection: bool, do_research_assignment: bool) -> JsValue {
    utils::set_panic_hook();
    serde_wasm_bindgen::to_value(&Restriction::from(doubloon_ratio, cube_ratio, cong_chips_ratio, time_ratio, ultra_blp_ratio, ultra_equip_ratio, fni_5_super_r, fni_5_ultra_r, fni_f, do_data_collection, do_research_assignment)).unwrap()
}

#[wasm_bindgen]
pub fn build_reference_value(doubloon: f64, cube: f64, time_of_an_hour: f64, super_rare: f64, ultra_rare: f64, ultra_equip: f64, cong_chips: f64, time_ratio: f64) -> JsValue {
    utils::set_panic_hook();
    serde_wasm_bindgen::to_value(&ReferenceValue::from(doubloon, cube, time_of_an_hour, super_rare, ultra_rare, ultra_equip, cong_chips, time_ratio)).unwrap()
}

#[wasm_bindgen]
pub fn calc(rest: JsValue, raw_ref: JsValue, refer_v: f64, limit: i8) -> JsValue {
    utils::set_panic_hook();
    let mut rest = serde_wasm_bindgen::from_value::<Restriction>(rest).unwrap();
    let mut raw_ref = serde_wasm_bindgen::from_value::<ReferenceValue>(raw_ref).unwrap();
    // let mut rest = Restriction::predef();
    // let mut raw_ref = ReferenceValue::predef();
    build(&mut rest, &mut raw_ref, &refer_v, limit)
}

fn build(rest: &mut Restriction, raw_ref: &mut ReferenceValue, refer_v: &f64, limit: i8) -> JsValue {
    utils::set_panic_hook();
    // 现在数据终于都他妈对了
    let mut db = Data::load();
    raw_ref.actual(&rest);
    let r = ActualRatio::from(&rest);
    db.refresh_with_actual_ratio(&r);
    let mut rr = ActualResearches::from(&db);
    rr.generate_single_income(&raw_ref, &r);
    rr.generate_cost(&raw_ref);
    rr.generate_refer(refer_v);
    rr.generate_tap(&rest);
    rr.sort();
    let mut rp = RefreshProjects::from(&rr, &r, limit);
    rp.generate();
    rp.generate_refresh_perf();
    let res = rp.get_result();
    let fnres = ResultPerDay::from(&res);
    let resp = ResultPlan::build(res, fnres, rp);
    return serde_wasm_bindgen::to_value(&(resp)).unwrap();
}
