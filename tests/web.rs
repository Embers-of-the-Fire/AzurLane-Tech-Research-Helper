//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use std::assert_eq;

use wasm_bindgen_test::*;
use tech_research;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn pass() {
    let rest = tech_research::predef_restriction();
    let rfv = tech_research::predef_reference_value();
    let r = tech_research::calc(rest, rfv, 156.515819396912, 10);
    let cv: tech_research::ResultPerDay = serde_wasm_bindgen::from_value(r).unwrap();
    // assert_eq!(1 + 1, 2);
    assert!(cv.cost_performance - 156.515819396912 <= 0.00001);
}
