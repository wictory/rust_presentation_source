use std::num::Float;
fn ln_option(x: f64) -> Option<f64> {
    if x <= 0.0 { None } else { Some(x.ln()) }
}
fn sqrt_option(x: f64) -> Option<f64> {
    if x < 0.0 { None } else { Some(x.sqrt()) }
}
fn main() {
    let a: Option<f64> = Some(-2.0);
    let b = a.map(|x| { x + 2.0 })
        .and_then(ln_option)
        .and_then(sqrt_option);
    println!("Result = {:?}", b);
}
