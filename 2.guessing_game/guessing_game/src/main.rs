//std is a crate a library and io is a module wich means a source file that has function already created for us and ready to be used
use std::io;
use rand::prelude::*;

fn main() {
    println!("guess the number!");

    println!("please input your guess");

    //variables in rust are immutable meaning that once declared we can t change them so we use mut keyword to make them mutable, guess a variable assinged to a new instance
    //of a string String::new , with new is an associated function associated to a datatype
    let mut guess = String::new();
    let mut rng = thread_rng();
    let random_number: i32 = rng.gen_range(0..100);

    println!("{}",random_number);
    //we can seperate a line into multiple lines in rust
    //stdin is a function provided in io module that will allow us to handle user input properly
    //if we hadn t do use std::io; at the beginig the syntaxe should look like this std::io::stdin().read_line()
    io::stdin()
        //read_line returns Result wich is an enum wich is a type that has some variants in our case Result is used to handle expectation or reading input errors, so Results
        //has 2 variants , Ok and Err , Ok means operation succeded and err means weencountred an error , and the type of the error or what caused the error etc...
        .read_line(&mut guess)//readline is a method that takes userinput to stdin by taking the reference to the string & and specifying that the variable is mutable
        .expect("failed to read user input");// exept is also from Result if the error is in , except will crash the progam and prints the error msg but if ok, the exept
        //will return the number of bytes you have on the string , if you don t call exept your code will compile but we ll get a warning

        println!("Your guess {guess}");

        /*
        let x = 5;
        let y = 10;

        println!("x = {x} and y + 2 = {}", y + 2);
         */

}
