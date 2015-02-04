#[derive(Show)]
struct Tuple { e1: i64,
               e2: i64,
               pub e3: i64 }
fn main() {
    let u = Tuple { e1: 1, e2: 2, e3: 3 };
    println!("u = {:?}", u);
    println!("u.e1 = {}", u.e1);
}

