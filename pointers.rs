
fn main() {
    let mut b = Box::new(2);
    println!("{}", b);
    b = Box::new(3);
    println!("{}", b);
}
