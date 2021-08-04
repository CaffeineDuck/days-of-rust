/// More info on day 2: https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html

fn main() { 
    // Creating a `String` type variable which is stored in a heap.
    let ownership_string = give_ownership();


    // Passing `ownership_string` var by reference and getting back 
    // its length
    let len = len_by_reference(&ownership_string);

    // Transferring the ownership from `ownership_string` to `new_string`.
    // Doing this makes `ownership_string` unusabe as the ownership is moved.
    // Try `rustc --explain E0382` to get more info on this error
    let new_string = ownership_string;
    
    // Passing the sliced `ownership_string` var as a string literal 
    // cause sliced value is always a string literal and rules of 
    // ownership doesn't apply to sliced data type (string literal [str])
    let sliced_string = slicing(&new_string[..]);


    // Prints the values to the console
    println!("String: {} is of length {} and its sliced form is {}.", new_string, len, sliced_string);
}

/// A new variable `some_string` has been created of type `String` which
/// is returned hence the ownership is transferred so now the variable 
/// `some_string` is unusable in the current scope as the ownership is
/// already transferred.
fn give_ownership() -> String {
    // Creates a string named `some_string` in a heap.
    let some_string = String::from("hello");

    // Returns the `some_string` var and transfers its ownership
    some_string          
}

/// Here the reference of `a` is passed rather than passing the actual 
/// `a` hence the ownership is not transferred. This is why the variable
/// `a` can still be used the `fn main()` scope.
fn len_by_reference(text: &String) -> usize {
    // String.len() returns the length of the string.
    text.len()
}

/// Refrence of sliced `String <&a>` has been passed here which is a
/// string Literal `&str` hence it can again be sliced and its reference
/// can be passed. But as this is a string literal, there is no worries 
/// about the value being changed of a reference.
fn slicing(text: &str) -> &str {
    // Slicing using [1..2] is equivalent of [1:2] in python. 
    // Its inclusive of the lower index and exclusive of the upper index.
    // as in: It includes the `1` and exlucdes `2` in [1..2]
    &text[1..2]
}
