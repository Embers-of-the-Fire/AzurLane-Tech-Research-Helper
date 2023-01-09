use std::rc::Rc;

use crate::{dataset, strict_check_larger, solutions::pso::pso::PsoHandler, value::{resource_value::ResourceValue, restriction::ResourceRestriction}, node::{meta::{PerformanceMode, MetaData}, rate::RateMeta, product::ProductPerDay}};


#[test]
fn test() {
    let mut idx = 0;
    loop {
        if idx > 4 {
            break;
        }
        println!("{}", idx);
        idx += 1;
    }
}

#[test]
fn test2() {
    let d: [u8; 3] = [0, 1, 5];
    let k = {
        let mut binary_data: [bool; 15] = [false; 15];
        for i in 0..d.len() as u8 {
            let mut h = d[i as usize].clone();
            let b = [
                {
                    if h >= 16 {
                        h -= 16;
                        true
                    } else {
                        false
                    }
                },
                {
                    if h >= 8 {
                        h -= 8;
                        true
                    } else {
                        false
                    }
                },
                {
                    if h >= 4 {
                        h -= 4;
                        true
                    } else {
                        false
                    }
                },
                {
                    if h >= 2 {
                        h -= 2;
                        true
                    } else {
                        false
                    }
                },
                {
                    if h == 1 {
                        true
                    } else {
                        false
                    }
                },
            ];
            let mut idx = 0u8;
            for d in b {
                binary_data[(i * 5 + idx) as usize] = d;
                idx += 1;
            }
        }
        binary_data
    };
    println!("{:?}", k);
    let k2 = {
        let mut res = [0u8; 3];
        for i in 0..(k.len() / 5) as u8 {
            res[i as usize] = k[(i * 5) as usize] as u8 * 16
                + k[(i * 5 + 1) as usize] as u8 * 8
                + k[(i * 5 + 2) as usize] as u8 * 4
                + k[(i * 5 + 3) as usize] as u8 * 2
                + k[(i * 5 + 4) as usize] as u8 * 1;
        }
        res
    };
    println!("{:?}", k2);
    assert_eq!(d, k2);
}

#[test]
fn tt() {
    println!("{}\n{}", 5.3_f64, 5.3_f64 as u8)
}

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
fn testets() {
    println!("{}", 28.999 as u8)
}

#[test]
fn testestset() {
    fn vaildary(pos: [u8; 3]) -> [u8; 3] {
        let mut res = [0u8; 3];
        let mut k: [(u8, u8); 3] = [(0, 0); 3];
        for i in 0..3 {
            k[i] = (i as u8, pos[i]);
        }
        k.sort_by(|a, b| a.1.cmp(&b.1));
        for i in 0..3 {
            res[i] = k[i].0;
        };
        res
    }
    let k = [1, 2, 0];
    println!("{:?}", vaildary(k));
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
