#![feature(box_syntax)]
fn main() {
    // malloc()
    let a = box (1, 2, 3);
    println!("*a = {:?}", *a);
    // free()
}
