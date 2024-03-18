use std::io;

fn main() {
    println!("welcome to celisus to fahrheit converter");
    display_options();
    let mut user_choice = String::new();

    println!("Plese enter the choice");

    io::stdin().read_line(&mut user_choice).expect("Failed to read users input");

    let user_choice: i32 = user_choice.trim().parse().expect("Please enter a valide number");    

    let value_choice = user_choicee(user_choice);

    let mut convert_value = String::new();

    println!("Plese enter the value to convert");

    io::stdin().read_line(&mut convert_value).expect("Failed to read users input");

    let convert_value: i32 = convert_value.trim().parse().expect("Please enter a valide number");

    evaluate_user_choice(value_choice, convert_value);
}

fn celisius_to_fahrenheit(user_input: i32) -> i32{
    let result = (user_input * (9 / 5)) + 32;
    result
}

fn fahrenheit_to_celisius(user_input:i32) -> i32 {
    let result = (user_input - 32) * (5 / 9);
    result
}

fn display_options() {
    println!("1. C to F");
    println!("2. F to C");
}

fn user_choicee(user_choice: i32) -> i32 {
    if user_choice == 1 {
        1
    } else if user_choice == 2 {
        2
    } else {
        0
    }
}

fn evaluate_user_choice(value: i32, convert_value: i32) {
    if value == 1 {
        let result = celisius_to_fahrenheit(convert_value);
        println!("The result is: {}", result);
    } else if value == 2 {
        let result = fahrenheit_to_celisius(convert_value);
        println!("The result is: {}", result);
    } else {
        println!("invalid choice");
    }
}