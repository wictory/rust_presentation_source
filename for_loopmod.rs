fn main() {
    for a in 0..10 { // Immutable variable!!
        println!("a = {}", a);
        // 100 lines of code ...
        if true { // Whoops! Someone put in a case
                  // which should never anyway happen.
            a = 1; // Compile error
        }
        // 100 more lines of code ...
    }
}
