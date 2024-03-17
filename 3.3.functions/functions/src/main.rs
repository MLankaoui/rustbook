fn main() {
    println!("Hello, world!");
    another_function(5);
}
//it doesn t matter where to put this function as long as their defined somewhere in a scope that can be seen by the caller
fn another_function(x: u32) {
    println!("Another function.");
    println!("The value of x is: {x}");
}
