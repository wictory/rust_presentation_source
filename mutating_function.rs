fn fun(x: &mut i64) {
    *x += 1;
}

fn main() {
    let mut a: i64 = 2;
    fun(&mut a);
    println!("a = {}", a);
}
