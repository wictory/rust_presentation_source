trait ToString {
    fn my_to_string(&self) -> String {
        String::from_str("")
    }
}
struct Singleton;
impl ToString for Singleton {
    fn my_to_string(&self) -> String {
        String::from_str("Nothing, more or less")
    }
}
fn main() {
    let a = Singleton;
    println!("a = {:?}", a.my_to_string());
}
