fn main() {
    println!("Hello, world!");
    another_function(5);
    print_labeled_measurements(5, 'h');
}
//it doesn t matter where to put this function as long as their defined somewhere in a scope that can be seen by the caller
fn another_function(x: i32) {
    println!("Another function.");
    println!("The value of x is: {x}");
}
// a function that has two arguments
fn print_labeled_measurements(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}
