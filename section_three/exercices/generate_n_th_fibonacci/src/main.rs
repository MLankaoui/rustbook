use std::io;

fn main() {
    display_message();
    let n = input_number();
    let result = fibonacci(n);

    println!("{}", result);

}
fn display_message() {
    println!("Welcome to nth fibonacci number generator!");
}
fn input_number() -> i32 {
    let mut number_input = String::new();

    println!("Please enter a number");

    io::stdin().read_line(&mut number_input).expect("Failed to read users input");

    let number_input: i32 = number_input.trim().parse().expect("Please enter a valide number");

    fibonacci(number_input);

    number_input
}

fn fibonacci(n: i32) -> i32 {
    if n == 0 || n == 1 {
        return n;
    }
    else {
        return fibonacci(n - 1) + fibonacci(n - 2);
    }
    
}