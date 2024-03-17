fn main() {
    //we got two data type subsets: scalar and compound
    
    /*rust is a statically type programming language but the compiler
    but the compiler knows the datatype based on the value provided
    but in some case it it important to specify the datatype of a variable
    such as when we converted a string the an unsigned variable
    
    let guess:u32 = "42".parse().expect("Not a number");
    */

    /*
    scalar type represents a single value
    rust has four promary scalar types: integers, floating-point, numbers, boolean, character
    
    INTEGER TYPES:

    i8, u8 , length = 8-bit
    i16, u16, length = 16-bit
    i32, u32, length = 32-bit
    i64, u64, length = 64-bit
    i128, u128, length = 128-bit
    isize, usize, length arch

    signed integer range = -(2^(n - 1)) to 2^(n - 1) - 1 if i8 -(2^(7)) to 2^(7) - 1
    usigned integer range = 0 to 2^n - 1 if u8 0 to 2^8 - 1
    */

    //floatings: f32, f64 by default all floating values are of type f64
    let x = 2.0// f64 has double precision
    let y: f32 = 3.0 // f32 single precision

    //maths operations in rust
    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1

    // remainder
    let remainder = 43 % 5;
}
