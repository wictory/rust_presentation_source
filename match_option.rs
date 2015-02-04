use std::num::Float;
fn sqrt_option(x: f64) -> Option<f64> {
    if x < 0.0 { None } else { Some(x.sqrt()) }
}
fn main() {
    println!("f = {}",
             match sqrt_option(2.0) {
                 Some(x) => format!("{}", x),
                 None    => format!("not real")
             });
}
