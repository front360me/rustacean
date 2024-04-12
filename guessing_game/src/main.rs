// The Rng trait must be in scope
use rand::Rng;

// Ordering type is enum (Less, Greater, Equal)
use std::cmp::Ordering;

// std - standard library
// io - input/output library (module)
use std::io;

// main function - the entry point into the program
fn main() {
    // println! - macro that prints a string to the screen
    println!("Guess the number!");

    // rand::thread_rng function returns random number generator        
    // gen_range - method on the random number generator
    let secret_number = rand::thread_rng().gen_range(1..=100); // i32 by default

    loop {
        println!("Please input your guess.");

        // the '::' syntax indicates that 'new' is an associated function of the String type
        let mut guess = String::new();
    
        // we call the stdin function from the io module
        // stdin - function that returns an instance of std::io::Stdin
        // a handle to the standard input for the terminal
        io::stdin()
            // & means that this argument is a reference
            // read_line returns Result value (an enumeration): Ok and Err
            // enum - type that can be in one of multiple possible states (variants)
            .read_line(&mut guess)
            // handle potential failure
            // instance of Result has expect method
            .expect("Failed to read line");
    
        // shadow the previous value of guess
        let guess: u32 = match guess.trim().parse() {
            // parse() returns a Result type
            Ok(num) => num,
            // _ is a catchall value
            Err(_) => continue,
        };
        
        println!("You guessed: {guess}");
    
        // match expression to decide what to do after comparison
        // cmp method compares two values and can be called on anything that can be compared
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
