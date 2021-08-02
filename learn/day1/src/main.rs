// More info on day1 here: https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html
// This file is about GUESSING GAME as provided in the rust ebook.

// use `something` syntax is similar to `import something` syntax
// in python. It is used to import something from other crates.

// rand crate is used to generate a random number
use rand::Rng;

// This is used for comparing the user guess with the 
// secret generated random number
use std::cmp::Ordering;

// io (Input/Ouput) is a module from standard library 
// used for input and ouput operations
use std::io;

fn main() {
    // Creates a variable named `secret_number` with value as a 
    // random number between 1 to 100 using the `rand` crate.
    let secret_number = rand::thread_rng().gen_range(0..101);

    // The `loop` keyword is used to loop as in iterate over and over
    loop {

        // `Println!` command is used to print to console
        println!("Please input your guess.");

        // Creating a mutatable `String` type variable
        let mut guess = String::new();

        // Reading the user input and mutating the variable
        // `guess` with the user input
        io::stdin()
            // Reading user input
            .read_line(&mut guess)
            // If there is a problem this is thrown as the error
            .expect("Failed to read line");

        // String is parsed into a number
        let guess: u32 = match guess.trim().parse() {
            // Returns the number if it succeeded
            Ok(num) => num,
            // Prints a message to the screen and moves to 
            // next iteration if there is an error while parsing the 
            // string to a number 
            Err(_) => {
                println!("Please enter a valid number");
                continue;
            },
        };

        // Match statement is used for structural pattern matching.
        // Here Ordering an enum which works together with match statement in
        // a string to handle various sitautaions such as when the user guess 
        // and the number matches. Or, when it is higher, or when it is lower.
        match guess.cmp(&secret_number) {
            // This is executed if the user guess is high
            Ordering::Greater => println!("Tooo high!"),
            // This is exucuted if the user guess is low
            Ordering::Less => println!("Too low!"),
            // This is executed if the user guessed correctly
            Ordering::Equal => {
                // Prints to the console
                println!("You win!");
                // Stops the loop as the game has ended
                break;
            },
        };
    }
}
