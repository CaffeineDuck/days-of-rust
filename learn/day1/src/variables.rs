use std::io;

fn main() {
    // Creating a string literal which is immutable
    let a = "Hello World";
    
    // This is how you can print a variable to the console
    println!("{}", a);

    // Creating a `String` type variable from the text "Hello World"
    // It can be printed the same way as a string literal to the console
    let b = String::from("Hello World");

    // Creating an empty mutable `String` type variable.
    // Mutable means it can be mutated or changed
    let mut c = String::new();

    // Taking user input
    io::stdin()
        // Reading the user input and mutating it to our empty string
        .read_line(&mut c)
        // This error is thrown if there was any problem while reading user's input
        .expect("There was an error while reading your response");

    println!("User told this: {}", c);

}