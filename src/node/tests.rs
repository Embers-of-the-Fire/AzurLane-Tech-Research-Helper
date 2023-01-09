#[test]
fn basic_repo_test() {
    let k = [0, 1, 2, 3, 4, 5];
    const STEP: f64 =  1.0 / 6.0;
    let mut res = [0.0_f64; 6];
    for i in 0..k.len() {
        if k[i] >= 6 { panic!("BasicMeta Processing Error: find element which is not in range [0,6]") }
        res[i] = STEP / 2.0 + STEP * k[i] as f64;
    };
    println!("{:?}", res);
    let mut res2 = [0u8; 6];
    for i in 0..6 {
        if res[i] > 1.0 || res[i] < 0.0 { panic!("BasicMeta Processing Error: find element to re-produce which is not in range [0.0, 1.0]") }
        res2[i] = (res[i] * 6.0) as u8;
    }
    println!("{:?}", res2);
    assert_eq!(k, res2);
}