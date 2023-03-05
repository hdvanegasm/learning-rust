fn main() {

    // Mutable types
    let mut x = 5;
    println!("The value of x is {x}");
    x = 6;
    println!("The value of x is {x}");

    // Constant types
    const HOUR_IN_SECONDS: u32 = 60;
    println!("One hour has {HOUR_IN_SECONDS} seconds");
    let two_ours = 2 * HOUR_IN_SECONDS;
    println!("Two hours has {two_ours} seconds");

    // Shadowing
    let y = 5;
    let y = y + 1;

    {
        let y = y * 6;
        println!("The value of y is {y} in inner scope");
    }

    println!("The value of y is {y}");

    // Overflow
    let mut bin_num: u16 = 255;
    bin_num = bin_num + 1;
    println!("The binary is {bin_num}");

    // Floating point
    let x = 2.54;
    println!("Floating point: {x}");

    // Division operator
    let x = 2;
    let y = 3;
    let div = x / y;
    println!("Division: {div}");

    // Boolean types
    let x: bool = true;
    println!("The boolean value is {x}");

    // Tuple types
    let tup: (i32, f64, u8) = (3, 4.21, 2);
    let tup_0 = tup.0;
    println!("The tuple in y has a value of {tup_0}");

    let a = 32;
    println!("Print {a}");
}
