mod calc;
mod dataset;
mod restriction;
mod actual;

pub use dataset::Data;
pub use actual::{ActualRatio, ActualResearches};
pub use calc::{ResultPerDay, RefreshProjects, ResultPlan};
pub use dataset::ReferenceValue;
pub use restriction::Restriction;


pub fn predef_restriction() -> Restriction {
    Restriction::predef()
}


pub fn predef_reference_value() -> ReferenceValue {
    ReferenceValue::predef()
}


pub fn build_restriction(doubloon_ratio: f64, cube_ratio: f64, cong_chips_ratio: f64, time_ratio: f64, ultra_blp_ratio: f64, ultra_equip_ratio: f64, fni_5_super_r: i8, fni_5_ultra_r: i8, fni_f: i8, do_data_collection: bool, do_research_assignment: bool) -> Restriction {
    Restriction::from(doubloon_ratio, cube_ratio, cong_chips_ratio, time_ratio, ultra_blp_ratio, ultra_equip_ratio, fni_5_super_r, fni_5_ultra_r, fni_f, do_data_collection, do_research_assignment)
}


pub fn build_reference_value(doubloon: f64, cube: f64, time_of_an_hour: f64, super_rare: f64, ultra_rare: f64, ultra_equip: f64, cong_chips: f64, time_ratio: f64) -> ReferenceValue {
    ReferenceValue::from(doubloon, cube, time_of_an_hour, super_rare, ultra_rare, ultra_equip, cong_chips, time_ratio)
}


pub fn calc(rest: Restriction, raw_ref: ReferenceValue, refer_v: f64, limit: i8) -> ResultPlan {
    let mut rest = rest.clone();
    let mut raw_ref = raw_ref.clone();
    let resp = build(&mut rest, &mut raw_ref, &refer_v, limit);
    resp

}

pub fn calc_auto(rest: Restriction, raw_ref: ReferenceValue, refer_v: f64, limit: i8) -> ResultPlan {
    let mut rest = rest.clone();
    let mut raw_ref = raw_ref.clone();
    let mut resp: ResultPlan;
    let mut f2 = 0.0;
    let mut f1 = 0.0;
    let mut fthis: f64;
    let mut times: i16 = 0;
    loop {
        resp = build(&mut rest, &mut raw_ref, &refer_v, limit);
        fthis = resp.result.cost_performance;
        if fthis - f1 <= 0.00001 {break}
        else if fthis - f2 <= 0.00001 {
            if fthis >= f1 {break}
            resp = build(&mut rest, &mut raw_ref, &refer_v, limit);
            break;
        }
        f2 = f1;
        f1 = fthis;
        times += 1;
        if times >= 100 {break};
    }
    resp
}

fn build(rest: &mut Restriction, raw_ref: &mut ReferenceValue, refer_v: &f64, limit: i8) -> ResultPlan {
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
    // return serde_wasm_bindgen::to_value(&(resp)).unwrap();
    return resp;
}


#[test]
fn test() {
    let rest = predef_restriction();
    let rfv = predef_reference_value();
    let r = calc(rest, rfv, 156.515819396912, 10);
    // assert_eq!(1 + 1, 2);
    assert!(r.result.cost_performance - 156.515819396912 <= 0.00001);
    let r = calc_auto(rest, rfv, 100.0, 10);
    // assert_eq!(1 + 1, 2);
    assert!(r.result.cost_performance - 156.515819396912 <= 0.00001);
}
