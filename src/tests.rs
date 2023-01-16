use std::rc::Rc;

use crate::{dataset, strict_check_larger, solutions::pso::pso::PsoHandler, value::{resource_value::ResourceValue, restriction::ResourceRestriction}, node::{meta::{PerformanceMode, MetaData}}};


#[test]
fn main_test() {
    let research_data: dataset::ResearchDataCollection = dataset::ResearchDataCollection::load();
    let data_ptr = Rc::new(research_data);
    let resv = ResourceValue::predef(2, 3, 10);
    let rest = ResourceRestriction::empty();
    let mut pso = PsoHandler::new(
        1000,
        50,
        resv,
        rest,
        2.0,
        2.0,
        0.9,
        0.4,
        Rc::clone(&data_ptr),
        0.2,
        PerformanceMode::PureIncome,
    );
    pso.generate();
    println!("{}\n{:?}", pso.global_best.best_performance(), pso.global_best.data.position);
}

#[test]
fn testv() {
    // let v = [15, 7, 11, 3, 0, 4, 5, 20, 19, 22, 14, 21, 8, 12, 23, 16, 25, 26, 2, 9, 27, 6, 1, 17, 10, 13, 28, 18, 24];
    // let v = [15, 7, 0, 4, 12, 16, 8, 1, 17, 13, 5, 3, 20, 6, 18, 11, 21, 22, 19, 26, 14, 27, 28, 2, 9, 23, 10, 25, 24];
    let v = [7, 15, 0, 4, 12, 16, 8, 1, 17, 13, 5, 11, 20, 6, 18, 19, 26, 2, 21, 3, 14, 22, 27, 28, 23, 9, 10, 24, 25];
    use dataset::PROJECT_NAME;
    for i in v {
        println!("{}", PROJECT_NAME[i]);
    }
}

#[test]
fn macro_test() {
    use crate::enum_val::Strict;
    let mut m = 0.0;
    let s = Strict::Loose(125.0);
    let mut otl = false;
    m += strict_check_larger!(1.0 * 2.0, s, 551.0, otl);
    println!("{}, {}", m, otl);
}

#[test]
fn realrate() {
    let dat = [15, 7, 11, 3, 0, 4, 5, 20, 19, 22, 23, 2, 6, 26, 12, 24, 27, 10, 16, 8, 21, 9, 17, 28, 18, 1, 13, 14, 25];
    let d = dataset::ResearchDataCollection::load();
    let ptr = Rc::new(d);
    let resv = ResourceValue::predef(2, 3, 10);
    let rest = ResourceRestriction::empty();
    let mut rm = MetaData::new(dat, Rc::clone(&ptr), &resv, &rest, PerformanceMode::PureIncome);
    rm.generate().cost(&resv, &rest);
    println!("{:?}", rm.rate.refresh_failure);
    let dat2 = [15, 7, 11, 3, 0, 4, 5, 20, 19, 22, 23, 6, 28, 16, 25, 17, 21, 1, 24, 12, 18, 10, 2, 14, 13, 26, 27, 8, 9];
    let mut rm2 = MetaData::new(dat2, Rc::clone(&ptr), &resv, &rest, PerformanceMode::PureIncome);
    rm2.generate().cost(&resv, &rest);
    println!("{:?}", rm2.rate.refresh_failure);
}
