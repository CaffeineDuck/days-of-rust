// Learn more about functions here: https://doc.rust-lang.org/book/ch03-03-how-functions-work.html

// Entry point for the rust program
fn main() {
    // This is how we call a function
    // Doing this runs the function
    print_10();

    // Declaring a variable a with the value 20
    let a = 20;

    // Passing the value of a into the function
    // As we are returning the value from the function
    // We can take returned values from the function in this way
    let multiplied = multiply_by_10(a);

    // Printing the multiplied value
    println!("20*10 = {}", multiplied);
}

// The position of the function doesn't matter,
// the function just needs to be defined inside 
// the program.

// This function prints "10"
fn print_10() {
    // Printing 10
    println!("10");
}

// This function takes a number of `isize` (https://doc.rust-lang.org/book/ch03-02-data-types.html)
// and returns that number multiplied by 10.

// Here the colon after number `:` is used to describe the type 
// of the variable number, and the arrow after the parenthesis
// is used to determine the return type of the function
fn multiply_by_10(number: isize) -> isize {
    // We can return the value from a function like this
    number * 10
}