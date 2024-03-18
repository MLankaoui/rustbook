fn main() {
    // if expressions in rust

    let number = 7;
    // this is an expression also called arm
    // and it should be a bool
    if number < 5 {
        println!("condition was true");
    }
    else {
        println!("condition was false");
    }

    //handling multiple conditions with else if
    let number_two = 6;
    //although 6 is divisible by 3 and 2 rust cares just for the first case and when it finds it it doesn't matter care about the rest of the code
    //using else if multiple time is not recommended that s why we should learn about match concept
    if number_two % 4 == 0 {
        println!("number_two is divisibile by 4");
    } else if number_two % 3 == 0 {
        println!("number_two is divisibile by 3");
    } else if number_two % 2 == 0 {
        println!("number_two is divisibile by 2");
    } else {
        println!("number_two is not divisibile by 4, 3 or 2");
    }


    //using if in a let statement

    let condition = false;
    // the values types should be the same
    let number_three = if condition {5} else {6};

    println!("The value of number_three is: {number_three}");

    // repetitions with loops
    //we can stop the loop using break
    // this is an infinite loop
    /*loop {
        println!("again!");
    }*/

    //returning values from loops
    let mut counter = 0;
    //result variable holds the value returned by the loop
    //the loop will continue until counter == 10 than breaks and multiply counter * 2
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is: {result}");

    //loop labels
    let mut countt = 0;
    //counting_up is a label ofr the outer loop
    //here is an nested loops example
    'counting_up: loop {
        println!("count = {countt}");
        let mut remaining = 10;

        loop {
            println!("remainig = {remaining}");
            if remaining == 9 {
                break;
            }
            if countt == 2 {
                break 'counting_up;
            }

            remaining -= 1;
        }

        countt += 1;
    }

    println!("End count: {countt}");
}
