use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess a number!");

    // Generate a random number between 1 and 100.
    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("\nPlease input your guess:");

        let mut guess = String::new();  
        
        io::stdin()   
            .read_line(&mut guess)
            .expect("Failed to read line");

        // Transforms String guess into a u32 integer.
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
                            
        println!("You guessed: {guess}");

        // Given that guess is a u32 integer, now we
        // can compare it to another integer using cmp().
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("==> You win!");
                break;
            }
        };
    }
}
