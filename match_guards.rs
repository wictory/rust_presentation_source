fn main() {
    let t = (1, 2);
    match t {
        (x, y) if x == y => println!("equal"),
        _                => println!("not equal")
    }




}
