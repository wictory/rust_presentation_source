
fn main() {
    // Create an iterator counting from 0 to 10.
    let a = range(0, 10);
    // Create an iterator which contains the double of each
    //  element in a.
    let the_double = a.map(|x| { x * 2 });
    // Zip together the_double with an infinite sequence
    // of increasing numbers.
    let mut b = the_double.zip(std::iter::count(1,1)); 
    // Sum together elements in b using a fold
    let s = range(0,10).fold(0, |ack, v| { ack + v });
    for (c, i) in b {
        // Print out the tuples in b
        println!("{} = {}", i, c); 
    }
    // Print out the sum.
    println!("sum = {}", s); 
}
