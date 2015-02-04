
fn main() {
    let n = 4; // Again created a simulated 
               // non-deterministic value
    println!("n is {}", match n {
        1 => "one",
        2 => "two",
        3 => "three",
        4 => "four",
        _ => "way too big"
    });
}
