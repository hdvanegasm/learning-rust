fn main() {
    // Conditional
    let number = 129;
    
    // Manny 'else if' conditions produces cluttering
    if number % 2 == 0 {
        println!("Divisible by 2");
    } else if number % 3 == 0 {
        println!("Divisible by 3");
    } else if number % 5 == 0 {
        println!("Divisible by 5");
    } else {
        println!("Not divisible by 2, 3, or 5");
    }
}
