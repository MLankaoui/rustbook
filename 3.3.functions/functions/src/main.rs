fn main() {
    println!("Hello, world!");
    another_function(5);
    print_labeled_measurements(5, 'h');
    // {} is na expression
    let y = {
        let x = 5;
        x + 1
    };
    
    println!("The value of y is: {y}");
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

//rust is an expression-based language
//statements are instructions that perform some action and do not return a value
//Expressions evaluate the result value

//let y = 6 is a statement

//function definitions are also statements

//you can t assign a let statement to another variable, you ll get an error
//let x = (let y = 6)


//functions with return values
