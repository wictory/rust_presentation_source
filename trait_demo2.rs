trait ToString {
    fn my_to_string(&self) -> String {
        String::from_str("")
    }
}

impl ToString for (i64, i64) {
    fn my_to_string(&self) -> String {
        format!("The first number is {} and the second number is {}", self.0, self.1)
    }
}
fn main() {
    let b = (5, 4);
    println!("b = {:?}", b.my_to_string());
}
