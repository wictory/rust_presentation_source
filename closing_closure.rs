
fn main () {
    let inc = 1;
    let add_inc = |&: x| { inc + x };
    let val = 10;
    println!("{} incremented by {} is {}",
            val, inc, add_inc(val));
}
