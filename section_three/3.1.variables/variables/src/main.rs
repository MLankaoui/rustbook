fn main() {
    //constants are alway immutable , you can t make them mutable
    //constants can be declared at any scope including global scope
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    //variables are immutable by default, you can t change the value after assignement
    /*let y = 5;
    println!("The value of y is: {y}");
    y = 6;
    println!("The value of y is: {y}"); this is gonna give us an error message */

    //example of a constant
    const THRE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    //shadowing , so the second let overshadows the first one
    let x = 5;

    let x = x + 1;
    // we can t perform shadowing on a mutable variable

    {
        let x = x * 2;// 6 * 2
        println!("The value of x in the inner scope is: {x}");//printing 12
    }

    println!("The value of x is: {x}");//printing 6

    //another example of shadowing
    let spaces = "  ";//string
    let spaces = spaces.len();//number
}