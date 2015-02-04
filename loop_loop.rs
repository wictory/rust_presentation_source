fn main() {
    let mut a = 0;
    loop {
        println!("a = {}", a);
        if a > 10 {
            break;
        } 
        a += 1;
    } 
}
